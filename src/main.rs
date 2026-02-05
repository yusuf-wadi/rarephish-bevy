use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod systems;

use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🎣 Rare Fish Game".into(),
                resolution: (1400.0, 900.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Resources
        .init_resource::<GameState>()
        .init_resource::<WorldSeed>()
        .init_resource::<SelectedUncle>()
        // Startup systems
        .add_systems(Startup, (setup::setup_camera, setup::setup_ui))
        .add_systems(Startup, tilemap::generate_tilemap)
        // Update systems
        .add_systems(Update, (
            gameplay::handle_uncle_placement,
            gameplay::uncle_fishing_system,
            gameplay::fish_escape_system,
            gameplay::cash_out_system,
            gameplay::cooldown_update_system,
            ui::update_ui_system,
            ui::handle_uncle_selection,
            ui::handle_new_world,
        ))
        .run();
}
