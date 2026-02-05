use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::srgb(0.059, 0.090, 0.165).into(),
        ..default()
    })
    .with_children(|parent| {
        // Header
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::srgb(0.118, 0.161, 0.231).into(),
            ..default()
        })
        .with_children(|header| {
            // Title
            header.spawn(TextBundle::from_section(
                "🎣 Rare Fish Game",
                TextStyle {
                    font_size: 32.0,
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
                stats.spawn(NodeBundle::default())
                    .with_children(|stat| {
                        stat.spawn(TextBundle::from_section(
                            "Fish: ",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.796, 0.835, 0.882),
                                ..default()
                            },
                        ));
                        stat.spawn((
                            TextBundle::from_section(
                                "0",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::srgb(0.945, 0.961, 0.973),
                                    ..default()
                                },
                            ),
                            crate::components::FishCountText,
                        ));
                    });

                // Gold
                stats.spawn(NodeBundle::default())
                    .with_children(|stat| {
                        stat.spawn(TextBundle::from_section(
                            "Gold: ",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.796, 0.835, 0.882),
                                ..default()
                            },
                        ));
                        stat.spawn((
                            TextBundle::from_section(
                                "100",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::srgb(0.984, 0.749, 0.141),
                                    ..default()
                                },
                            ),
                            crate::components::GoldCountText,
                        ));
                    });

                // Multiplier
                stats.spawn(NodeBundle::default())
                    .with_children(|stat| {
                        stat.spawn(TextBundle::from_section(
                            "Multiplier: ",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.796, 0.835, 0.882),
                                ..default()
                            },
                        ));
                        stat.spawn((
                            TextBundle::from_section(
                                "1.0x",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::srgb(0.945, 0.961, 0.973),
                                    ..default()
                                },
                            ),
                            crate::components::MultiplierText,
                        ));
                    });
            });

            // Seed display
            header.spawn((
                TextBundle::from_section(
                    "Seed: 0",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.796, 0.835, 0.882),
                        ..default()
                    },
                ),
                crate::components::SeedText,
            ));
        });

        // TODO: Main game area and sidebar will be added in future commits
    });
}
