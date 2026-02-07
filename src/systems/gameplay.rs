use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::components::{Tile, TileType, Uncle, UncleType, FishRarity, Fish, UncleBasket, SelectedUncleMarker};
use crate::constants::*;
use crate::resources::{GameState, WorldSeed, SelectedUncle, DayNightCycle};

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
    let basket_capacity = uncle_type.basket_capacity();
    
    let uncle = Uncle {
        uncle_type,
        x: tile_x,
        y: tile_y,
        fishing_timer: Timer::from_seconds(
            speed_ms as f32 / 1000.0,
            TimerMode::Repeating,
        ),
        basket: UncleBasket::new(basket_capacity),
    };

    if let Some(asset_path) = uncle_type.asset_path() {
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

/// Handles mouse clicks for placing uncles OR selecting placed uncles
pub fn handle_uncle_placement(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tiles_q: Query<(Entity, &Tile, &Transform)>,
    uncles_q: Query<(Entity, &Uncle, &Transform), Without<Tile>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    selected_uncle: Res<SelectedUncle>,
    selected_marker_q: Query<Entity, With<SelectedUncleMarker>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            // First check if clicking an existing uncle
            for (uncle_entity, _uncle, uncle_transform) in uncles_q.iter() {
                let uncle_pos = uncle_transform.translation.truncate();
                let half_size = UNCLE_SPRITE_SIZE / 2.0;

                if world_pos.x >= uncle_pos.x - half_size
                    && world_pos.x <= uncle_pos.x + half_size
                    && world_pos.y >= uncle_pos.y - half_size
                    && world_pos.y <= uncle_pos.y + half_size
                {
                    // Clicked an uncle! Select it
                    // Remove previous selection marker
                    for entity in selected_marker_q.iter() {
                        commands.entity(entity).remove::<SelectedUncleMarker>();
                    }
                    // Add marker to this uncle
                    commands.entity(uncle_entity).insert(SelectedUncleMarker);
                    return;
                }
            }

            // Not clicking uncle, try to place new one
            for (_tile_entity, tile, tile_transform) in tiles_q.iter() {
                let tile_pos = tile_transform.translation.truncate();
                let half_size = TILE_SIZE / 2.0;

                if world_pos.x >= tile_pos.x - half_size
                    && world_pos.x <= tile_pos.x + half_size
                    && world_pos.y >= tile_pos.y - half_size
                    && world_pos.y <= tile_pos.y + half_size
                {
                    if tile.tile_type != TileType::Land {
                        return;
                    }

                    let is_occupied = uncles_q.iter().any(|(_, u, _)| u.x == tile.x && u.y == tile.y);
                    if is_occupied {
                        return;
                    }

                    if !is_tile_near_water(tile.x, tile.y, &tiles_q) {
                        return;
                    }

                    let cost = selected_uncle.uncle_type.cost();
                    if game_state.gold < cost {
                        return;
                    }

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

/// Updates fishing timers and adds fish to individual uncle baskets
pub fn uncle_fishing_system(
    mut uncles_q: Query<&mut Uncle>,
    mut world_seed: ResMut<WorldSeed>,
    time: Res<Time>,
) {
    for mut uncle in uncles_q.iter_mut() {
        // Skip if basket is full
        if uncle.basket.is_full() {
            continue;
        }

        uncle.fishing_timer.tick(time.delta());

        if uncle.fishing_timer.just_finished() {
            // Generate a fish and add to uncle's basket
            let fish = generate_fish(&mut world_seed, uncle.uncle_type);
            uncle.basket.add_fish(fish);
        }
    }
}

/// Generates a fish with random attributes
fn generate_fish(
    world_seed: &mut WorldSeed,
    uncle_type: UncleType,
) -> Fish {
    let rng = &mut world_seed.rng;

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

    let color = FISH_COLORS[rng.gen_range(0..FISH_COLORS.len())];
    let pattern = FISH_PATTERNS[rng.gen_range(0..FISH_PATTERNS.len())];
    let shape = FISH_SHAPES[rng.gen_range(0..FISH_SHAPES.len())];
    let name = format!("{} {} {}fish", color, pattern, shape);

    let value = match rarity {
        FishRarity::Common => rng.gen_range(COMMON_VALUE_MIN..=COMMON_VALUE_MAX),
        FishRarity::Uncommon => rng.gen_range(UNCOMMON_VALUE_MIN..=UNCOMMON_VALUE_MAX),
        FishRarity::Rare => rng.gen_range(RARE_VALUE_MIN..=RARE_VALUE_MAX),
    };

    Fish {
        name,
        rarity,
        value,
        time_alive: 0.0,
        failed_escape_attempts: 0,
        caught_by_uncle: uncle_type,
    }
}

/// Fish escape system now works on individual uncle baskets
pub fn fish_escape_system(
    mut uncles_q: Query<&mut Uncle>,
    mut world_seed: ResMut<WorldSeed>,
    time: Res<Time>,
) {
    let rng = &mut world_seed.rng;
    let delta = time.delta_seconds();

    for mut uncle in uncles_q.iter_mut() {
        let mut escaped_indices = Vec::new();

        // Check each fish in this uncle's basket
        for (i, fish) in uncle.basket.fish.iter_mut().enumerate() {
            fish.time_alive += delta;
            let escape_chance_per_second = fish.calculate_escape_chance();
            let escape_chance_this_frame = escape_chance_per_second * delta;

            if rng.gen::<f32>() < escape_chance_this_frame {
                escaped_indices.push(i);
            } else {
                fish.failed_escape_attempts += 1;
            }
        }

        // Remove escaped fish
        for &i in escaped_indices.iter().rev() {
            uncle.basket.fish.remove(i);
        }
    }
}

/// Remove lowest value fish from selected uncle's basket (R key)
/// Strategic use: free up space for potentially better fish
pub fn remove_fish_from_basket(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut uncles_q: Query<&mut Uncle, With<SelectedUncleMarker>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    for mut uncle in uncles_q.iter_mut() {
        if uncle.basket.fish.is_empty() {
            continue;
        }

        // Find lowest value fish
        let lowest_idx = uncle.basket.fish
            .iter()
            .enumerate()
            .min_by_key(|(_, fish)| fish.value)
            .map(|(idx, _)| idx);

        if let Some(idx) = lowest_idx {
            let removed = uncle.basket.fish.remove(idx);
            println!("🗑️ Removed {} ({}g) to make space", removed.name, removed.value);
        }
    }
}

/// Cash out selected uncle's basket
pub fn cash_out_selected_uncle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut uncles_q: Query<&mut Uncle, With<SelectedUncleMarker>>,
    mut game_state: ResMut<GameState>,
    mut day_night: ResMut<DayNightCycle>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    // Check cooldown
    if game_state.cash_out_cooldown > 0.0 {
        return;
    }

    // Check daily limit
    if day_night.cashouts_remaining == 0 {
        println!("❌ No cash-outs remaining! Wait for day {} ({})", day_night.day_number + 1, day_night.time_string());
        return;
    }

    // Cash out the selected uncle
    for mut uncle in uncles_q.iter_mut() {
        if uncle.basket.fish.is_empty() {
            continue;
        }

        let total_value = uncle.basket.total_value();
        let gold_earned = (total_value as f32 * game_state.multiplier) as u32;
        let fish_count = uncle.basket.fish.len() as u32;

        game_state.gold += gold_earned;
        game_state.fish_count += fish_count;
        uncle.basket.cash_out();

        game_state.multiplier = (game_state.multiplier + MULTIPLIER_INCREMENT).min(MAX_MULTIPLIER);
        game_state.cash_out_cooldown = CASH_OUT_COOLDOWN;
        day_night.cashouts_remaining -= 1;
        
        println!("💰 Cashed out {} fish for {}g! Remaining: {}/{}", 
                 fish_count, gold_earned, day_night.cashouts_remaining, day_night.max_cashouts_per_day);
        break;
    }
}

/// Cash out ALL uncles at once
pub fn cash_out_all_uncles(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut uncles_q: Query<&mut Uncle>,
    mut game_state: ResMut<GameState>,
    mut day_night: ResMut<DayNightCycle>,
) {
    // Must press A key to trigger
    if !keyboard.just_pressed(KeyCode::KeyA) {
        return;
    }

    // Check cooldown
    if game_state.cash_out_cooldown > 0.0 {
        return;
    }

    // Check daily limit
    if day_night.cashouts_remaining == 0 {
        println!("❌ No cash-outs remaining! Wait for day {} ({})", day_night.day_number + 1, day_night.time_string());
        return;
    }

    let mut total_gold = 0;
    let mut total_fish = 0;

    for mut uncle in uncles_q.iter_mut() {
        if !uncle.basket.fish.is_empty() {
            let value = uncle.basket.total_value();
            total_gold += value;
            total_fish += uncle.basket.fish.len() as u32;
            uncle.basket.cash_out();
        }
    }

    if total_fish > 0 {
        let gold_earned = (total_gold as f32 * game_state.multiplier) as u32;
        game_state.gold += gold_earned;
        game_state.fish_count += total_fish;
        game_state.multiplier = (game_state.multiplier + MULTIPLIER_INCREMENT).min(MAX_MULTIPLIER);
        game_state.cash_out_cooldown = CASH_OUT_COOLDOWN;
        day_night.cashouts_remaining -= 1;
        
        println!("💰 Cashed out ALL: {} fish for {}g! Remaining: {}/{}", 
                 total_fish, gold_earned, day_night.cashouts_remaining, day_night.max_cashouts_per_day);
    }
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
