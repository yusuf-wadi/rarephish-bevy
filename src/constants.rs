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

// === Fish Escape Physics Model ===
// Based on biased random walk research for fish flopping mechanics

// Metabolic Phases (time-based)
pub const BURST_PHASE_DURATION: f32 = 10.0;      // 0-10 seconds: high intensity
pub const STOCHASTIC_PHASE_DURATION: f32 = 20.0; // 10-30 seconds: lactic buildup
pub const FATIGUE_PHASE_START: f32 = 30.0;       // 30+ seconds: exhaustion

// Base escape probabilities per phase (per second, not per frame)
// These represent "flop frequency" × "success chance per flop"
pub const BURST_ESCAPE_BASE: f32 = 0.15;       // 15% per second (3-5 Hz flops)
pub const STOCHASTIC_ESCAPE_BASE: f32 = 0.08;  // 8% per second (1-2 Hz flops)
pub const FATIGUE_ESCAPE_BASE: f32 = 0.02;     // 2% per second (<0.5 Hz flops)

// Escape reduction per failed attempt (represents building "distance to safety")
// Each failed flop moves fish slightly away or at wrong angle
pub const ESCAPE_REDUCTION_PER_FAIL: f32 = 0.02; // -2% per failed attempt
pub const MIN_ESCAPE_CHANCE: f32 = 0.001;        // Never goes below 0.1%

// Rarity modifiers (rarer fish are stronger/more desperate)
pub const COMMON_ESCAPE_MULTIPLIER: f32 = 0.6;    // 60% of base
pub const UNCOMMON_ESCAPE_MULTIPLIER: f32 = 1.0;  // 100% of base  
pub const RARE_ESCAPE_MULTIPLIER: f32 = 1.4;      // 140% of base (more vigorous)

// Uncle retention abilities (reduce escape chance)
pub const MONGOLIAN_RETENTION: f32 = 1.0;  // No bonus (base)
pub const SOMALI_RETENTION: f32 = 0.9;     // 10% better retention (faster response)
pub const JAPANESE_RETENTION: f32 = 1.1;   // 10% worse (focused on rarity, not retention)

// Fish value ranges
pub const COMMON_VALUE_MIN: u32 = 1;
pub const COMMON_VALUE_MAX: u32 = 8;
pub const UNCOMMON_VALUE_MIN: u32 = 10;
pub const UNCOMMON_VALUE_MAX: u32 = 24;
pub const RARE_VALUE_MIN: u32 = 30;
pub const RARE_VALUE_MAX: u32 = 100;

// Fish generation
pub const FISH_COLORS: &[&str] = &["Blue", "Red", "Green", "Yellow", "Purple", "Orange", "Pink", "Teal"];
pub const FISH_PATTERNS: &[&str] = &["Striped", "Spotted", "Solid", "Marbled", "Gradient"];
pub const FISH_SIZES: &[&str] = &["Tiny", "Small", "Medium", "Large", "Huge"];
pub const FISH_SHAPES: &[&str] = &["Slim", "Round", "Flat", "Long", "Bulky"];

// Uncle costs (also in components.rs, but here for reference)
pub const MONGOLIAN_COST: u32 = 100;
pub const SOMALI_COST: u32 = 300;
pub const JAPANESE_COST: u32 = 500;

// World generation
pub const WATER_THRESHOLD: f32 = 0.3;
pub const CENTER_WATER_RADIUS: f32 = 3.0;
