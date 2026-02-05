use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::components::{Tile, TileType, Uncle, UncleType, FishRarity};
use crate::constants::*;
use crate::resources::{GameState, WorldSeed, SelectedUncle, CaughtFish};
use crate::systems::tilemap::is_near_water;

/// Handles mouse clicks for placing uncles on tiles
pub fn handle_uncle_placement(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tiles_q: Query<(Entity, &Tile, &Transform)>,
    uncles_q: Query<&Uncle>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    selected_uncle: Res<SelectedUncle>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor_position) = window.cursor_position() {
        // Convert screen position to world position
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            // Find clicked tile
            for (tile_entity, tile, tile_transform) in tiles_q.iter() {
                let tile_pos = tile_transform.translation.truncate();
                let half_size = TILE_SIZE / 2.0;

                if world_pos.x >= tile_pos.x - half_size
                    && world_pos.x <= tile_pos.x + half_size
                    && world_pos.y >= tile_pos.y - half_size
                    && world_pos.y <= tile_pos.y + half_size
                {
                    // Check placement validity
                    if tile.tile_type != TileType::Land {
                        return; // Can only place on land
                    }

                    // Check if tile is already occupied
                    let is_occupied = uncles_q.iter().any(|u| u.x == tile.x && u.y == tile.y);
                    if is_occupied {
                        return;
                    }

                    // Check if near water
                    if !is_near_water(tile.x, tile.y, &tiles_q) {
                        return;
                    }

                    // Check gold cost
                    let cost = selected_uncle.uncle_type.cost();
                    if game_state.gold < cost {
                        return;
                    }

                    // Place uncle
                    game_state.gold -= cost;

                    let speed_ms = selected_uncle.uncle_type.speed_ms();
                    commands.spawn((
                        Uncle {
                            uncle_type: selected_uncle.uncle_type,
                            x: tile.x,
                            y: tile.y,
                            fishing_timer: Timer::from_seconds(
                                speed_ms as f32 / 1000.0,
                                TimerMode::Repeating,
                            ),
                        },
                        Text2dBundle {
                            text: Text::from_section(
                                selected_uncle.uncle_type.sprite(),
                                TextStyle {
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            transform: Transform::from_xyz(
                                tile_transform.translation.x,
                                tile_transform.translation.y,
                                1.0,
                            ),
                            ..default()
                        },
                    ));

                    return;
                }
            }
        }
    }
}

/// Updates fishing timers and generates fish when timers complete
pub fn uncle_fishing_system(
    mut uncles_q: Query<&mut Uncle>,
    mut game_state: ResMut<GameState>,
    mut world_seed: ResMut<WorldSeed>,
    time: Res<Time>,
) {
    for mut uncle in uncles_q.iter_mut() {
        uncle.fishing_timer.tick(time.delta());

        if uncle.fishing_timer.just_finished() {
            // Generate a fish
            generate_fish(&mut game_state, &mut world_seed, uncle.uncle_type);
        }
    }
}

/// Generates a fish with random attributes based on rarity
fn generate_fish(
    game_state: &mut GameState,
    world_seed: &mut WorldSeed,
    uncle_type: UncleType,
) {
    let mut rng = &mut world_seed.rng;

    // Determine rarity with uncle bonus
    let rare_threshold = RARE_CHANCE + uncle_type.rare_bonus();
    let uncommon_threshold = rare_threshold + UNCOMMON_CHANCE;

    let roll = rng.gen::<f32>();
    let rarity = if roll < rare_threshold {
        FishRarity::Rare
    } else if roll < uncommon_threshold {
        FishRarity::Uncommon
    } else {
        FishRarity::Common
    };

    // Generate fish name
    let color = FISH_COLORS[rng.gen_range(0..FISH_COLORS.len())];
    let pattern = FISH_PATTERNS[rng.gen_range(0..FISH_PATTERNS.len())];
    let shape = FISH_SHAPES[rng.gen_range(0..FISH_SHAPES.len())];
    let name = format!("{} {} {}fish", color, pattern, shape);

    // Determine value based on rarity
    let value = match rarity {
        FishRarity::Common => rng.gen_range(COMMON_VALUE_MIN..=COMMON_VALUE_MAX),
        FishRarity::Uncommon => rng.gen_range(UNCOMMON_VALUE_MIN..=UNCOMMON_VALUE_MAX),
        FishRarity::Rare => rng.gen_range(RARE_VALUE_MIN..=RARE_VALUE_MAX),
    };

    // Store for potential escape check
    game_state.current_catch.push(CaughtFish {
        name,
        rarity,
        value,
    });
}

/// Checks if caught fish escape based on rarity-specific chances
pub fn fish_escape_system(
    mut game_state: ResMut<GameState>,
    mut world_seed: ResMut<WorldSeed>,
) {
    let mut rng = &mut world_seed.rng;
    let mut escaped_indices = Vec::new();

    for (i, fish) in game_state.current_catch.iter().enumerate() {
        let escape_chance = match fish.rarity {
            FishRarity::Common => COMMON_ESCAPE,
            FishRarity::Uncommon => UNCOMMON_ESCAPE,
            FishRarity::Rare => RARE_ESCAPE,
        };

        if rng.gen::<f32>() < escape_chance {
            escaped_indices.push(i);
        }
    }

    // Remove escaped fish (reverse order to maintain indices)
    for &i in escaped_indices.iter().rev() {
        game_state.current_catch.remove(i);
    }
}

/// Handles cash out action when cooldown is zero
pub fn cash_out_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    // Check if Space key pressed
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    // Check cooldown
    if game_state.cash_out_cooldown > 0.0 {
        return;
    }

    // Must have fish to cash out
    if game_state.current_catch.is_empty() {
        return;
    }

    // Calculate total value with multiplier
    let total_value: u32 = game_state.current_catch.iter().map(|f| f.value).sum();
    let gold_earned = (total_value as f32 * game_state.multiplier) as u32;

    // Update game state
    game_state.gold += gold_earned;
    game_state.fish_count += game_state.current_catch.len() as u32;
    game_state.current_catch.clear();

    // Increase multiplier
    game_state.multiplier = (game_state.multiplier + MULTIPLIER_INCREMENT).min(MAX_MULTIPLIER);

    // Set cooldown
    game_state.cash_out_cooldown = CASH_OUT_COOLDOWN;
}

/// Updates the cash out cooldown timer
pub fn cooldown_update_system(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    if game_state.cash_out_cooldown > 0.0 {
        game_state.cash_out_cooldown -= time.delta_seconds();
        game_state.cash_out_cooldown = game_state.cash_out_cooldown.max(0.0);
    }
}
