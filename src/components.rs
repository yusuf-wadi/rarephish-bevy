use bevy::prelude::*;

use crate::constants;

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
    pub basket: UncleBasket,
}

/// Individual uncle's fishing basket
#[derive(Clone)]
pub struct UncleBasket {
    pub fish: Vec<Fish>,
    pub capacity: usize,
}

impl UncleBasket {
    pub fn new(capacity: usize) -> Self {
        Self {
            fish: Vec::new(),
            capacity,
        }
    }

    pub fn is_full(&self) -> bool {
        self.fish.len() >= self.capacity
    }

    pub fn space_remaining(&self) -> usize {
        self.capacity.saturating_sub(self.fish.len())
    }

    pub fn add_fish(&mut self, fish: Fish) -> bool {
        if !self.is_full() {
            self.fish.push(fish);
            true
        } else {
            false
        }
    }

    pub fn total_value(&self) -> u32 {
        self.fish.iter().map(|f| f.value).sum()
    }

    pub fn cash_out(&mut self) -> Vec<Fish> {
        std::mem::take(&mut self.fish)
    }
}

/// Marker for selected uncle (shows their basket in UI)
#[derive(Component)]
pub struct SelectedUncleMarker;

#[derive(Clone, Copy, PartialEq)]
pub enum UncleType {
    Mongolian,  // Basic: 2s, 50g, best retention, small basket
    Somali,     // Fast: 1.5s, 150g, good retention, medium basket
    Japanese,   // RareFinder: 2.5s, 300g, bonus rare, large basket
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
            UncleType::Mongolian => constants::MONGOLIAN_COST,
            UncleType::Somali => constants::SOMALI_COST,
            UncleType::Japanese => constants::JAPANESE_COST,
        }
    }

    /// Basket capacity - trade-off with other stats
    pub fn basket_capacity(&self) -> usize {
        match self {
            UncleType::Mongolian => 5,   // Small basket, best retention
            UncleType::Somali => 8,      // Medium basket, fast speed
            UncleType::Japanese => 12,   // Large basket, rare finder
        }
    }

    /// Asset path for uncle sprite
    pub fn asset_path(&self) -> Option<&'static str> {
        match self {
            UncleType::Mongolian => None,
            UncleType::Somali => None,
            UncleType::Japanese => None,
        }
    }

    pub fn letter(&self) -> &'static str {
        match self {
            UncleType::Mongolian => "M",
            UncleType::Somali => "S",
            UncleType::Japanese => "J",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            UncleType::Mongolian => Color::srgb(0.824, 0.706, 0.549),
            UncleType::Somali => Color::srgb(0.247, 0.596, 0.757),
            UncleType::Japanese => Color::srgb(0.969, 0.706, 0.788),
        }
    }

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
            UncleType::Mongolian => "Best retention, small basket",
            UncleType::Somali => "Fast + good retention, medium basket",
            UncleType::Japanese => "Rare fish, large basket, weak grip",
        }
    }

    pub fn rare_bonus(&self) -> f32 {
        match self {
            UncleType::Japanese => 0.05,
            _ => 0.0,
        }
    }

    pub fn retention_multiplier(&self) -> f32 {
        match self {
            UncleType::Mongolian => constants::MONGOLIAN_RETENTION,
            UncleType::Somali => constants::SOMALI_RETENTION,
            UncleType::Japanese => constants::JAPANESE_RETENTION,
        }
    }
}

/// Component for fish entities with escape physics
#[derive(Clone)]
pub struct Fish {
    pub name: String,
    pub rarity: FishRarity,
    pub value: u32,
    pub time_alive: f32,
    pub failed_escape_attempts: u32,
    pub caught_by_uncle: UncleType,
}

impl Fish {
    pub fn get_phase(&self) -> MetabolicPhase {
        if self.time_alive < crate::constants::BURST_PHASE_DURATION {
            MetabolicPhase::Burst
        } else if self.time_alive < crate::constants::BURST_PHASE_DURATION + crate::constants::STOCHASTIC_PHASE_DURATION {
            MetabolicPhase::Stochastic
        } else {
            MetabolicPhase::Fatigue
        }
    }

    pub fn calculate_escape_chance(&self) -> f32 {
        use crate::constants::*;

        let phase_base = match self.get_phase() {
            MetabolicPhase::Burst => BURST_ESCAPE_BASE,
            MetabolicPhase::Stochastic => STOCHASTIC_ESCAPE_BASE,
            MetabolicPhase::Fatigue => FATIGUE_ESCAPE_BASE,
        };

        let rarity_mult = match self.rarity {
            FishRarity::Common => COMMON_ESCAPE_MULTIPLIER,
            FishRarity::Uncommon => UNCOMMON_ESCAPE_MULTIPLIER,
            FishRarity::Rare => RARE_ESCAPE_MULTIPLIER,
        };

        let uncle_mult = self.caught_by_uncle.retention_multiplier();
        let failed_reduction = self.failed_escape_attempts as f32 * ESCAPE_REDUCTION_PER_FAIL;

        (phase_base * rarity_mult * uncle_mult - failed_reduction)
            .max(MIN_ESCAPE_CHANCE)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MetabolicPhase {
    Burst,
    Stochastic,
    Fatigue,
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
pub struct CashOutAllButton;

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

#[derive(Component)]
pub struct UncleBasketDisplay;
