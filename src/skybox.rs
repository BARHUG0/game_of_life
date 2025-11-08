use raylib::prelude::*;

/// Trait for all skybox types to implement
pub trait SkyboxSample {
    fn sample(&self, ray_direction: &Vector3) -> Color;
}

/// Enum representing different skybox types
pub enum Skybox {
    Solid(SolidSkybox),
    Gradient(GradientSkybox),
    // Future variants:
    // Textured(TexturedSkybox),
}

impl Skybox {
    /// Sample the skybox color based on ray direction
    #[inline(always)]
    pub fn sample(&self, ray_direction: &Vector3) -> Color {
        match self {
            Skybox::Solid(skybox) => skybox.sample(ray_direction),
            Skybox::Gradient(skybox) => skybox.sample(ray_direction),
        }
    }
}

/// Simple solid color skybox
pub struct SolidSkybox {
    color: Color,
}

impl SolidSkybox {
    pub fn new(color: Color) -> Self {
        SolidSkybox { color }
    }
}

impl SkyboxSample for SolidSkybox {
    #[inline(always)]
    fn sample(&self, _ray_direction: &Vector3) -> Color {
        self.color
    }
}

pub struct GradientSkybox {
    zenith_color: Color,
    horizon_color: Color,
    ground_color: Color,

    // Sun properties
    sun_direction: Vector3,
    sun_color: Color,
    sun_size: f32,
    sun_intensity: f32,

    // Moon properties (NEW)
    moon_direction: Vector3,
    moon_color: Color,
    moon_size: f32,
    moon_intensity: f32,

    // Atmospheric effects
    halo_size: f32,
    halo_intensity: f32,

    stars_enabled: bool,
    stars_intensity: f32,
    stars_twinkle_speed: f32,
}

impl GradientSkybox {
    /// Create a new gradient skybox with sun
    ///
    /// # Arguments
    /// * `zenith_color` - Color at the top of the sky (y = +1)
    /// * `horizon_color` - Color at the horizon (y = 0)
    /// * `ground_color` - Color below the horizon (y = -1)
    /// * `sun_direction` - Normalized direction toward the sun
    /// * `sun_color` - Color of the sun disc
    /// * `sun_size` - Angular size in radians (0.01 = small, 0.1 = large)
    /// * `sun_intensity` - Brightness multiplier (1.0 = normal, higher = brighter)
    ///
    pub fn new(
        zenith_color: Color,
        horizon_color: Color,
        ground_color: Color,
        sun_direction: Vector3,
        sun_color: Color,
        sun_size: f32,
        sun_intensity: f32,
        halo_size: f32,      // NEW parameter
        halo_intensity: f32, // NEW parameter
    ) -> Self {
        GradientSkybox {
            zenith_color,
            horizon_color,
            ground_color,
            sun_direction: sun_direction.normalized(),
            sun_color,
            sun_size,
            sun_intensity,
            halo_size,                                    // NEW field
            halo_intensity,                               // NEW field
            moon_direction: Vector3::new(0.0, -1.0, 0.0), // Below horizon by default
            moon_color: Color::new(220, 220, 255, 255),   // Cool white
            moon_size: 0.018,
            moon_intensity: 0.0,
            stars_enabled: true,
            stars_intensity: 0.0, // Disabled by default for static skyboxes
            stars_twinkle_speed: 1.0,
        }
    }

    /// Create skybox from day/night cycle (use this for animated sky)
    pub fn from_cycle(
        zenith: Color,
        horizon: Color,
        ground: Color,
        sun_dir: Vector3,
        sun_color: Color,
        sun_size: f32,
        sun_intensity: f32,
        moon_dir: Vector3,
        moon_color: Color,
        moon_size: f32,
        moon_intensity: f32,
        halo_size: f32,
        halo_intensity: f32,
        stars_intensity: f32,
    ) -> Self {
        GradientSkybox {
            zenith_color: zenith,
            horizon_color: horizon,
            ground_color: ground,
            sun_direction: sun_dir.normalized(),
            sun_color,
            sun_size,
            sun_intensity,
            moon_direction: moon_dir.normalized(),
            moon_color,
            moon_size,
            moon_intensity,
            halo_size,
            halo_intensity,
            stars_enabled: true, // NEW field
            stars_intensity,     // NEW field
            stars_twinkle_speed: 1.0,
        }
    }

    /// Create a simple two-color gradient with sun
    pub fn simple(top_color: Color, bottom_color: Color, sun_direction: Vector3) -> Self {
        GradientSkybox {
            zenith_color: top_color,
            horizon_color: bottom_color,
            ground_color: bottom_color,
            sun_direction: sun_direction.normalized(),
            sun_color: Color::new(255, 255, 200, 255),
            sun_size: 0.02,
            sun_intensity: 8.0,
            halo_size: 0.3,                               // NEW: Reasonable default
            halo_intensity: 0.5,                          // NEW: Reasonable default
            moon_direction: Vector3::new(0.0, -1.0, 0.0), // Below horizon by default
            moon_color: Color::new(220, 220, 255, 255),   // Cool white
            moon_size: 0.018,
            moon_intensity: 0.0,
            stars_enabled: true,
            stars_intensity: 0.0, // Disabled by default for static skyboxes
            stars_twinkle_speed: 1.0,
        }
    }

    /// Create a default sky-like gradient with sun
    pub fn default_sky() -> Self {
        GradientSkybox {
            zenith_color: Color::new(20, 50, 150, 255),
            horizon_color: Color::new(100, 150, 200, 255),
            ground_color: Color::new(50, 40, 30, 255),
            sun_direction: Vector3::new(0.5, 0.8, 0.3).normalized(),
            sun_color: Color::new(255, 255, 220, 255),
            sun_size: 0.02,
            sun_intensity: 10.0,
            halo_size: 0.3,                               // NEW: Medium halo
            halo_intensity: 0.6,                          // NEW: Moderate intensity
            moon_direction: Vector3::new(0.0, -1.0, 0.0), // Below horizon by default
            moon_color: Color::new(220, 220, 255, 255),   // Cool white
            moon_size: 0.018,
            moon_intensity: 0.0,
            stars_enabled: true,
            stars_intensity: 0.0, // Disabled by default for static skyboxes
            stars_twinkle_speed: 1.0,
        }
    }

    /// Update sun position (useful for day/night cycle later)
    pub fn set_sun_direction(&mut self, direction: Vector3) {
        self.sun_direction = direction.normalized();
    }

    /// Create a skybox without a sun (sun_intensity = 0)
    pub fn no_sun(zenith_color: Color, horizon_color: Color, ground_color: Color) -> Self {
        GradientSkybox {
            zenith_color,
            horizon_color,
            ground_color,
            sun_direction: Vector3::new(0.0, 1.0, 0.0),
            sun_color: Color::WHITE,
            sun_size: 0.02,
            sun_intensity: 0.0,
            halo_size: 0.0,                               // NEW: No halo
            halo_intensity: 0.0,                          // NEW: No halo
            moon_direction: Vector3::new(0.0, -1.0, 0.0), // Below horizon by default
            moon_color: Color::new(220, 220, 255, 255),   // Cool white
            moon_size: 0.018,
            moon_intensity: 0.0,
            stars_enabled: true,
            stars_intensity: 0.0, // Disabled by default for static skyboxes
            stars_twinkle_speed: 1.0,
        }
    }
}

impl SkyboxSample for GradientSkybox {
    #[inline(always)]
    fn sample(&self, ray_direction: &Vector3) -> Color {
        // First, get the base gradient color
        let y = ray_direction.y;
        let mut base_color = if y >= 0.0 {
            let t = y;
            color_lerp(self.horizon_color, self.zenith_color, t)
        } else {
            let t = -y;
            color_lerp(self.horizon_color, self.ground_color, t)
        };

        // Check if we're looking at or near the sun
        if self.sun_intensity > 0.0 {
            let dot = ray_direction.dot(self.sun_direction);
            let sun_threshold = (self.sun_size * 0.5).cos();

            if dot >= sun_threshold {
                // === SUN DISC with LIMB DARKENING ===
                // Normalize position within sun disc (0 at edge, 1 at center)
                let sun_factor = (dot - sun_threshold) / (1.0 - sun_threshold);

                // Limb darkening: edges are darker, center is brighter
                // Using power of 0.7 creates subtle darkening at edges
                let limb_darkening = sun_factor.powf(0.7);

                // Apply limb darkening to sun intensity
                let sun_contrib =
                    color_multiply(self.sun_color, self.sun_intensity * limb_darkening);
                return color_add(base_color, sun_contrib);
            }

            // === OUTER GLOW (existing) ===
            let glow_size = self.sun_size * 3.0;
            let glow_threshold = (glow_size * 0.5).cos();

            if dot >= glow_threshold {
                let glow_factor =
                    ((dot - glow_threshold) / (sun_threshold - glow_threshold)).powf(2.0);
                let glow_intensity = self.sun_intensity * 0.3 * glow_factor;
                let glow_contrib = color_multiply(self.sun_color, glow_intensity);
                base_color = color_add(base_color, glow_contrib);
            }

            // === ATMOSPHERIC HALO ===
            if self.halo_intensity > 0.0 {
                let halo_threshold = (self.halo_size * 0.5).cos();

                if dot >= halo_threshold {
                    // Distance from halo edge (0) to glow edge (1)
                    let halo_factor = ((dot - halo_threshold) / (glow_threshold - halo_threshold))
                        .clamp(0.0, 1.0)
                        .powf(1.5);

                    // Warm atmospheric color (orange/yellow tint)
                    let halo_color = Color::new(255, 200, 150, 255);
                    let halo_strength = self.halo_intensity * halo_factor * 0.4;

                    // Blend halo color into base sky color
                    base_color = color_blend(base_color, halo_color, halo_strength);
                }
            }
        }

        if self.moon_intensity > 0.0 {
            let moon_dot = ray_direction.dot(self.moon_direction);
            let moon_threshold = (self.moon_size * 0.5).cos();

            if moon_dot >= moon_threshold {
                // Moon disc with limb darkening
                let moon_factor = (moon_dot - moon_threshold) / (1.0 - moon_threshold);
                let limb_darkening = moon_factor.powf(0.6); // Slightly different than sun

                let moon_contrib =
                    color_multiply(self.moon_color, self.moon_intensity * limb_darkening);
                base_color = color_add(base_color, moon_contrib);
            }

            // Subtle moon glow
            let moon_glow_size = self.moon_size * 2.5;
            let moon_glow_threshold = (moon_glow_size * 0.5).cos();

            if moon_dot >= moon_glow_threshold {
                let glow_factor = ((moon_dot - moon_glow_threshold)
                    / (moon_threshold - moon_glow_threshold))
                    .clamp(0.0, 1.0)
                    .powf(2.0);
                let glow_intensity = self.moon_intensity * 0.2 * glow_factor;
                let glow_contrib = color_multiply(self.moon_color, glow_intensity);
                base_color = color_add(base_color, glow_contrib);
            }
        }

        // === STAR RENDERING ===
        if self.stars_enabled && self.stars_intensity > 0.0 {
            // Only render stars when sun is well below horizon
            let sun_height = self.sun_direction.y;

            if sun_height < -0.1 {
                // Fade stars in/out based on sun position
                let star_fade = ((-sun_height - 0.1) / 0.4).clamp(0.0, 1.0);

                // Only compute stars in upper hemisphere (y > -0.2)
                if ray_direction.y > -0.2 {
                    let star_brightness = sample_star(ray_direction);

                    if star_brightness > 0.0 {
                        // Apply twinkling (using sun height as time proxy for now)
                        let time_factor = sun_height * 10.0; // Will be replaced with actual time
                        let final_brightness =
                            apply_star_twinkle(star_brightness, ray_direction, time_factor);

                        // Dim stars near horizon (atmospheric effect)
                        let horizon_fade = ((ray_direction.y + 0.2) / 0.4).clamp(0.0, 1.0);

                        let star_color = Color::new(255, 255, 255, 255);
                        let star_contrib = color_multiply(
                            star_color,
                            final_brightness * self.stars_intensity * star_fade * horizon_fade,
                        );
                        base_color = color_add(base_color, star_contrib);
                    }
                }
            }
        }

        base_color
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

/// Multiply a color by an intensity factor
#[inline(always)]
fn color_multiply(color: Color, intensity: f32) -> Color {
    Color::new(
        (color.r as f32 * intensity).min(255.0) as u8,
        (color.g as f32 * intensity).min(255.0) as u8,
        (color.b as f32 * intensity).min(255.0) as u8,
        255,
    )
}

/// Add two colors together (clamped)
#[inline(always)]
fn color_add(a: Color, b: Color) -> Color {
    Color::new(
        (a.r as u16 + b.r as u16).min(255) as u8,
        (a.g as u16 + b.g as u16).min(255) as u8,
        (a.b as u16 + b.b as u16).min(255) as u8,
        255,
    )
}

/// Blend two colors together with a weight factor
#[inline(always)]
fn color_blend(a: Color, b: Color, weight: f32) -> Color {
    let weight = weight.clamp(0.0, 1.0);
    let inv_weight = 1.0 - weight;
    Color::new(
        (a.r as f32 * inv_weight + b.r as f32 * weight) as u8,
        (a.g as f32 * inv_weight + b.g as f32 * weight) as u8,
        (a.b as f32 * inv_weight + b.b as f32 * weight) as u8,
        255,
    )
}

/// Generate procedural stars using hash-based approach
/// Returns star brightness (0.0 = no star, > 0.0 = star brightness)
#[inline(always)]
fn sample_star(ray_direction: &Vector3) -> f32 {
    // Simple hash function based on ray direction
    // We quantize the direction to create discrete "star positions"
    let scale = 500.0; // Controls star density (higher = more stars)
    let x = (ray_direction.x * scale) as i32;
    let y = (ray_direction.y * scale) as i32;
    let z = (ray_direction.z * scale) as i32;

    // Hash the quantized coordinates
    let mut hash = x.wrapping_mul(73856093);
    hash ^= y.wrapping_mul(19349663);
    hash ^= z.wrapping_mul(83492791);
    hash = hash.wrapping_mul(hash);

    // Early rejection: most of sky has no stars (99.5% rejection)
    if hash % 700 != 0 {
        return 0.0;
    }

    // If we passed early rejection, determine star properties
    let brightness_hash = hash.wrapping_mul(2147483647);

    // Star brightness variation (0.3 to 1.0)
    let brightness = 0.1 + ((brightness_hash % 1000) as f32 / 1000.0) * 0.6;

    // Occasional brighter stars (1% chance of 2x brightness)
    let is_bright = (brightness_hash % 100) == 0;
    if is_bright {
        brightness * 2.0
    } else {
        brightness
    }
}

/// Add twinkling effect to star brightness
#[inline(always)]
fn apply_star_twinkle(base_brightness: f32, ray_direction: &Vector3, time_factor: f32) -> f32 {
    if base_brightness < 0.01 {
        return 0.0;
    }

    // Use ray direction as seed for consistent twinkling per star
    let seed = ((ray_direction.x * 1000.0) as i32
        + (ray_direction.y * 2000.0) as i32
        + (ray_direction.z * 3000.0) as i32) as f32;

    // Simple sine-based twinkling
    let twinkle = ((seed + time_factor * 3.0).sin() * 0.5 + 0.5) * 0.3 + 0.7;

    base_brightness * twinkle
}
