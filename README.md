# 🎣 Rarephish (Bevy)

A seeded, procedurally generated idle fishing game built with the Bevy engine. You place uncles on land tiles near water, they auto-fish for you, and you decide when to cash out before rare fish get away.

---

## High-level overview

Rarephish is about **placement**, risk, and compounding value. You hire uncles from different regions, each with distinct abilities and costs, then position them around a procedurally generated map of land and water. They fish automatically over time, generating fish with different rarities, values, and escape chances. You choose when to harvest your catch via a cash-out system on cooldown, trading risk of escape for higher multipliers and long-term scaling.

---

## Core fantasy

- You are orchestrating a small **village** of uncles from Mongolia, Somalia, Japan, and beyond, each with their own fishing style and strengths.
- The world is seeded and reproducible, so each "run" feels like a coherent, generated landscape rather than random noise.
- Fish are tiny generative artifacts: color, pattern, size, and shape combine into named fish whose rarity translates into real economic impact.
- The idle layer is about optimizing layouts and timings; the active layer is about when to risk staying in the water for one more rare pull.

---

## Features

- **Seeded worlds**
  - Uses a `WorldSeed` resource backed by a `ChaCha8Rng` to generate deterministic worlds per seed.
  - Worlds can be regenerated with new seeds while keeping the same generation logic and probabilities.

- **Dynamic tilemap**
  - Grid of `TILE_WIDTH x TILE_HEIGHT` tiles (16×12) rendered as colored sprites in world space.
  - Each tile is either `Water` or `Land`, with water clustered around the center via both noise and distance checks.
  - Tile entities carry a `Tile { x, y, tile_type }` component for pure ECS interactions.

- **Uncle types (workers)**
  - `Mongolian Uncle`: basic ability, ~2000 ms fishing speed, cost 50 gold.
  - `Somali Uncle`: fast ability, ~1500 ms speed, cost 150 gold.
  - `Japanese Uncle`: "rare finder", ~2500 ms speed with bonus rare-fish chance, cost 300 gold.
  - All uncle types expose helpers for cost, sprite, display name, ability label, and rare-chance bonus.

- **Placement rules**
  - Uncles can only be placed on `Land` tiles that are adjacent to at least one `Water` tile.
  - Placement consumes gold according to the uncle type's cost and attaches an `Uncle` component at a tile's grid position.

- **Procedural fish generation**
  - Fish names are built from parts: `color + pattern + shape + "fish"` (e.g. "Blue Striped Slimfish").
  - Uses configured lists of colors, patterns, sizes, and shapes in `constants.rs`.
  - Rarities: `Common`, `Uncommon`, `Rare`, each with its own base drop probability and gold value ranges.

- **Escape mechanics**
  - Each rarity has its own escape chance:
    - Common: ~5%
    - Uncommon: ~20%
    - Rare: ~40%
  - Rare-finder uncles get a small bonus to hitting rare thresholds without changing escape probabilities, keeping them high-risk/high-reward.

- **Economy & risk systems**
  - `GameState` tracks `fish_count`, `gold`, `current_catch`, `multiplier`, and a `cash_out_cooldown` timer.
  - Starting gold is 100, letting you immediately hire at least a Mongolian uncle.
  - Cashing out converts all fish in `current_catch` into gold and increases a global multiplier up to a max.
  - Cash out is locked behind a 30-second cooldown to force risk management decisions.

- **Bevy-native UX**
  - Uses `Camera2dBundle` plus Bevy UI nodes for header stats (Fish, Gold, Multiplier, Seed).
  - Text markers: `FishCountText`, `GoldCountText`, `MultiplierText`, `SeedText` components for clean UI system updates.
  - Systems are grouped in `systems/` by intent: setup, tilemap generation, gameplay, UI.

---

## Game loop

1. **Generate world**
   - On startup, `WorldSeed` is initialized with a time-based seed and used by `generate_tilemap` to spawn water and land tiles.
   - The seed is displayed in the UI so you can note or share specific worlds.

2. **Select an uncle type**
   - Selection is stored in a `SelectedUncle` resource containing an `UncleType` enum.
   - Currently defaults to Mongolian uncle (UI selection buttons to be added).

3. **Place uncles on the map**
   - Clicking a valid land tile that is adjacent to water attempts to place the currently selected uncle.
   - The placement system checks:
     - Is the tile land?
     - Is it unoccupied?
     - Is it near water?
     - Do you have enough gold to pay the uncle's cost?
   - If valid, gold is reduced, and an `Uncle` is spawned at that tile's position with an emoji sprite.

4. **Auto fishing**
   - Each uncle has an internal timer derived from `uncle_type.speed_ms()`.
   - `uncle_fishing_system` ticks these timers; when a timer completes, it rolls to generate a fish for that uncle.
   - Fish rarity is rolled with base probabilities plus the Japanese uncle's rare bonus, then a name and value range are chosen from constants.

5. **Fish escape roll**
   - After a fish is generated, `fish_escape_system` evaluates an escape roll using rarity-specific escape chances.
   - If the fish escapes, it's removed from `current_catch`, preserving the risk tension around chasing rare fish.
   - If it stays, it remains in `current_catch` with its name, rarity, and value.

6. **Cash out**
   - `cash_out_system` is triggered by pressing **Space** key, gated by `cash_out_cooldown` and presence of fish.
   - On success:
     - Sums all `value`s in `current_catch` into `gold`.
     - Clears `current_catch`.
     - Increases `multiplier` by a configured increment up to a maximum (e.g. `+0.1` up to `5.0x`).
     - Resets `cash_out_cooldown` to 30 seconds.

7. **Cooldown & progression**
   - `cooldown_update_system` decrements the cooldown every frame using `Time`, clamping at zero.
   - As your economy grows, you can afford faster or rarer-leaning uncles, gradually transforming the character of each run.

---

## Systems and modules

### File layout

```text
src/
  main.rs         # Bevy app wiring and system schedule
  components.rs   # ECS components for tiles, uncles, fish, UI
  resources.rs    # GameState, WorldSeed, SelectedUncle
  constants.rs    # Gameplay tuning and generation constants
  systems/
    mod.rs        # System module exports
    setup.rs      # Camera + root UI setup
    tilemap.rs    # Seeded tilemap generation & adjacency helpers
    gameplay.rs   # Uncle placement, fishing, escape, cash out
    ui.rs         # UI updates & interactions
```

- `main.rs` wires up resources, startup systems, and update systems into a clean schedule, separating setup, tilemap generation, gameplay, and UI concerns.
- `components.rs` defines pure data types for Tile, Uncle, Fish, and UI markers, keeping logic out of components.
- `resources.rs` centralizes long-lived game state (economy and RNG), ready for serialization/saving later.
- `constants.rs` is the single source of truth for tuning gameplay numbers and lists used in fish generation.
- `systems/` keeps each behavior domain in a focused module, so you can iterate without touching unrelated code.

### Architecture notes

- **ECS-first design**
  - Tiles, uncles, fish, and UI elements are all entities with focused components.
  - Systems operate on queries or resources, making it easy to parallelize or extend gameplay over time.

- **Deterministic random**
  - World generation and fish attributes use the WorldSeed resource with a seeded ChaCha8Rng, making runs reproducible given the same seed.
  - You can evolve the generator to support chunked/tiled world expansion without changing the core contract.

- **UI decoupling**
  - No direct game logic lives in UI hierarchies; UI is updated via marker components and dedicated systems in ui.rs.
  - This keeps the design close to a clean Apple-style separation between model and presentation.

- **Extendability**
  - New uncle types: add a variant to UncleType, define its speed/cost/bonus, and the rest of the pipeline works automatically.
  - New fish attributes: extend the FISH_* arrays and adjust rarity/value curves in constants.rs.
  - New world biomes: derive additional TileTypes or overlays and enhance generate_tilemap with different patterns per region.

---

## Running the game

### Prerequisites

- Rust (via rustup) installed and up to date.
- A system that supports Bevy 2D rendering.

### Clone and build

```bash
git clone https://github.com/yusuf-wadi/rarephish-bevy.git
cd rarephish-bevy

# First build (will fetch Bevy + deps)
cargo build
```

### Run dev build

```bash
cargo run
```

This runs a development build with faster compile times and decent runtime performance.

### Run optimized build

```bash
cargo run --release
```

Use this if you're profiling, stress-testing a lot of entities, or preparing a build for distribution.

### Optional: fast iteration

Install cargo-watch once:

```bash
cargo install cargo-watch
```

Then in the project directory:

```bash
cargo watch -x run
```

This will rebuild and relaunch the game whenever you change code.

---

## Controls

- **Left Click** - Place the currently selected uncle on a valid land tile (must be adjacent to water)
- **Space** - Cash out your current catch (only when cooldown is 0 and you have fish)

---

## Design and future directions

The current Bevy build captures the core loop of the original Rarephish concept—different uncles, procedural fish, rarity-driven value, and fish-to-gold exchange—while shifting from a button-clicker to a spatial, tile-based strategy layer. From here you can naturally extend into:

- World tiling and plenoptree-style infinite maps (chunked tile generation around active regions)
- Deeper uncle traits (gear, fatigue, synergies, location affinities)
- Fish effects (temporary buffs, global modifiers, rare event triggers)
- Progression layers (meta-upgrades, relics, different world archetypes keyed by seed ranges)
- Full UI implementation (uncle selection sidebar, current catch display, cash out button)
- Save/load system with persistent world seeds
- Audio and visual polish (animations, particles, sound effects)

---

## License

MIT
