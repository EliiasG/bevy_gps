use bevy::prelude::*;

use crate::floppy::FloppyBody;

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_satellite_circle);
    }
}

#[derive(Component)]
pub struct Satellite;

fn draw_satellite_circle(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<FloppyBody>>,
    satellite_query: Query<&Transform, (With<Satellite>, Without<FloppyBody>)>,
) {
    for player in player_query.iter() {
        for satellite in satellite_query.iter() {
            gizmos.circle_2d(
                satellite.translation.xy(),
                satellite.translation.distance(player.translation),
                Color::BLUE,
            );
        }
    }
}
