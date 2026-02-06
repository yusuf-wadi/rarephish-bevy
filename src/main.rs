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
        // Update systems - Gameplay
        .add_systems(Update, (
            gameplay::handle_uncle_placement,
            gameplay::handle_uncle_selection_world,
            gameplay::uncle_fishing_system,
            gameplay::fish_escape_system,
            gameplay::cash_out_selected_uncle,
            gameplay::cash_out_all_uncles,
            gameplay::cooldown_update_system,
        ))
        // Update systems - UI
        .add_systems(Update, (
            ui::update_ui_system,
            ui::update_basket_display,
            ui::update_basket_value_display,
            ui::handle_uncle_selection,
            ui::handle_cash_out_button,
            ui::handle_cash_out_all_button,
            ui::cash_out_button_visual,
            ui::cash_out_all_button_visual,
            ui::uncle_button_visual,
            ui::handle_new_world,
        ))
        .run();
}
