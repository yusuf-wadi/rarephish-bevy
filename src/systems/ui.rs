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
    mut value_text_q: Query<&mut Text, (With<CatchValueText>, Without<FishCountText>, Without<GoldCountText>, Without<MultiplierText>, Without<SeedText>, Without<CooldownText>)>,
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

    // Update catch value
    if let Ok(mut text) = value_text_q.get_single_mut() {
        let total: u32 = game_state.current_catch.iter().map(|f| f.value).sum();
        let with_mult = (total as f32 * game_state.multiplier) as u32;
        text.sections[0].value = format!("{}g", with_mult);
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

/// Handles cash out button clicks
pub fn handle_cash_out_button(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<CashOutButton>)>,
    mut game_state: ResMut<GameState>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            // Check if can cash out
            if game_state.cash_out_cooldown > 0.0 || game_state.current_catch.is_empty() {
                continue;
            }

            // Perform cash out
            let total_value: u32 = game_state.current_catch.iter().map(|f| f.value).sum();
            let gold_earned = (total_value as f32 * game_state.multiplier) as u32;

            game_state.gold += gold_earned;
            game_state.fish_count += game_state.current_catch.len() as u32;
            game_state.current_catch.clear();
            game_state.multiplier = (game_state.multiplier + 0.1).min(5.0);
            game_state.cash_out_cooldown = 30.0;
        }
    }
}

/// Updates fish feed display when new fish are caught
pub fn update_fish_feed(
    game_state: Res<GameState>,
    mut commands: Commands,
    feed_container: Query<Entity, With<FishFeedContainer>>,
    existing_entries: Query<Entity, With<FishFeedEntry>>,
) {
    // Only update if game state changed
    if !game_state.is_changed() {
        return;
    }

    let container = match feed_container.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    // Clear existing entries
    for entry in existing_entries.iter() {
        commands.entity(entry).despawn_recursive();
    }

    // Add new entries (last 10 fish in current catch)
    let recent_fish: Vec<_> = game_state.current_catch.iter()
        .rev()
        .take(10)
        .collect();

    for fish in recent_fish {
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
                        font_size: 13.0,
                        color: Color::srgb(0.945, 0.961, 0.973),
                        ..default()
                    },
                ));

                // Rarity and value
                entry.spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(TextBundle::from_section(
                        fish.rarity.name(),
                        TextStyle {
                            font_size: 11.0,
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

/// Visual feedback for cash out button hover
pub fn cash_out_button_visual(
    mut interaction_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CashOutButton>)>,
    game_state: Res<GameState>,
) {
    for (interaction, mut color) in interaction_q.iter_mut() {
        let can_cash_out = game_state.cash_out_cooldown <= 0.0 && !game_state.current_catch.is_empty();
        
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
            // Generate new seed
            world_seed.new_seed();

            // Reset game state
            game_state.fish_count = 0;
            game_state.gold = 100;
            game_state.current_catch.clear();
            game_state.multiplier = 1.0;
            game_state.cash_out_cooldown = 0.0;

            // Despawn all tiles
            for entity in tiles_q.iter() {
                commands.entity(entity).despawn();
            }

            // Despawn all uncles
            for entity in uncles_q.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}
