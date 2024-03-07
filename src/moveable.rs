use bevy::{math::vec2, prelude::*, window::PrimaryWindow};

use crate::camera::CameraController;

pub struct MoveablePlugin;

impl Plugin for MoveablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurMoving {
            entity: None,
            offset: Vec2::ZERO,
        });
        app.add_systems(Update, update_moveables);
    }
}

#[derive(Component)]
pub struct Moveable {
    /// radius for selection - in pixels
    pub radius: f32,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Deletable;

#[derive(Resource)]
struct CurMoving {
    entity: Option<Entity>,
    offset: Vec2,
}

fn update_moveables(
    time: Res<Time>,
    mut commands: Commands,
    mut cur_moving: ResMut<CurMoving>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    camera: Query<&Transform, With<CameraController>>,
    mut moveable_query: Query<
        (
            &mut Transform,
            &mut Moveable,
            Entity,
            Has<Deletable>,
            &InheritedVisibility,
        ),
        Without<CameraController>,
    >,
) {
    // Amazing example of good code and SRP...
    let mut window = window.single_mut();
    let camera_transform = camera.single();
    let mouse_pos = (match window.cursor_position() {
        Some(v) => v,
        None => return,
    } - vec2(window.width(), window.height()) / 2.)
        * camera_transform.scale.xy()
        * vec2(1., -1.)
        + camera_transform.translation.xy();
    window.cursor.icon = CursorIcon::Default;

    if let Some(moving) = cur_moving.entity {
        window.cursor.icon = CursorIcon::Pointer;
        let (mut moveable_transform, mut moveable, _, _, _) = match moveable_query.get_mut(moving) {
            Ok(v) => v,
            Err(_) => return,
        };
        if !mouse.pressed(MouseButton::Left) {
            cur_moving.entity = None;
            moveable.velocity = Vec2::ZERO;
            return;
        }
        let new_pos = mouse_pos + cur_moving.offset * camera_transform.scale.xy();
        moveable.velocity = (new_pos - moveable_transform.translation.xy()) / time.delta_seconds();
        moveable_transform.translation = new_pos.extend(moveable_transform.translation.z);
        return;
    }

    for (transform, moveable, entity, has_deleteable, vis) in moveable_query.iter_mut() {
        let offset = (transform.translation.xy() - mouse_pos) / camera_transform.scale.xy();
        if !vis.get() {
            continue;
        }
        if offset.length_squared() < moveable.radius.powi(2) {
            window.cursor.icon = CursorIcon::Pointer;
            if mouse.just_pressed(MouseButton::Left) {
                cur_moving.offset = offset;
                cur_moving.entity = Some(entity);
                return;
            } else if mouse.just_pressed(MouseButton::Right) && has_deleteable {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
