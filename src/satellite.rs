use bevy::{math::vec2, prelude::*};

use crate::{floppy::FloppyBody, GraphVisibility, Settings};

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
    settings: Res<Settings>,
    player_query: Query<&Transform, With<FloppyBody>>,
    satellite_query: Query<&Transform, (With<Satellite>, Without<FloppyBody>)>,
) {
    for player in player_query.iter() {
        let player = player.translation.xy();
        let mut last = None;
        for satellite in satellite_query.iter() {
            let p = satellite.translation.xy();
            let dst = p.distance(player);
            if settings.ranges_visible {
                gizmos
                    .circle_2d(p, dst + settings.ranges_offset, Color::BLUE)
                    .segments(256);
            }
            if settings.graph_visibility == GraphVisibility::Some {
                if let Some((lst, lstr)) = last {
                    darw_arcs(&mut gizmos, p, dst, lst, lstr)
                } else {
                    gizmos.circle_2d(p, 200., Color::RED);
                }
            }
            last = Some((p, dst))
        }
        if settings.graph_visibility != GraphVisibility::All {
            continue;
        }
        for [s1, s2] in satellite_query.iter_combinations() {
            let s1 = s1.translation.xy();
            let s2 = s2.translation.xy();
            darw_arcs(
                &mut gizmos,
                s1,
                s1.distance(player),
                s2,
                s2.distance(player),
            );
        }
    }
}

fn darw_arcs(gizmos: &mut Gizmos, p1: Vec2, r1: f32, p2: Vec2, r2: f32) {
    let d = p1.distance(p2);
    let a: f32 = (r1 + r2 - d) * 0.5f32;
    let step = 5.;
    let amt = 100.;
    let n = ((amt / step) as f32).ceil() as usize;
    let mut v1 = Vec::with_capacity(n + 1);
    let mut v2 = Vec::with_capacity(n + 1);
    for i in 0..n {
        let off = -a + (i as f32 + 0.5).powi(3) * step;
        let [p1, p2] = circle_intersect(p1, r1 + off, p2, r2 + off);
        if v1.is_empty() {
            v1.push(p2);
        }
        v1.push(p1);
        v2.push(p2);
    }
    gizmos.linestrip_2d(v1, Color::GREEN);
    gizmos.linestrip_2d(v2, Color::GREEN);
}

// from https://stackoverflow.com/questions/3349125/circle-circle-intersection-points, translated by ChatGPT with manual edits
fn circle_intersect(p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> [Vec2; 2] {
    let d = p1.distance(p2);
    let a = (r1.powi(2) - r2.powi(2) + d.powi(2)) / (2.0 * d);
    let h = (r1.powi(2) - a.powi(2)).sqrt();

    let x2 = p1.x + a * (p2.x - p1.x) / d;
    let y2 = p1.y + a * (p2.y - p1.y) / d;
    let v1 = h * (p2.y - p1.y) / d;
    let v2 = h * (p2.x - p1.x) / d;
    [vec2(x2 + v1, y2 - v2), vec2(x2 - v1, y2 + v2)]
}
