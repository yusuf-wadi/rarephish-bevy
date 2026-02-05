use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use crate::components::{UncleType, Fish};

/// Global game state resource
#[derive(Resource)]
pub struct GameState {
    pub fish_count: u32,
    pub gold: u32,
    pub current_catch: Vec<Fish>,  // Changed from CaughtFish to Fish
    pub multiplier: f32,
    pub cash_out_cooldown: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            fish_count: 0,
            gold: 100, // Starting gold
            current_catch: Vec::new(),
            multiplier: 1.0,
            cash_out_cooldown: 0.0,
        }
    }
}

/// World seed resource for procedural generation
#[derive(Resource)]
pub struct WorldSeed {
    pub seed: u64,
    pub rng: ChaCha8Rng,
}

impl Default for WorldSeed {
    fn default() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            seed,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

impl WorldSeed {
    pub fn new_seed(&mut self) {
        self.seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.rng = ChaCha8Rng::seed_from_u64(self.seed);
    }
}

/// Currently selected uncle type for placement
#[derive(Resource)]
pub struct SelectedUncle {
    pub uncle_type: UncleType,
}

impl Default for SelectedUncle {
    fn default() -> Self {
        Self {
            uncle_type: UncleType::Mongolian,
        }
    }
}
