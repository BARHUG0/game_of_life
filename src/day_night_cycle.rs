use raylib::prelude::*;
use std::f32::consts::PI;

pub struct DayNightCycle {
    time: f32,           // 0.0 to 1.0 (0.0 = midnight, 0.5 = noon, 1.0 = midnight again)
    cycle_duration: f32, // Total seconds for one full day/night cycle
    time_speed: f32,     // Speed multiplier (1.0 = normal, 2.0 = 2x, -1.0 = reverse)
    paused: bool,        // Whether time is paused
}

impl DayNightCycle {
    /// Create a new day/night cycle
    ///
    /// # Arguments
    /// * `cycle_duration` - How many seconds for a complete cycle (e.g., 30.0)
    /// * `starting_time` - Initial time (0.0 = midnight, 0.25 = sunrise, 0.5 = noon, 0.75 = sunset)
    pub fn new(cycle_duration: f32, starting_time: f32) -> Self {
        DayNightCycle {
            time: starting_time.clamp(0.0, 1.0),
            cycle_duration,
            time_speed: 1.0,
            paused: false,
        }
    }

    /// Create cycle starting at noon
    pub fn starting_at_noon(cycle_duration: f32) -> Self {
        Self::new(cycle_duration, 0.5)
    }

    /// Create cycle starting at sunrise
    pub fn starting_at_sunrise(cycle_duration: f32) -> Self {
        Self::new(cycle_duration, 0.25)
    }

    /// Update the cycle based on elapsed time
    pub fn update(&mut self, delta_time: f32) {
        if !self.paused {
            // Advance time based on speed
            self.time += (delta_time / self.cycle_duration) * self.time_speed;

            // Wrap time to stay in 0.0..1.0 range
            self.time = self.time.rem_euclid(1.0);
        }
    }

    pub fn time_speed(&self) -> f32 {
        self.time_speed
    }

    /// Get current time of day (0.0 to 1.0)
    pub fn get_time(&self) -> f32 {
        self.time
    }

    /// Set time directly (for manual control)
    pub fn set_time(&mut self, time: f32) {
        self.time = time.clamp(0.0, 1.0);
    }

    /// Toggle pause
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Increase time speed
    pub fn speed_up(&mut self) {
        self.time_speed *= 1.5;
        self.time_speed = self.time_speed.clamp(-10.0, 10.0);
    }

    /// Decrease time speed
    pub fn slow_down(&mut self) {
        self.time_speed /= 1.5;
        if self.time_speed.abs() < 0.1 {
            self.time_speed = 0.1 * self.time_speed.signum();
        }
    }

    /// Reverse time direction
    pub fn reverse(&mut self) {
        self.time_speed = -self.time_speed;
    }

    /// Get sun direction based on realistic arc
    /// Sun rises in the east (+X), peaks in the south (-Z), sets in the west (-X)
    pub fn get_sun_direction(&self) -> Vector3 {
        // Convert time to angle (0 = sunrise east, 0.5 = noon south, 1.0 = sunrise east again)
        let angle = self.time * 2.0 * PI;

        // Sun travels in an arc from east to west
        // X: east-west movement (cos gives us east->west->east)
        // Y: height (sin gives us below->above->below horizon)
        // Z: north-south (sun peaks slightly south for realism)

        let x = angle.cos(); // East to west
        let y = angle.sin(); // Height above/below horizon
        let z = -0.3; // Slightly south for visual variety

        Vector3::new(x, y, z).normalized()
    }

    /// Get moon direction (opposite side of sky from sun)
    pub fn get_moon_direction(&self) -> Vector3 {
        let sun_dir = self.get_sun_direction();
        // Moon is roughly opposite the sun
        Vector3::new(-sun_dir.x, -sun_dir.y, sun_dir.z).normalized()
    }

    /// Get sky colors based on time of day
    /// Returns (zenith_color, horizon_color, ground_color)
    pub fn get_sky_colors(&self) -> (Color, Color, Color) {
        let sun_dir = self.get_sun_direction();
        let sun_height = sun_dir.y; // -1.0 (below) to +1.0 (above)

        // Define colors for different times of day
        let day_zenith = Color::new(20, 50, 150, 255); // Deep blue
        let day_horizon = Color::new(100, 150, 200, 255); // Light blue
        let day_ground = Color::new(50, 40, 30, 255); // Brown

        let sunset_zenith = Color::new(40, 20, 80, 255); // Purple
        let sunset_horizon = Color::new(255, 140, 60, 255); // Orange
        let sunset_ground = Color::new(30, 20, 20, 255); // Dark

        let night_zenith = Color::new(5, 5, 20, 255); // Very dark blue
        let night_horizon = Color::new(20, 20, 40, 255); // Dark blue-gray
        let night_ground = Color::new(5, 5, 5, 255); // Almost black

        // Transition based on sun height
        if sun_height > 0.3 {
            // Full day (sun high in sky)
            (day_zenith, day_horizon, day_ground)
        } else if sun_height > -0.1 {
            // Sunset/sunrise transition (sun near horizon)
            let t = (sun_height + 0.1) / 0.4; // 0.0 at horizon, 1.0 at full day

            (
                color_lerp(sunset_zenith, day_zenith, t),
                color_lerp(sunset_horizon, day_horizon, t),
                color_lerp(sunset_ground, day_ground, t),
            )
        } else if sun_height > -0.5 {
            // Twilight (sun just below horizon)
            let t = (sun_height + 0.5) / 0.4; // 0.0 at deep night, 1.0 at horizon

            (
                color_lerp(night_zenith, sunset_zenith, t),
                color_lerp(night_horizon, sunset_horizon, t),
                color_lerp(night_ground, sunset_ground, t),
            )
        } else {
            // Full night (sun well below horizon)
            (night_zenith, night_horizon, night_ground)
        }
    }

    /// Get sun properties (color, intensity)
    /// Returns (color, intensity, size)
    pub fn get_sun_properties(&self) -> (Color, f32, f32) {
        let sun_dir = self.get_sun_direction();
        let sun_height = sun_dir.y;

        if sun_height > 0.0 {
            // Sun is visible above horizon
            let base_color = Color::new(255, 255, 220, 255); // Warm white
            let sunset_color = Color::new(255, 150, 80, 255); // Orange

            if sun_height > 0.3 {
                // High sun - bright white
                (base_color, 12.0, 0.02)
            } else {
                // Low sun - transition to orange
                let t = sun_height / 0.3;
                let color = color_lerp(sunset_color, base_color, t);
                let intensity = 8.0 + 4.0 * t; // Dimmer at sunset
                (color, intensity, 0.025) // Slightly larger near horizon
            }
        } else {
            // Sun below horizon - not visible
            (Color::new(255, 255, 220, 255), 0.0, 0.02)
        }
    }

    /// Get moon properties (color, intensity)
    /// Returns (color, intensity, size)
    pub fn get_moon_properties(&self) -> (Color, f32, f32) {
        let moon_dir = self.get_moon_direction();
        let moon_height = moon_dir.y;

        if moon_height > 0.0 {
            // Moon is visible above horizon
            let moon_color = Color::new(220, 220, 255, 255); // Cool white
            let intensity = 3.0 * moon_height.powf(0.5); // Brighter when higher
            (moon_color, intensity, 0.018) // Slightly smaller than sun
        } else {
            // Moon below horizon - not visible
            (Color::new(220, 220, 255, 255), 0.0, 0.018)
        }
    }

    /// Get atmospheric halo properties
    /// Returns (size, intensity)
    pub fn get_halo_properties(&self) -> (f32, f32) {
        let sun_dir = self.get_sun_direction();
        let sun_height = sun_dir.y;

        if sun_height > 0.0 {
            // Stronger halo at sunset/sunrise
            let base_size = 0.3;
            let base_intensity = if sun_height < 0.3 {
                // Sunset - stronger halo
                0.8
            } else {
                // Midday - moderate halo
                0.5
            };
            (base_size, base_intensity)
        } else {
            // No halo at night
            (0.3, 0.0)
        }
    }

    /// Get star intensity (0.0 = no stars, 1.0 = full stars)
    pub fn get_star_intensity(&self) -> f32 {
        let sun_height = self.get_sun_direction().y;

        if sun_height < -0.1 {
            // Stars visible when sun is below horizon
            // Fade in from -0.1 to -0.5 sun height
            let fade = ((-sun_height - 0.1) / 0.4).clamp(0.0, 1.0);
            fade * 1.5 // Base star intensity
        } else {
            0.0 // No stars during day
        }
    }
}

/// Linear interpolation between two colors
#[inline(always)]
fn color_lerp(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        (a.r as f32 * (1.0 - t) + b.r as f32 * t) as u8,
        (a.g as f32 * (1.0 - t) + b.g as f32 * t) as u8,
        (a.b as f32 * (1.0 - t) + b.b as f32 * t) as u8,
        255,
    )
}
