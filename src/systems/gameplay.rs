use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::components::{Tile, TileType, Uncle, UncleType, FishRarity, Fish};
use crate::constants::*;
use crate::resources::{GameState, WorldSeed, SelectedUncle};

/// Helper function to check if a tile position is near water
fn is_tile_near_water(x: usize, y: usize, tiles_q: &Query<(Entity, &Tile, &Transform)>) -> bool {
    let directions = [
        (-1i32, -1i32), (0, -1), (1, -1),
        (-1, 0),                 (1, 0),
        (-1, 1),        (0, 1),  (1, 1),
    ];

    for (dx, dy) in directions {
        let check_x = x as i32 + dx;
        let check_y = y as i32 + dy;

        if check_x >= 0 && check_x < TILE_WIDTH as i32 && check_y >= 0 && check_y < TILE_HEIGHT as i32 {
            for (_entity, tile, _transform) in tiles_q.iter() {
                if tile.x == check_x as usize && tile.y == check_y as usize && tile.tile_type == TileType::Water {
                    return true;
                }
            }
        }
    }

    false
}

/// Spawns an uncle entity at a given position with sprite support
fn spawn_uncle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    uncle_type: UncleType,
    world_x: f32,
    world_y: f32,
    tile_x: usize,
    tile_y: usize,
) {
    let speed_ms = uncle_type.speed_ms();
    let uncle = Uncle {
        uncle_type,
        x: tile_x,
        y: tile_y,
        fishing_timer: Timer::from_seconds(
            speed_ms as f32 / 1000.0,
            TimerMode::Repeating,
        ),
    };

    // Check if we have an asset path, otherwise use fallback
    if let Some(asset_path) = uncle_type.asset_path() {
        // Load sprite from assets folder
        commands.spawn((
            uncle,
            SpriteBundle {
                texture: asset_server.load(asset_path),
                transform: Transform::from_xyz(world_x, world_y, 2.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(UNCLE_SPRITE_SIZE, UNCLE_SPRITE_SIZE)),
                    ..default()
                },
                ..default()
            },
        ));
    } else {
        // Fallback: colored square with letter overlay
        let uncle_entity = commands.spawn((
            uncle,
            SpriteBundle {
                sprite: Sprite {
                    color: uncle_type.color(),
                    custom_size: Some(Vec2::new(UNCLE_SPRITE_SIZE, UNCLE_SPRITE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(world_x, world_y, 2.0),
                ..default()
            },
        )).id();

        // Add letter text as child entity
        commands.entity(uncle_entity).with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    uncle_type.letter(),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.1, 0.1, 0.15),
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
        });
    }
}

/// Handles mouse clicks for placing uncles on tiles
pub fn handle_uncle_placement(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tiles_q: Query<(Entity, &Tile, &Transform)>,
    uncles_q: Query<&Uncle>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            // Find clicked tile
            for (_tile_entity, tile, tile_transform) in tiles_q.iter() {
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
                    if !is_tile_near_water(tile.x, tile.y, &tiles_q) {
                        return;
                    }

                    // Check gold cost
                    let cost = selected_uncle.uncle_type.cost();
                    if game_state.gold < cost {
                        return;
                    }

                    // Place uncle
                    game_state.gold -= cost;
                    spawn_uncle(
                        &mut commands,
                        &asset_server,
                        selected_uncle.uncle_type,
                        tile_transform.translation.x,
                        tile_transform.translation.y,
                        tile.x,
                        tile.y,
                    );

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
    let rng = &mut world_seed.rng;

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

    // Create fish with physics-based escape state
    let fish = Fish {
        name,
        rarity,
        value,
        time_alive: 0.0,
        failed_escape_attempts: 0,
        caught_by_uncle: uncle_type,
    };

    game_state.current_catch.push(fish);
}

/// Physics-based fish escape system using biased random walk model
/// Models fish flopping with metabolic phases and decreasing success per failed attempt
pub fn fish_escape_system(
    mut game_state: ResMut<GameState>,
    mut world_seed: ResMut<WorldSeed>,
    time: Res<Time>,
) {
    let rng = &mut world_seed.rng;
    let delta = time.delta_seconds();
    let mut escaped_indices = Vec::new();

    // Update each fish's time and check for escape
    for (i, fish) in game_state.current_catch.iter_mut().enumerate() {
        // Update time alive
        fish.time_alive += delta;

        // Calculate escape chance using biased random walk model
        let escape_chance_per_second = fish.calculate_escape_chance();
        
        // Convert per-second probability to per-frame probability
        // Using: P(frame) = 1 - (1 - P(second))^(delta)
        // Approximation for small probabilities: P(frame) ≈ P(second) * delta
        let escape_chance_this_frame = escape_chance_per_second * delta;

        // Roll for escape
        if rng.gen::<f32>() < escape_chance_this_frame {
            // Fish successfully escapes!
            escaped_indices.push(i);
        } else {
            // Failed escape attempt - fish moved wrong direction or fatigued
            fish.failed_escape_attempts += 1;
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
