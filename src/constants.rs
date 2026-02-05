// Tilemap dimensions
pub const TILE_WIDTH: usize = 16;
pub const TILE_HEIGHT: usize = 12;
pub const TILE_SIZE: f32 = 40.0;

// Uncle sprite dimensions
pub const UNCLE_SPRITE_SIZE: f32 = 32.0;

// Game balance
pub const STARTING_GOLD: u32 = 100;
pub const CASH_OUT_COOLDOWN: f32 = 30.0; // seconds
pub const MULTIPLIER_INCREMENT: f32 = 0.1;
pub const MAX_MULTIPLIER: f32 = 5.0;

// Fish rarity probabilities (before bonuses)
pub const RARE_CHANCE: f32 = 0.05;      // 5%
pub const UNCOMMON_CHANCE: f32 = 0.20;  // 20%
// Common is remainder: 75%

// Fish escape chances by rarity
pub const COMMON_ESCAPE: f32 = 0.05;     // 5%
pub const UNCOMMON_ESCAPE: f32 = 0.20;   // 20%
pub const RARE_ESCAPE: f32 = 0.40;       // 40%

// Fish value ranges
pub const COMMON_VALUE_MIN: u32 = 1;
pub const COMMON_VALUE_MAX: u32 = 10;
pub const UNCOMMON_VALUE_MIN: u32 = 15;
pub const UNCOMMON_VALUE_MAX: u32 = 40;
pub const RARE_VALUE_MIN: u32 = 50;
pub const RARE_VALUE_MAX: u32 = 100;

// Fish generation
pub const FISH_COLORS: &[&str] = &["Blue", "Red", "Green", "Yellow", "Purple", "Orange", "Pink", "Teal"];
pub const FISH_PATTERNS: &[&str] = &["Striped", "Spotted", "Solid", "Marbled", "Gradient"];
pub const FISH_SIZES: &[&str] = &["Tiny", "Small", "Medium", "Large", "Huge"];
pub const FISH_SHAPES: &[&str] = &["Slim", "Round", "Flat", "Long", "Bulky"];

// Uncle costs (also in components.rs, but here for reference)
pub const MONGOLIAN_COST: u32 = 50;
pub const SOMALI_COST: u32 = 150;
pub const JAPANESE_COST: u32 = 300;

// World generation
pub const WATER_THRESHOLD: f32 = 0.3;
pub const CENTER_WATER_RADIUS: f32 = 3.0;
