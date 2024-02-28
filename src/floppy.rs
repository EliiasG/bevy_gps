use bevy::prelude::*;

use crate::moveable::Moveable;
pub struct FloppyPlugin;

impl Plugin for FloppyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_floppy_velocity, update_floppy_components).chain(),
        );
    }
}
pub struct FloppyDebugPlugin;

impl Plugin for FloppyDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_floppy_body);
    }
}

#[derive(Component)]
pub struct FloppyBody {
    pub magnitude: f32,
    pub decay: f32,
    pub max_velocity: f32,
    velocity: Vec2,
}

#[derive(Component)]
pub struct FloppyComponent {
    pub x_flop: f32,
    pub y_flop: f32,
    pub magnitude_flop: f32,
}

impl Default for FloppyBody {
    fn default() -> Self {
        Self {
            magnitude: 1.,
            max_velocity: 5000.,
            decay: 10000.,
            velocity: Vec2::ZERO,
        }
    }
}

fn update_floppy_components(
    parent_query: Query<&FloppyBody>,
    mut component_query: Query<(&FloppyComponent, &mut Transform, &Parent)>,
) {
    for (component, mut transform, parent) in component_query.iter_mut() {
        let parent = parent_query
            .get(parent.get())
            .expect("FloppyComponent not child of floppybody");
        let vel = parent.velocity;
        transform.rotation = Quat::from_rotation_z(
            (vel.length() * component.magnitude_flop
                + vel.x * component.x_flop
                + vel.y * component.y_flop)
                / parent.max_velocity,
        );
    }
}

fn update_floppy_velocity(time: Res<Time>, mut floppy_query: Query<(&mut FloppyBody, &Moveable)>) {
    for (mut floppy, moveable) in floppy_query.iter_mut() {
        let vel = moveable.velocity.clamp_length_max(floppy.max_velocity) * floppy.magnitude;
        if vel.length_squared() > floppy.velocity.length_squared() {
            floppy.velocity = vel;
        }
        floppy.velocity = floppy.velocity.clamp_length_max(
            (floppy.velocity.length()
                - floppy.decay
                    * if moveable.velocity.abs_diff_eq(Vec2::ZERO, f32::EPSILON) {
                        3.
                    } else {
                        1.
                    }
                    * time.delta_seconds())
            .max(0.),
        )
    }
}

fn debug_floppy_body(
    floppy_query: Query<(&FloppyBody, &Moveable, &Transform)>,
    mut gizmos: Gizmos,
) {
    for (floppy, moveable, &transform) in floppy_query.iter() {
        gizmos.arrow_2d(
            transform.translation.xy(),
            transform.translation.xy() + moveable.velocity * 0.15,
            Color::RED,
        );
        gizmos.arrow_2d(
            transform.translation.xy(),
            transform.translation.xy() + floppy.velocity * 0.15,
            Color::BLUE,
        );
    }
}
