use background::{Ground, GroundPlugin};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::texture::{ImageSampler, ImageSamplerDescriptor},
    sprite::Anchor,
};
use bevy_wasm_window_resize::WindowResizePlugin;
use camera::{CameraController, CameraControllerPlugin};
#[allow(unused_imports)]
use floppy::{FloppyBody, FloppyComponent, FloppyDebugPlugin, FloppyPlugin};
use moveable::{Moveable, MoveablePlugin};
use satellite::SatellitePlugin;
use ui::UiPlugin;

pub mod background;
pub mod camera;
pub mod floppy;
pub mod moveable;
pub mod satellite;
pub mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin {
                ..default() //default_sampler: ImageSamplerDescriptor::nearest(),
            }),
            CameraControllerPlugin,
            GroundPlugin,
            MoveablePlugin,
            FloppyPlugin,
            //FloppyDebugPlugin,
            SatellitePlugin,
            UiPlugin,
            WindowResizePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, update_character))
        .insert_resource(Settings {
            character_visible: true,
            ranges_visible: true,
            graph_visibility: GraphVisibility::All,
            ranges_offset: 0.,
        })
        .run()
}

#[derive(Resource)]
struct BackgroundImage {
    image: Handle<Image>,
    set: bool,
}

#[derive(Eq, PartialEq)]
pub enum GraphVisibility {
    None,
    Some,
    All,
}

#[derive(Resource)]
pub struct Settings {
    pub character_visible: bool,
    pub ranges_visible: bool,
    pub graph_visibility: GraphVisibility,
    pub ranges_offset: f32,
}

#[derive(Component)]
struct Character;

fn update_character(settings: Res<Settings>, mut query: Query<&mut Visibility, With<Character>>) {
    for mut vis in query.iter_mut() {
        *vis = if settings.character_visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), CameraController::default()));
    let img = asset_server.load("ground.png");
    commands.insert_resource(BackgroundImage {
        image: img.clone(),
        set: false,
    });
    spawn_dude(&mut commands, &asset_server);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(100000., 100000.)),
                ..default()
            },
            texture: img,
            transform: Transform::from_xyz(0., 0., -100.),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 128.,
        },
        Ground { size: 256. },
    ));
}

fn spawn_dude(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let collider = asset_server.load("Collider.png");
    let torso = asset_server.load("Torso.png");
    let head = asset_server.load("Head.png");
    let left_leg = asset_server.load("LeftLeg.png");
    let right_leg = asset_server.load("RightLeg.png");
    let left_arm = asset_server.load("LeftArm.png");
    let right_arm = asset_server.load("RightArm.png");
    let mirror = vec![
        (
            vec3(-22.5, 70., 2.),
            vec2(0.1, 0.4),
            left_leg,
            right_leg,
            -30f32.to_radians(),
            5f32.to_radians(),
            -15f32.to_radians(),
        ),
        (
            vec3(-45., 180., 4.),
            vec2(-0.1, 0.45),
            left_arm,
            right_arm,
            -30f32.to_radians(),
            15f32.to_radians(),
            -50f32.to_radians(),
        ),
    ];
    let mut components = vec![
        (vec3(0., 190., 3.), vec2(0., 0.5), torso, 0., 0., 0.),
        (
            vec3(0., 190., 2.5),
            vec2(0., -0.4),
            head,
            45f32.to_radians(),
            0.,
            0.,
        ),
    ];
    components.append(
        &mut mirror
            .iter()
            .map(|(pos, off, l, _, x, y, vel)| (pos.clone(), off.clone(), l.clone(), *x, *y, *vel))
            .collect(),
    );
    components.append(
        &mut mirror
            .iter()
            .map(|(pos, off, _, r, x, y, vel)| {
                (
                    *pos * vec3(-1., 1., 1.),
                    *off * vec2(-1., 1.),
                    r.clone(),
                    *x,
                    -*y,
                    -*vel,
                )
            })
            .collect(),
    );
    commands
        .spawn((
            SpriteBundle {
                texture: collider,
                transform: Transform::from_translation(vec3(0., 0., 0.5)),
                ..default()
            },
            Moveable {
                radius: 30.,
                velocity: vec2(0., 0.),
            },
            Character,
            FloppyBody::default(),
        ))
        .with_children(|builder| {
            for (pos, anchor, img, x_flop, y_flop, magnitude_flop) in components {
                builder.spawn((
                    SpriteBundle {
                        texture: img,
                        transform: Transform::from_translation(pos),
                        sprite: Sprite {
                            anchor: Anchor::Custom(anchor),
                            ..default()
                        },
                        ..default()
                    },
                    FloppyComponent {
                        x_flop,
                        y_flop,
                        magnitude_flop,
                    },
                ));
            }
        });
}

fn update(mut image: ResMut<BackgroundImage>, mut images: ResMut<Assets<Image>>) {
    if image.set {
        return;
    }
    if let Some(img) = images.get_mut(&image.image) {
        img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
        image.set = true;
    }
}
