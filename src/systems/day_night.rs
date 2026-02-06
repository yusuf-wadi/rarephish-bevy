use bevy::prelude::*;
use crate::resources::DayNightCycle;
use crate::components::{DayNumberText, TimeOfDayText, CashoutsRemainingText};
use crate::constants::*;

/// Updates the day/night cycle progression
pub fn day_night_cycle_system(
    mut cycle: ResMut<DayNightCycle>,
    time: Res<Time>,
) {
    cycle.time_elapsed += time.delta_seconds();

    // Calculate day progress (0.0 to 1.0)
    cycle.day_progress = (cycle.time_elapsed % DAY_LENGTH_SECONDS) / DAY_LENGTH_SECONDS;

    // Check if we passed midnight (new day)
    let was_day = cycle.is_day;
    cycle.is_day = cycle.is_daytime();

    // New day trigger: when we transition from night to day
    if !was_day && cycle.is_day && cycle.time_elapsed > DAY_LENGTH_SECONDS {
        cycle.new_day();
    }
}

/// Updates day/night cycle UI elements
pub fn update_day_night_ui(
    cycle: Res<DayNightCycle>,
    mut day_text_q: Query<&mut Text, (With<DayNumberText>, Without<TimeOfDayText>, Without<CashoutsRemainingText>)>,
    mut time_text_q: Query<&mut Text, (With<TimeOfDayText>, Without<DayNumberText>, Without<CashoutsRemainingText>)>,
    mut cashouts_text_q: Query<&mut Text, (With<CashoutsRemainingText>, Without<DayNumberText>, Without<TimeOfDayText>)>,
) {
    // Update day number
    if let Ok(mut text) = day_text_q.get_single_mut() {
        text.sections[0].value = format!("Day {}", cycle.day_number);
    }

    // Update time of day
    if let Ok(mut text) = time_text_q.get_single_mut() {
        text.sections[0].value = format!("{} {}", cycle.time_emoji(), cycle.time_string());
    }

    // Update cashouts remaining
    if let Ok(mut text) = cashouts_text_q.get_single_mut() {
        let color = if cycle.cashouts_remaining > 0 {
            Color::srgb(0.13, 0.77, 0.37) // Green
        } else {
            Color::srgb(0.9, 0.4, 0.4) // Red
        };
        text.sections[0].value = format!("Cashouts: {}/{}", cycle.cashouts_remaining, cycle.max_cashouts_per_day);
        text.sections[0].style.color = color;
    }
}
