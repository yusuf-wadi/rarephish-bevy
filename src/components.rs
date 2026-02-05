use bevy::prelude::*;

/// Marker component for tile entities
#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Water,
    Land,
}

/// Component for uncle entities placed on tiles
#[derive(Component)]
pub struct Uncle {
    pub uncle_type: UncleType,
    pub x: usize,
    pub y: usize,
    pub fishing_timer: Timer,
}

#[derive(Clone, Copy, PartialEq)]
pub enum UncleType {
    Mongolian,  // Basic: 2s, 50g, best retention
    Somali,     // Fast: 1.5s, 150g, good retention
    Japanese,   // RareFinder: 2.5s, 300g, bonus rare, worse retention
}

impl UncleType {
    pub fn speed_ms(&self) -> u64 {
        match self {
            UncleType::Mongolian => 2000,
            UncleType::Somali => 1500,
            UncleType::Japanese => 2500,
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            UncleType::Mongolian => 50,
            UncleType::Somali => 150,
            UncleType::Japanese => 300,
        }
    }

    /// Asset path for uncle sprite
    /// Return None to use fallback colored square + letter
    /// Return Some("path/to/sprite.png") to load from assets folder
    pub fn asset_path(&self) -> Option<&'static str> {
        match self {
            UncleType::Mongolian => None, // Future: Some("sprites/mongolian_uncle.png")
            UncleType::Somali => None,    // Future: Some("sprites/somali_uncle.png")
            UncleType::Japanese => None,  // Future: Some("sprites/japanese_uncle.png")
        }
    }

    /// Fallback letter identifier when no asset is available
    pub fn letter(&self) -> &'static str {
        match self {
            UncleType::Mongolian => "M",
            UncleType::Somali => "S",
            UncleType::Japanese => "J",
        }
    }

    /// Fallback color for colored square when no asset is available
    pub fn color(&self) -> Color {
        match self {
            UncleType::Mongolian => Color::srgb(0.824, 0.706, 0.549), // Sandy brown
            UncleType::Somali => Color::srgb(0.247, 0.596, 0.757),    // Ocean blue
            UncleType::Japanese => Color::srgb(0.969, 0.706, 0.788),  // Cherry blossom pink
        }
    }

    /// Emoji representation (for UI display only, not world sprites)
    pub fn emoji(&self) -> &'static str {
        match self {
            UncleType::Mongolian => "🏜️",
            UncleType::Somali => "🌊",
            UncleType::Japanese => "🗾",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            UncleType::Mongolian => "Mongolian Uncle",
            UncleType::Somali => "Somali Uncle",
            UncleType::Japanese => "Japanese Uncle",
        }
    }

    pub fn ability(&self) -> &'static str {
        match self {
            UncleType::Mongolian => "Strong Grip",
            UncleType::Somali => "Quick Reflexes",
            UncleType::Japanese => "Rare Finder",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UncleType::Mongolian => "Best at keeping fish captured",
            UncleType::Somali => "Fast catches + good retention",
            UncleType::Japanese => "Finds rare fish, weaker grip",
        }
    }

    pub fn rare_bonus(&self) -> f32 {
        match self {
            UncleType::Japanese => 0.05,
            _ => 0.0,
        }
    }

    /// Fish retention multiplier (lower = better at keeping fish)
    pub fn retention_multiplier(&self) -> f32 {
        match self {
            UncleType::Mongolian => 0.7,  // 30% better retention
            UncleType::Somali => 0.85,    // 15% better retention
            UncleType::Japanese => 1.2,   // 20% worse retention
        }
    }
}

/// Component for fish entities with escape physics
#[derive(Component, Clone)]
pub struct Fish {
    pub name: String,
    pub rarity: FishRarity,
    pub value: u32,
    pub time_alive: f32,              // Tracks metabolic phase
    pub failed_escape_attempts: u32,  // Builds "distance to safety"
    pub caught_by_uncle: UncleType,   // Which uncle caught it
}

impl Fish {
    /// Get current metabolic phase based on time alive
    pub fn get_phase(&self) -> MetabolicPhase {
        if self.time_alive < crate::constants::BURST_PHASE_DURATION {
            MetabolicPhase::Burst
        } else if self.time_alive < crate::constants::BURST_PHASE_DURATION + crate::constants::STOCHASTIC_PHASE_DURATION {
            MetabolicPhase::Stochastic
        } else {
            MetabolicPhase::Fatigue
        }
    }

    /// Calculate current escape chance using biased random walk model
    pub fn calculate_escape_chance(&self) -> f32 {
        use crate::constants::*;

        // 1. Base escape probability from metabolic phase
        let phase_base = match self.get_phase() {
            MetabolicPhase::Burst => BURST_ESCAPE_BASE,
            MetabolicPhase::Stochastic => STOCHASTIC_ESCAPE_BASE,
            MetabolicPhase::Fatigue => FATIGUE_ESCAPE_BASE,
        };

        // 2. Rarity modifier (rarer fish are more vigorous)
        let rarity_mult = match self.rarity {
            FishRarity::Common => COMMON_ESCAPE_MULTIPLIER,
            FishRarity::Uncommon => UNCOMMON_ESCAPE_MULTIPLIER,
            FishRarity::Rare => RARE_ESCAPE_MULTIPLIER,
        };

        // 3. Uncle retention ability
        let uncle_mult = self.caught_by_uncle.retention_multiplier();

        // 4. Reduction from failed attempts (building distance to safety)
        let failed_reduction = self.failed_escape_attempts as f32 * ESCAPE_REDUCTION_PER_FAIL;

        // 5. Combine all factors
        let final_chance = (phase_base * rarity_mult * uncle_mult - failed_reduction)
            .max(MIN_ESCAPE_CHANCE);

        final_chance
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MetabolicPhase {
    Burst,      // 0-10s: 3-5 Hz flops, highest intensity
    Stochastic, // 10-30s: 1-2 Hz flops, lactic buildup
    Fatigue,    // 30+s: <0.5 Hz, exhaustion
}

#[derive(Clone, Copy, PartialEq)]
pub enum FishRarity {
    Common,
    Uncommon,
    Rare,
}

impl FishRarity {
    pub fn color(&self) -> Color {
        match self {
            FishRarity::Common => Color::srgb(0.58, 0.64, 0.72),
            FishRarity::Uncommon => Color::srgb(0.13, 0.77, 0.37),
            FishRarity::Rare => Color::srgb(0.66, 0.33, 0.97),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FishRarity::Common => "Common",
            FishRarity::Uncommon => "Uncommon",
            FishRarity::Rare => "Rare",
        }
    }
}

// UI marker components
#[derive(Component)]
pub struct FishCountText;

#[derive(Component)]
pub struct GoldCountText;

#[derive(Component)]
pub struct MultiplierText;

#[derive(Component)]
pub struct SeedText;

#[derive(Component)]
pub struct CooldownText;

#[derive(Component)]
pub struct CatchValueText;

#[derive(Component)]
pub struct CashOutButton;

#[derive(Component)]
pub struct NewWorldButton;

#[derive(Component)]
pub struct UncleSelectButton {
    pub uncle_type: UncleType,
}

#[derive(Component)]
pub struct FishFeedContainer;

#[derive(Component)]
pub struct FishFeedEntry;
