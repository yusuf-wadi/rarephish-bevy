use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use crate::components::*;
use crate::resources::{GameState, WorldSeed, SelectedUncle};

/// Updates all UI text displays based on current game state
pub fn update_ui_system(
    game_state: Res<GameState>,
    world_seed: Res<WorldSeed>,
    mut fish_text_q: Query<&mut Text, (With<FishCountText>, Without<GoldCountText>, Without<MultiplierText>, Without<SeedText>, Without<CooldownText>, Without<CatchValueText>)>,
    mut gold_text_q: Query<&mut Text, (With<GoldCountText>, Without<FishCountText>, Without<MultiplierText>, Without<SeedText>, Without<CooldownText>, Without<CatchValueText>)>,
    mut mult_text_q: Query<&mut Text, (With<MultiplierText>, Without<FishCountText>, Without<GoldCountText>, Without<SeedText>, Without<CooldownText>, Without<CatchValueText>)>,
    mut seed_text_q: Query<&mut Text, (With<SeedText>, Without<FishCountText>, Without<GoldCountText>, Without<MultiplierText>, Without<CooldownText>, Without<CatchValueText>)>,
    mut cooldown_text_q: Query<&mut Text, (With<CooldownText>, Without<FishCountText>, Without<GoldCountText>, Without<MultiplierText>, Without<SeedText>, Without<CatchValueText>)>,
) {
    // Update fish count
    if let Ok(mut text) = fish_text_q.get_single_mut() {
        text.sections[0].value = format!("{}", game_state.fish_count);
    }

    // Update gold count
    if let Ok(mut text) = gold_text_q.get_single_mut() {
        text.sections[0].value = format!("{}", game_state.gold);
    }

    // Update multiplier
    if let Ok(mut text) = mult_text_q.get_single_mut() {
        text.sections[0].value = format!("{:.1}x", game_state.multiplier);
    }

    // Update seed
    if let Ok(mut text) = seed_text_q.get_single_mut() {
        text.sections[0].value = format!("Seed: {}", world_seed.seed);
    }

    // Update cooldown display
    if let Ok(mut text) = cooldown_text_q.get_single_mut() {
        if game_state.cash_out_cooldown > 0.0 {
            text.sections[0].value = format!("Cooldown: {:.0}s", game_state.cash_out_cooldown);
            text.sections[0].style.color = Color::srgb(0.9, 0.4, 0.4);
        } else {
            text.sections[0].value = "Ready!".to_string();
            text.sections[0].style.color = Color::srgb(0.13, 0.77, 0.37);
        }
    }
}

/// Updates basket value display for selected uncle
pub fn update_basket_value_display(
    uncles_q: Query<&Uncle, With<SelectedUncleMarker>>,
    mut value_text_q: Query<&mut Text, With<CatchValueText>>,
    game_state: Res<GameState>,
) {
    if let Ok(mut text) = value_text_q.get_single_mut() {
        if let Ok(uncle) = uncles_q.get_single() {
            // Show selected uncle's basket value
            let total = uncle.basket.total_value();
            let with_mult = (total as f32 * game_state.multiplier) as u32;
            text.sections[0].value = format!("{}g", with_mult);
        } else {
            // No uncle selected
            text.sections[0].value = "Select uncle".to_string();
        }
    }
}

/// Updates basket display showing fish in selected uncle's basket
pub fn update_basket_display(
    uncles_q: Query<&Uncle, (With<SelectedUncleMarker>, Changed<Uncle>)>,
    mut commands: Commands,
    basket_container: Query<Entity, With<UncleBasketDisplay>>,
    existing_entries: Query<Entity, With<FishFeedEntry>>,
) {
    // Only update if selection changed or uncle basket changed
    let uncle = match uncles_q.get_single() {
        Ok(u) => u,
        Err(_) => {
            // No uncle selected - clear display
            let container = match basket_container.get_single() {
                Ok(entity) => entity,
                Err(_) => return,
            };

            for entry in existing_entries.iter() {
                commands.entity(entry).despawn_recursive();
            }

            // Show instruction message
            commands.entity(container).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Right-click an uncle\nto view their basket",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.6, 0.65, 0.7),
                            ..default()
                        },
                    ),
                    FishFeedEntry,
                ));
            });
            return;
        }
    };

    let container = match basket_container.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    // Clear existing entries
    for entry in existing_entries.iter() {
        commands.entity(entry).despawn_recursive();
    }

    // Show uncle info header
    commands.entity(container).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(5.0)),
                    row_gap: Val::Px(3.0),
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                background_color: uncle.uncle_type.color().with_alpha(0.3).into(),
                ..default()
            },
            FishFeedEntry,
        ))
        .with_children(|header| {
            header.spawn(TextBundle::from_section(
                format!("{} {}", uncle.uncle_type.emoji(), uncle.uncle_type.name()),
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.945, 0.961, 0.973),
                    ..default()
                },
            ));
            header.spawn(TextBundle::from_section(
                format!(
                    "Basket: {}/{} fish",
                    uncle.basket.fish.len(),
                    uncle.basket.capacity
                ),
                TextStyle {
                    font_size: 12.0,
                    color: if uncle.basket.is_full() {
                        Color::srgb(0.9, 0.4, 0.4)
                    } else {
                        Color::srgb(0.984, 0.749, 0.141)
                    },
                    ..default()
                },
            ));
        });
    });

    // Show fish in basket
    if uncle.basket.fish.is_empty() {
        commands.entity(container).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "No fish yet...",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.6, 0.65, 0.7),
                        ..default()
                    },
                ),
                FishFeedEntry,
            ));
        });
    } else {
        for fish in uncle.basket.fish.iter() {
            commands.entity(container).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(5.0)),
                            row_gap: Val::Px(2.0),
                            ..default()
                        },
                        background_color: Color::srgba(0.118, 0.161, 0.231, 0.6).into(),
                        ..default()
                    },
                    FishFeedEntry,
                ))
                .with_children(|entry| {
                    // Fish name
                    entry.spawn(TextBundle::from_section(
                        &fish.name,
                        TextStyle {
                            font_size: 12.0,
                            color: Color::srgb(0.945, 0.961, 0.973),
                            ..default()
                        },
                    ));

                    // Rarity, value, and phase
                    entry.spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn(TextBundle::from_section(
                            format!("{} • {}s", fish.rarity.name(), fish.time_alive as u32),
                            TextStyle {
                                font_size: 10.0,
                                color: fish.rarity.color(),
                                ..default()
                            },
                        ));
                        row.spawn(TextBundle::from_section(
                            format!("{}g", fish.value),
                            TextStyle {
                                font_size: 11.0,
                                color: Color::srgb(0.984, 0.749, 0.141),
                                ..default()
                            },
                        ));
                    });
                });
            });
        }
    }
}

/// Handles uncle type selection button clicks with visual feedback
pub fn handle_uncle_selection(
    interaction_q: Query<(&Interaction, &UncleSelectButton), Changed<Interaction>>,
    mut button_query: ParamSet<(
        Query<(&UncleSelectButton, &mut BorderColor)>,
    )>,
    mut selected_uncle: ResMut<SelectedUncle>,
) {
    // Check for interactions first
    let mut selected_type: Option<UncleType> = None;
    for (interaction, button) in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            selected_type = Some(button.uncle_type);
            selected_uncle.uncle_type = button.uncle_type;
            break;
        }
    }

    // Update borders if selection changed
    if let Some(selected) = selected_type {
        for (button, mut border) in button_query.p0().iter_mut() {
            if button.uncle_type == selected {
                *border = Color::srgb(0.13, 0.77, 0.37).into();
            } else {
                *border = Color::NONE.into();
            }
        }
    }
}

/// Handles cash out button clicks (selected uncle only)
pub fn handle_cash_out_button(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<CashOutButton>)>,
    mut uncles_q: Query<&mut Uncle, With<SelectedUncleMarker>>,
    mut game_state: ResMut<GameState>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            if game_state.cash_out_cooldown > 0.0 {
                continue;
            }

            // Cash out selected uncle if any
            if let Ok(mut uncle) = uncles_q.get_single_mut() {
                if uncle.basket.fish.is_empty() {
                    continue;
                }

                let caught_fish = uncle.basket.cash_out();
                let total_value: u32 = caught_fish.iter().map(|f| f.value).sum();
                let gold_earned = (total_value as f32 * game_state.multiplier) as u32;

                game_state.gold += gold_earned;
                game_state.fish_count += caught_fish.len() as u32;
                game_state.multiplier = (game_state.multiplier + 0.1).min(5.0);
                game_state.cash_out_cooldown = 30.0;
            }
        }
    }
}

/// Handles cash out ALL button clicks
pub fn handle_cash_out_all_button(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<CashOutAllButton>)>,
    mut uncles_q: Query<&mut Uncle>,
    mut game_state: ResMut<GameState>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            if game_state.cash_out_cooldown > 0.0 {
                continue;
            }

            let mut total_fish = 0;
            let mut total_value = 0;

            for mut uncle in uncles_q.iter_mut() {
                if !uncle.basket.fish.is_empty() {
                    let caught_fish = uncle.basket.cash_out();
                    total_fish += caught_fish.len();
                    total_value += caught_fish.iter().map(|f| f.value).sum::<u32>();
                }
            }

            if total_fish > 0 {
                let gold_earned = (total_value as f32 * game_state.multiplier) as u32;
                game_state.gold += gold_earned;
                game_state.fish_count += total_fish as u32;
                game_state.multiplier = (game_state.multiplier + 0.1).min(5.0);
                game_state.cash_out_cooldown = 30.0;
            }
        }
    }
}

/// Visual feedback for cash out button hover
pub fn cash_out_button_visual(
    mut interaction_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CashOutButton>)>,
    game_state: Res<GameState>,
) {
    for (interaction, mut color) in interaction_q.iter_mut() {
        let can_cash_out = game_state.cash_out_cooldown <= 0.0;
        
        match *interaction {
            Interaction::Pressed => {
                if can_cash_out {
                    *color = Color::srgb(0.10, 0.60, 0.30).into();
                }
            }
            Interaction::Hovered => {
                if can_cash_out {
                    *color = Color::srgb(0.15, 0.85, 0.42).into();
                } else {
                    *color = Color::srgb(0.5, 0.5, 0.5).into();
                }
            }
            Interaction::None => {
                if can_cash_out {
                    *color = Color::srgb(0.13, 0.77, 0.37).into();
                } else {
                    *color = Color::srgb(0.4, 0.4, 0.4).into();
                }
            }
        }
    }
}

/// Visual feedback for cash out ALL button hover
pub fn cash_out_all_button_visual(
    mut interaction_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CashOutAllButton>)>,
    game_state: Res<GameState>,
) {
    for (interaction, mut color) in interaction_q.iter_mut() {
        let can_cash_out = game_state.cash_out_cooldown <= 0.0;
        
        match *interaction {
            Interaction::Pressed => {
                if can_cash_out {
                    *color = Color::srgb(0.20, 0.50, 0.65).into();
                }
            }
            Interaction::Hovered => {
                if can_cash_out {
                    *color = Color::srgb(0.30, 0.65, 0.85).into();
                } else {
                    *color = Color::srgb(0.5, 0.5, 0.5).into();
                }
            }
            Interaction::None => {
                if can_cash_out {
                    *color = Color::srgb(0.247, 0.596, 0.757).into();
                } else {
                    *color = Color::srgb(0.4, 0.4, 0.4).into();
                }
            }
        }
    }
}

/// Visual feedback for uncle selection buttons
pub fn uncle_button_visual(
    mut interaction_q: Query<(&Interaction, &mut BackgroundColor, &UncleSelectButton), Changed<Interaction>>,
) {
    for (interaction, mut color, _button) in interaction_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgba(0.15, 0.20, 0.28, 0.9).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.14, 0.19, 0.27, 0.9).into();
            }
            Interaction::None => {
                *color = Color::srgba(0.118, 0.161, 0.231, 0.8).into();
            }
        }
    }
}

/// Handles new world generation button click (currently unused but ready for future)
pub fn handle_new_world(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<NewWorldButton>)>,
    mut commands: Commands,
    mut world_seed: ResMut<WorldSeed>,
    mut game_state: ResMut<GameState>,
    tiles_q: Query<Entity, With<Tile>>,
    uncles_q: Query<Entity, With<Uncle>>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            world_seed.new_seed();
            game_state.fish_count = 0;
            game_state.gold = 100;
            game_state.multiplier = 1.0;
            game_state.cash_out_cooldown = 0.0;

            for entity in tiles_q.iter() {
                commands.entity(entity).despawn();
            }
            for entity in uncles_q.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}
