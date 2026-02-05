use bevy::prelude::*;
use crate::components::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_ui(mut commands: Commands) {
    // Root UI container - transparent to show game world
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    })
    .with_children(|parent| {
        // === HEADER BAR ===
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                padding: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgba(0.118, 0.161, 0.231, 0.95).into(),
            ..default()
        })
        .with_children(|header| {
            // Title
            header.spawn(TextBundle::from_section(
                "🎣 Rare Fish Game",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.945, 0.961, 0.973),
                    ..default()
                },
            ));

            // Stats container
            header.spawn(NodeBundle {
                style: Style {
                    column_gap: Val::Px(30.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|stats| {
                // Fish count
                stats.spawn(NodeBundle {
                    style: Style {
                        column_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|stat| {
                    stat.spawn(TextBundle::from_section(
                        "Fish:",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::srgb(0.796, 0.835, 0.882),
                            ..default()
                        },
                    ));
                    stat.spawn((
                        TextBundle::from_section(
                            "0",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.945, 0.961, 0.973),
                                ..default()
                            },
                        ),
                        FishCountText,
                    ));
                });

                // Gold
                stats.spawn(NodeBundle {
                    style: Style {
                        column_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|stat| {
                    stat.spawn(TextBundle::from_section(
                        "Gold:",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::srgb(0.796, 0.835, 0.882),
                            ..default()
                        },
                    ));
                    stat.spawn((
                        TextBundle::from_section(
                            "100",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.984, 0.749, 0.141),
                                ..default()
                            },
                        ),
                        GoldCountText,
                    ));
                });

                // Multiplier
                stats.spawn(NodeBundle {
                    style: Style {
                        column_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|stat| {
                    stat.spawn(TextBundle::from_section(
                        "Multiplier:",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::srgb(0.796, 0.835, 0.882),
                            ..default()
                        },
                    ));
                    stat.spawn((
                        TextBundle::from_section(
                            "1.0x",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.13, 0.77, 0.37),
                                ..default()
                            },
                        ),
                        MultiplierText,
                    ));
                });
            });

            // Seed display
            header.spawn((
                TextBundle::from_section(
                    "Seed: 0",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.796, 0.835, 0.882),
                        ..default()
                    },
                ),
                SeedText,
            ));
        });

        // === MAIN CONTENT AREA (Three panels) ===
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|main_area| {
            // === LEFT SIDEBAR - Uncle Selection ===
            main_area.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(15.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                background_color: Color::srgba(0.094, 0.129, 0.196, 0.9).into(),
                ..default()
            })
            .with_children(|sidebar| {
                // Title
                sidebar.spawn(TextBundle::from_section(
                    "UNCLES",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.796, 0.835, 0.882),
                        ..default()
                    },
                ));

                // Uncle cards
                for uncle_type in [UncleType::Mongolian, UncleType::Somali, UncleType::Japanese] {
                    spawn_uncle_card(sidebar, uncle_type);
                }
            });

            // === CENTER - Game World ===
            main_area.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(65.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            });

            // === RIGHT SIDEBAR - Fish Feed ===
            main_area.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(20.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgba(0.094, 0.129, 0.196, 0.9).into(),
                ..default()
            })
            .with_children(|sidebar| {
                // Title
                sidebar.spawn(TextBundle::from_section(
                    "RECENT CATCHES",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.796, 0.835, 0.882),
                        ..default()
                    },
                ));

                // Fish feed container (scrollable list)
                sidebar.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(60.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.0),
                            padding: UiRect::all(Val::Px(5.0)),
                            overflow: Overflow::clip_y(),
                            ..default()
                        },
                        background_color: Color::srgba(0.059, 0.090, 0.165, 0.5).into(),
                        ..default()
                    },
                    FishFeedContainer,
                ));

                // Spacer
                sidebar.spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                });

                // Cash out section
                sidebar.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.118, 0.161, 0.231, 0.8).into(),
                    ..default()
                })
                .with_children(|cashout| {
                    // Current value
                    cashout.spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn(TextBundle::from_section(
                            "Catch Value:",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::srgb(0.796, 0.835, 0.882),
                                ..default()
                            },
                        ));
                        row.spawn((
                            TextBundle::from_section(
                                "0g",
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::srgb(0.984, 0.749, 0.141),
                                    ..default()
                                },
                            ),
                            CatchValueText,
                        ));
                    });

                    // Cooldown
                    cashout.spawn((
                        TextBundle::from_section(
                            "Ready!",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::srgb(0.13, 0.77, 0.37),
                                ..default()
                            },
                        ),
                        CooldownText,
                    ));

                    // Cash out button
                    cashout.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.13, 0.77, 0.37).into(),
                            ..default()
                        },
                        CashOutButton,
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "CASH OUT",
                            TextStyle {
                                font_size: 16.0,
                                color: Color::srgb(0.945, 0.961, 0.973),
                                ..default()
                            },
                        ));
                    });

                    // Instructions
                    cashout.spawn(TextBundle::from_section(
                        "Press SPACE or click button",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::srgb(0.6, 0.65, 0.7),
                            ..default()
                        },
                    ));
                });
            });
        });
    });
}

/// Helper function to spawn an uncle selection card
fn spawn_uncle_card(parent: &mut ChildBuilder, uncle_type: UncleType) {
    let is_selected = matches!(uncle_type, UncleType::Mongolian); // Default selection
    
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(5.0),
                border: UiRect::all(Val::Px(if is_selected { 2.0 } else { 0.0 })),
                ..default()
            },
            background_color: Color::srgba(0.118, 0.161, 0.231, 0.8).into(),
            border_color: if is_selected {
                Color::srgb(0.13, 0.77, 0.37).into()
            } else {
                Color::NONE.into()
            },
            ..default()
        },
        UncleSelectButton { uncle_type },
    ))
    .with_children(|card| {
        // Uncle identifier with color
        card.spawn(NodeBundle {
            style: Style {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: uncle_type.color().into(),
            ..default()
        })
        .with_children(|icon| {
            icon.spawn(TextBundle::from_section(
                uncle_type.letter(),
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.1, 0.1, 0.15),
                    ..default()
                },
            ));
        });

        // Name
        card.spawn(TextBundle::from_section(
            uncle_type.name(),
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.945, 0.961, 0.973),
                ..default()
            },
        ));

        // Cost and speed
        card.spawn(TextBundle::from_section(
            format!("{} gold • {:.1}s", uncle_type.cost(), uncle_type.speed_ms() as f32 / 1000.0),
            TextStyle {
                font_size: 12.0,
                color: Color::srgb(0.984, 0.749, 0.141),
                ..default()
            },
        ));

        // Ability
        card.spawn(TextBundle::from_section(
            uncle_type.ability(),
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.796, 0.835, 0.882),
                ..default()
            },
        ));
    });
}
