use bevy::{math::vec3, prelude::*};
use rand::prelude::*;

use crate::{
    moveable::{Deletable, Moveable},
    satellite::Satellite,
    GraphVisibility, Settings,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_ui).add_systems(
            Update,
            (
                state_button_interaction,
                sat_button_interaction,
                range_button_interaction,
                vis_button_interaction,
            ),
        );
    }
}
#[derive(Component)]
struct SatelliteButton;

#[derive(Component)]
enum RangeButton {
    Increase(f32),
    Decrease(f32),
    Reset,
}

#[derive(Component)]
enum VisibilityButton {
    Character,
    Ranges,
    Graphs,
}

fn sat_button_interaction(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Interaction, (Changed<Interaction>, With<SatelliteButton>)>,
) {
    let mut rng = thread_rng();
    for interaction in query.iter() {
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

fn vis_button_interaction(
    mut settings: ResMut<Settings>,
    query: Query<(&VisibilityButton, &StateButton), Changed<Interaction>>,
) {
    for (vis, state) in query.iter() {
        match *vis {
            VisibilityButton::Character => settings.character_visible = state.state == 0,
            VisibilityButton::Ranges => settings.ranges_visible = state.state == 1,
            VisibilityButton::Graphs => {
                settings.graph_visibility = match state.state {
                    1 => GraphVisibility::Some,
                    2 => GraphVisibility::All,
                    _ => GraphVisibility::None,
                }
            }
        }
    }
}

fn range_button_interaction(
    time: Res<Time>,
    mut settings: ResMut<Settings>,
    mut button_query: Query<(&Interaction, &mut RangeButton)>,
) {
    let speed = 250.;
    for (int, mut range) in button_query.iter_mut() {
        if *int != Interaction::Pressed {
            *range = match *range {
                RangeButton::Increase(_) => RangeButton::Increase(0.),
                RangeButton::Decrease(_) => RangeButton::Decrease(0.),
                RangeButton::Reset => RangeButton::Reset,
            };
            continue;
        }
        match *range {
            RangeButton::Increase(b_time) => {
                *range = RangeButton::Increase(b_time + time.delta_seconds());
                settings.ranges_offset += time.delta_seconds() * speed * (b_time + 1.);
            }
            RangeButton::Decrease(b_time) => {
                *range = RangeButton::Decrease(b_time + time.delta_seconds());
                settings.ranges_offset -= time.delta_seconds() * speed * (b_time + 1.);
            }
            RangeButton::Reset => settings.ranges_offset = 0.,
        }
    }
}

fn state_button_interaction(
    mut query: Query<
        (&Interaction, &mut StateButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut child_query: Query<&mut UiImage>,
) {
    for (interaction, mut btn, children) in query.iter_mut() {
        let state = &btn.states[btn.state as usize];
        let s = match interaction {
            Interaction::Pressed => state.1.clone(),
            Interaction::Hovered => state.1.clone(),
            Interaction::None => state.0.clone(),
        };
        let mut ent = None;
        for child in children {
            if let Ok(_) = child_query.get(*child) {
                ent = Some(child);
                break;
            }
        }
        let mut ui_im = child_query.get_mut(*ent.unwrap()).unwrap();
        ui_im.texture = s;
        if *interaction == Interaction::Pressed {
            btn.state = (btn.state + 1) % btn.states.len() as u16;
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
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                border: UiRect::all(Val::Px(4.)),
                row_gap: Val::Px(4.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        column_gap: Val::Px(4.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("Cross.png"),
                            ..default()
                        },
                        ..default()
                    },));
                    parent
                        .spawn((
                            button(vec![(
                                asset_server.load("AddSat.png"),
                                asset_server.load("AddSatSel.png"),
                            )]),
                            SatelliteButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle::default());
                        });
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("Cross.png"),
                            ..default()
                        },
                        ..default()
                    });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        column_gap: Val::Px(4.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("Error.png"),
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn((
                            button(vec![(
                                asset_server.load("Dec.png"),
                                asset_server.load("DecSel.png"),
                            )]),
                            RangeButton::Decrease(0.),
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle::default());
                        });

                    parent
                        .spawn((
                            button(vec![(
                                asset_server.load("Inc.png"),
                                asset_server.load("IncSel.png"),
                            )]),
                            RangeButton::Increase(0.),
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle::default());
                        });

                    parent
                        .spawn((
                            button(vec![(
                                asset_server.load("Reset.png"),
                                asset_server.load("ResetSel.png"),
                            )]),
                            RangeButton::Reset,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle::default());
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        column_gap: Val::Px(4.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("HideText.png"),
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn((
                            button(vec![
                                (
                                    asset_server.load("Shown.png"),
                                    asset_server.load("ShownSel.png"),
                                ),
                                (
                                    asset_server.load("Hidden.png"),
                                    asset_server.load("HiddenSel.png"),
                                ),
                            ]),
                            VisibilityButton::Character,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle::default());
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        column_gap: Val::Px(4.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("ShowRanges.png"),
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn((
                            button(vec![
                                (
                                    asset_server.load("Hidden.png"),
                                    asset_server.load("HiddenSel.png"),
                                ),
                                (
                                    asset_server.load("Shown.png"),
                                    asset_server.load("ShownSel.png"),
                                ),
                            ]),
                            VisibilityButton::Ranges,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle::default());
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        column_gap: Val::Px(4.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("ShowGraph.png"),
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn((
                            button(vec![
                                (
                                    asset_server.load("GraphNone.png"),
                                    asset_server.load("GraphNoneSel.png"),
                                ),
                                (
                                    asset_server.load("GraphSome.png"),
                                    asset_server.load("GraphSomeSel.png"),
                                ),
                                (
                                    asset_server.load("GraphAll.png"),
                                    asset_server.load("GraphAllSel.png"),
                                ),
                            ]),
                            VisibilityButton::Graphs,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle::default());
                        });
                });
        });
}

#[derive(Component)]
pub struct StateButton {
    pub state: u16,
    pub states: Vec<(Handle<Image>, Handle<Image>)>,
}

fn button(states: Vec<(Handle<Image>, Handle<Image>)>) -> impl Bundle {
    (
        ButtonBundle {
            background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
            ..default()
        },
        StateButton { state: 0, states },
    )
}
