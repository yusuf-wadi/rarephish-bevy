use bevy::prelude::*;
use crate::resources::DayNightCycle;
use crate::components::{DayNumberText, TimeOfDayText, CashoutsRemainingText};
use crate::constants::*;

/// Updates the day/night cycle progression
pub fn day_night_cycle_system(
    mut day_night: ResMut<DayNightCycle>,
    time: Res<Time>,
) {
    day_night.time_elapsed += time.delta_seconds();

    // Calculate progress through day (0.0 to 1.0)
    day_night.day_progress = (day_night.time_elapsed / DAY_LENGTH_SECONDS) % 1.0;

    // Check if new day started
    if day_night.time_elapsed >= DAY_LENGTH_SECONDS {
        day_night.time_elapsed = 0.0;
        day_night.new_day();
        println!("☀️ Day {} begins! Cash-outs refreshed: {}", day_night.day_number, day_night.cashouts_remaining);
    }

    // Update day/night state
    let was_day = day_night.is_day;
    day_night.is_day = day_night.is_daytime();

    // Transition events
    if !was_day && day_night.is_day {
        println!("🌅 Dawn - daytime begins");
    } else if was_day && !day_night.is_day {
        println!("🌆 Dusk - nighttime begins");
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

    // Update time of day with color
    if let Ok(mut text) = time_text_q.get_single_mut() {
        text.sections[0].value = format!("{} | {}", cycle.time_of_day_text(), cycle.time_string());
        text.sections[0].style.color = cycle.time_of_day_color();
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
