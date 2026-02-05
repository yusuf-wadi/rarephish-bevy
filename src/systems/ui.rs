use bevy::prelude::*;
use crate::components::{
    FishCountText, GoldCountText, MultiplierText, SeedText,
    UncleSelectButton, NewWorldButton, Tile, Uncle,
};
use crate::resources::{GameState, WorldSeed, SelectedUncle};

/// Updates UI text displays based on current game state
pub fn update_ui_system(
    game_state: Res<GameState>,
    world_seed: Res<WorldSeed>,
    mut fish_text_q: Query<&mut Text, (With<FishCountText>, Without<GoldCountText>, Without<MultiplierText>, Without<SeedText>)>,
    mut gold_text_q: Query<&mut Text, (With<GoldCountText>, Without<FishCountText>, Without<MultiplierText>, Without<SeedText>)>,
    mut mult_text_q: Query<&mut Text, (With<MultiplierText>, Without<FishCountText>, Without<GoldCountText>, Without<SeedText>)>,
    mut seed_text_q: Query<&mut Text, (With<SeedText>, Without<FishCountText>, Without<GoldCountText>, Without<MultiplierText>)>,
) {
    // Update fish count
    if let Ok(mut text) = fish_text_q.get_single_mut() {
        text.sections[0].value = format!("{}", game_state.fish_count);
    }

    // Update gold count
    if let Ok(mut text) = gold_text_q.get_single_mut() {
        text.sections[0].value = format!("{}", game_state.gold);
    }

    // Update multiplier
    if let Ok(mut text) = mult_text_q.get_single_mut() {
        text.sections[0].value = format!("{:.1}x", game_state.multiplier);
    }

    // Update seed
    if let Ok(mut text) = seed_text_q.get_single_mut() {
        text.sections[0].value = format!("Seed: {}", world_seed.seed);
    }
}

/// Handles uncle type selection button clicks
pub fn handle_uncle_selection(
    interaction_q: Query<(&Interaction, &UncleSelectButton), Changed<Interaction>>,
    mut selected_uncle: ResMut<SelectedUncle>,
) {
    for (interaction, button) in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            selected_uncle.uncle_type = button.uncle_type;
        }
    }
}

/// Handles new world generation button click
pub fn handle_new_world(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<NewWorldButton>)>,
    mut commands: Commands,
    mut world_seed: ResMut<WorldSeed>,
    mut game_state: ResMut<GameState>,
    tiles_q: Query<Entity, With<Tile>>,
    uncles_q: Query<Entity, With<Uncle>>,
) {
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            // Generate new seed
            world_seed.new_seed();

            // Reset game state
            game_state.fish_count = 0;
            game_state.gold = 100;
            game_state.current_catch.clear();
            game_state.multiplier = 1.0;
            game_state.cash_out_cooldown = 0.0;

            // Despawn all tiles
            for entity in tiles_q.iter() {
                commands.entity(entity).despawn();
            }

            // Despawn all uncles
            for entity in uncles_q.iter() {
                commands.entity(entity).despawn();
            }

            // Note: Runtime world regeneration requires calling generate_tilemap
            // Consider adding an event-based system for full world reload
        }
    }
}
