use bevy::{math::vec3, prelude::*, ui::widget::UiImageSize, utils::RandomState};
use rand::prelude::*;

use crate::{
    moveable::{Deletable, Moveable},
    satellite::Satellite,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_ui)
            .add_systems(Update, button_interaction);
    }
}

fn button_interaction(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Transform, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    let mut rng = rand::thread_rng();
    for (mut transform, interaction) in query.iter_mut() {
        let s = match interaction {
            Interaction::Pressed => 0.75,
            Interaction::Hovered => 0.75,
            Interaction::None => 1.,
        };
        transform.scale = vec3(s, s, s);
        if *interaction == Interaction::Pressed {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("Satellite.png"),
                    transform: Transform::from_translation(vec3(
                        rng.gen::<f32>() * 2000. - 1000.,
                        rng.gen::<f32>() * 2000. - 1000.,
                        1.,
                    )),
                    ..default()
                },
                Moveable {
                    radius: 50.,
                    velocity: Vec2::ZERO,
                },
                Satellite,
                Deletable,
            ));
        }
    }
}

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("Ui.png"),
                    ..default()
                },
                style: Style {
                    align_self: AlignSelf::Start,
                    margin: UiRect::all(Val::Px(8.)),
                    ..default()
                },
                ..default()
            });
            parent
                .spawn(ButtonBundle {
                    background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("AddIcon.png"),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}
