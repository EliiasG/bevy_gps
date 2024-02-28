use bevy::prelude::*;

use crate::camera::CameraController;

pub struct GroundPlugin;
impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct Ground {
    pub size: f32,
}

fn update(
    camera_query: Query<&Transform, With<CameraController>>,
    mut ground_query: Query<(&mut Transform, &Ground), Without<CameraController>>,
) {
    let camera_transform = camera_query.single();
    let (mut ground_transform, ground) = ground_query.single_mut();

    ground_transform.translation =
        (camera_transform.translation / ground.size).floor() * ground.size;
}
