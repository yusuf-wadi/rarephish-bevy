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
    Mongolian,  // Basic: 2s, 50g
    Somali,     // Fast: 1.5s, 150g
    Japanese,   // RareFinder: 2.5s, 300g, bonus rare
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
            UncleType::Mongolian => "Basic",
            UncleType::Somali => "Fast",
            UncleType::Japanese => "Rare Finder",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UncleType::Mongolian => "Steady and reliable fisherman",
            UncleType::Somali => "Quick catches, fast turnaround",
            UncleType::Japanese => "Higher chance for rare fish",
        }
    }

    pub fn rare_bonus(&self) -> f32 {
        match self {
            UncleType::Japanese => 0.05,
            _ => 0.0,
        }
    }
}

/// Component for fish entities
#[derive(Component)]
pub struct Fish {
    pub name: String,
    pub rarity: FishRarity,
    pub value: u32,
    pub escape_chance: f32,
    pub lifetime: Timer,
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
