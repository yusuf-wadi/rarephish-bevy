use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use crate::components::UncleType;
use crate::constants::*;

/// Global game state resource
#[derive(Resource)]
pub struct GameState {
    pub fish_count: u32,
    pub gold: u32,
    pub multiplier: f32,
    pub cash_out_cooldown: f32,  // Small cooldown to prevent spam clicks
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            fish_count: 0,
            gold: STARTING_GOLD,
            multiplier: 1.0,
            cash_out_cooldown: 0.0,
        }
    }
}

/// Day/Night cycle tracker
#[derive(Resource)]
pub struct DayNightCycle {
    pub time_elapsed: f32,         // Seconds elapsed in current cycle
    pub day_progress: f32,         // 0.0 to 1.0 (0 = midnight, 0.5 = noon)
    pub day_number: u32,           // Current day count
    pub is_day: bool,              // true = day, false = night
    pub cashouts_remaining: u32,   // Cash-outs left this day
    pub max_cashouts_per_day: u32, // Upgradeable limit
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self {
            time_elapsed: 0.0,
            day_progress: DAY_START_TIME,  // Start at morning
            day_number: 1,
            is_day: true,
            cashouts_remaining: STARTING_CASHOUTS_PER_DAY,
            max_cashouts_per_day: STARTING_CASHOUTS_PER_DAY,
        }
    }
}

impl DayNightCycle {
    /// Returns true if currently daytime
    pub fn is_daytime(&self) -> bool {
        self.day_progress >= DAY_START_TIME && self.day_progress < NIGHT_START_TIME
    }

    /// Returns readable time string (e.g., "6:00 AM", "3:30 PM")
    pub fn time_string(&self) -> String {
        let hours = (self.day_progress * 24.0) as u32;
        let minutes = ((self.day_progress * 24.0 * 60.0) % 60.0) as u32;
        let period = if hours < 12 { "AM" } else { "PM" };
        let display_hour = if hours == 0 { 12 } else if hours > 12 { hours - 12 } else { hours };
        format!("{:02}:{:02} {}", display_hour, minutes, period)
    }

    /// Resets cash-outs for new day
    pub fn new_day(&mut self) {
        self.day_number += 1;
        self.cashouts_remaining = self.max_cashouts_per_day;
    }

    /// Returns emoji for current time of day
    pub fn time_emoji(&self) -> &'static str {
        if self.day_progress < 0.25 {       // Midnight to 6 AM
            "🌙"
        } else if self.day_progress < 0.5 {  // 6 AM to Noon
            "🌅"
        } else if self.day_progress < 0.75 { // Noon to 6 PM
            "☀️"
        } else {                             // 6 PM to Midnight
            "🌆"
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
