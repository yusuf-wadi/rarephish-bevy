use bevy::prelude::*;
use rand::Rng;
use crate::components::{Tile, TileType};
use crate::constants::*;
use crate::resources::WorldSeed;

pub fn generate_tilemap(
    mut commands: Commands,
    mut world_seed: ResMut<WorldSeed>,
) {
    let center_x = TILE_WIDTH as f32 / 2.0;
    let center_y = TILE_HEIGHT as f32 / 2.0;

    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            // Use seeded RNG for deterministic generation
            let noise = world_seed.rng.gen::<f32>();

            // Calculate distance from center
            let dist_from_center = (
                (x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)
            ).sqrt();

            // Determine tile type
            let tile_type = if noise < WATER_THRESHOLD || dist_from_center < CENTER_WATER_RADIUS {
                TileType::Water
            } else {
                TileType::Land
            };

            // Calculate world position
            let world_x = (x as f32 - (TILE_WIDTH as f32 / 2.0)) * TILE_SIZE;
            let world_y = (y as f32 - (TILE_HEIGHT as f32 / 2.0)) * TILE_SIZE;

            // Spawn tile entity
            let color = match tile_type {
                TileType::Water => Color::srgb(0.047, 0.290, 0.431),
                TileType::Land => Color::srgb(0.086, 0.639, 0.290),
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y, 0.0),
                    ..default()
                },
                Tile {
                    x,
                    y,
                    tile_type,
                },
            ));
        }
    }
}

/// Check if a tile position is adjacent to water
pub fn is_near_water(x: usize, y: usize, tiles: &Query<&Tile>) -> bool {
    let directions = [
        (-1i32, -1i32), (0, -1), (1, -1),
        (-1, 0),                 (1, 0),
        (-1, 1),        (0, 1),  (1, 1),
    ];

    for (dx, dy) in directions {
        let check_x = x as i32 + dx;
        let check_y = y as i32 + dy;

        if check_x >= 0 && check_x < TILE_WIDTH as i32 && check_y >= 0 && check_y < TILE_HEIGHT as i32 {
            for tile in tiles.iter() {
                if tile.x == check_x as usize && tile.y == check_y as usize && tile.tile_type == TileType::Water {
                    return true;
                }
            }
        }
    }

    false
}
