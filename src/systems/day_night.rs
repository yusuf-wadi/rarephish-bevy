use bevy::prelude::*;
use crate::resources::DayNightCycle;
use crate::constants::*;

/// Updates the day/night cycle timer and triggers new day events
pub fn update_day_night_cycle(
    mut cycle: ResMut<DayNightCycle>,
    time: Res<Time>,
) {
    cycle.time_elapsed += time.delta_seconds();

    // Calculate progress through the day (0.0 to 1.0)
    cycle.day_progress = (cycle.time_elapsed / DAY_LENGTH_SECONDS) % 1.0;

    // Update day/night boolean
    let was_day = cycle.is_day;
    cycle.is_day = cycle.is_daytime();

    // Detect when full day completes (crosses midnight)
    if cycle.time_elapsed >= DAY_LENGTH_SECONDS {
        cycle.new_day();
        cycle.time_elapsed = 0.0;
        println!("☀️ Day {} begins! Cash-outs refreshed: {}", 
                 cycle.day_number, 
                 cycle.cashouts_remaining);
    }

    // Log day/night transitions
    if was_day != cycle.is_day {
        if cycle.is_day {
            println!("🌅 Dawn breaks - daytime begins");
        } else {
            println!("🌙 Dusk falls - nighttime begins");
        }
    }
}

/// Visual feedback for time of day through background color
pub fn update_time_visual(
    cycle: Res<DayNightCycle>,
    mut clear_color: ResMut<ClearColor>,
) {
    // Smooth color transitions based on time of day
    let color = if cycle.day_progress < 0.25 {  // Night (midnight to 6 AM)
        let t = cycle.day_progress / 0.25;
        Color::srgb(
            0.05 + t * 0.15,  // 0.05 -> 0.20 (dark to dawn)
            0.05 + t * 0.20,  // 0.05 -> 0.25
            0.15 + t * 0.25,  // 0.15 -> 0.40
        )
    } else if cycle.day_progress < 0.5 {  // Morning (6 AM to noon)
        let t = (cycle.day_progress - 0.25) / 0.25;
        Color::srgb(
            0.20 + t * 0.35,  // 0.20 -> 0.55 (dawn to bright)
            0.25 + t * 0.45,  // 0.25 -> 0.70
            0.40 + t * 0.45,  // 0.40 -> 0.85
        )
    } else if cycle.day_progress < 0.75 {  // Afternoon (noon to 6 PM)
        let t = (cycle.day_progress - 0.5) / 0.25;
        Color::srgb(
            0.55 - t * 0.25,  // 0.55 -> 0.30 (bright to dusk)
            0.70 - t * 0.35,  // 0.70 -> 0.35
            0.85 - t * 0.45,  // 0.85 -> 0.40
        )
    } else {  // Evening (6 PM to midnight)
        let t = (cycle.day_progress - 0.75) / 0.25;
        Color::srgb(
            0.30 - t * 0.25,  // 0.30 -> 0.05 (dusk to night)
            0.35 - t * 0.30,  // 0.35 -> 0.05
            0.40 - t * 0.25,  // 0.40 -> 0.15
        )
    };

    clear_color.0 = color;
}

/// System to prevent cashouts when limit is reached
pub fn check_cashout_availability(
    cycle: Res<DayNightCycle>,
) -> bool {
    cycle.cashouts_remaining > 0
}
