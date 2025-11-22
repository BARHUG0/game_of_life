use crate::orbital_math::{calculate_orbital_position, generate_orbital_path};
use crate::shader::ShaderType;
use raylib::prelude::*;

#[derive(Clone)]
pub enum CentralBody {
    BlackHole {
        position: Vector3,
        scale: f32,
    },
    Star {
        position: Vector3,
        scale: f32,
        shader: ShaderType,
    },
    Binary {
        separation: f32,
        angle: f32,
        scale: f32,
        shader: ShaderType,
    },
}

impl CentralBody {
    pub fn position(&self) -> Vector3 {
        match self {
            CentralBody::BlackHole { position, .. } => *position,
            CentralBody::Star { position, .. } => *position,
            CentralBody::Binary { .. } => Vector3::zero(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let CentralBody::Binary { angle, .. } = self {
            *angle += dt * 0.3;
        }
    }
}

#[derive(Clone)]
pub struct OrbitalParams {
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub inclination: f32,
    pub mean_motion: f32,  // NEW: radians per second (replaces rotation_speed)
    pub mean_anomaly: f32, // NEW: This is what increases linearly (replaces current_angle)
}

#[derive(Clone)]
pub struct Planet {
    pub orbital_params: OrbitalParams,
    pub scale: f32,
    pub shader: ShaderType,
    pub rotation: Vector3,
    pub trail_positions: Vec<Vector3>,
    pub trail_max_length: usize,
}

impl Planet {
    /// Get current position using centralized Keplerian orbital math
    /// Now properly uses mean anomaly!
    pub fn calculate_position(&self, center: Vector3) -> Vector3 {
        calculate_orbital_position(
            center,
            self.orbital_params.semi_major_axis,
            self.orbital_params.eccentricity,
            self.orbital_params.inclination,
            self.orbital_params.mean_anomaly, // This is the mean anomaly now!
        )
    }

    pub fn update(&mut self, dt: f32, center: Vector3) {
        // Update mean anomaly linearly (this is correct for Keplerian orbits!)
        self.orbital_params.mean_anomaly += self.orbital_params.mean_motion * dt;

        // Also update visual rotation
        self.rotation.y += dt * 0.5;

        // Calculate current position
        let current_pos = self.calculate_position(center);

        // Only add trail point if moved far enough (distance-based sampling)
        let should_add = if let Some(last_pos) = self.trail_positions.last() {
            let dx = current_pos.x - last_pos.x;
            let dy = current_pos.y - last_pos.y;
            let dz = current_pos.z - last_pos.z;
            let dist_squared = dx * dx + dy * dy + dz * dz;
            dist_squared > 0.0025 // Threshold: 0.05^2 (avoids sqrt)
        } else {
            true // Always add first point
        };

        if should_add {
            self.trail_positions.push(current_pos);

            // Trim old trail points
            if self.trail_positions.len() > self.trail_max_length {
                self.trail_positions.remove(0);
            }
        }
    }

    /// Generate the full orbital path using centralized math
    pub fn generate_orbital_path(&self, center: Vector3, num_points: usize) -> Vec<Vector3> {
        generate_orbital_path(
            center,
            self.orbital_params.semi_major_axis,
            self.orbital_params.eccentricity,
            self.orbital_params.inclination,
            num_points,
        )
    }

    pub fn get_trail_color(&self) -> Color {
        match self.shader {
            ShaderType::Rocky => Color::new(100, 150, 100, 255),
            ShaderType::GasGiant => Color::new(200, 150, 100, 255),
            ShaderType::Ringed => Color::new(150, 180, 200, 255),
            ShaderType::Magenta => Color::new(200, 100, 200, 255),
            ShaderType::WaterWorld => Color::new(100, 150, 200, 255),
            ShaderType::BlackHole => Color::new(255, 150, 50, 255),
            _ => Color::new(150, 150, 150, 255),
        }
    }
}

pub struct SolarSystem {
    pub center: CentralBody,
    pub planets: Vec<Planet>,
    pub distance_ratio: f32,
    pub show_orbital_paths: bool,
    pub show_trails: bool,
}

impl SolarSystem {
    pub fn new(center: CentralBody, distance_ratio: f32) -> Self {
        Self {
            center,
            planets: Vec::new(),
            distance_ratio,
            show_orbital_paths: true,
            show_trails: true,
        }
    }

    pub fn add_planet(
        &mut self,
        shader: ShaderType,
        scale: f32,
        eccentricity: f32,
        inclination: f32,
    ) {
        let planet_count = self.planets.len();
        let distance = if planet_count == 0 {
            5.0
        } else {
            self.planets[planet_count - 1]
                .orbital_params
                .semi_major_axis
                * self.distance_ratio
        };

        // Calculate mean motion using Kepler's third law (simplified)
        // n = sqrt(GM/a³) ≈ k/sqrt(a³) where k is a constant
        // For our purposes, we'll use a simple inverse relationship with distance
        let mean_motion = 0.3 / distance.sqrt();

        let planet = Planet {
            orbital_params: OrbitalParams {
                semi_major_axis: distance,
                eccentricity,
                inclination,
                mean_motion,       // Radians per second
                mean_anomaly: 0.0, // Start at periapsis
            },
            scale,
            shader,
            rotation: Vector3::zero(),
            trail_positions: Vec::new(),
            trail_max_length: 80,
        };

        self.planets.push(planet);
    }

    pub fn remove_planet(&mut self, index: usize) {
        if index < self.planets.len() {
            self.planets.remove(index);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.center.update(dt);
        let center_pos = self.center.position();
        for planet in &mut self.planets {
            planet.update(dt, center_pos);
        }
    }

    pub fn set_center(&mut self, new_center: CentralBody) {
        self.center = new_center;
    }

    pub fn toggle_orbital_paths(&mut self) {
        self.show_orbital_paths = !self.show_orbital_paths;
    }

    pub fn toggle_trails(&mut self) {
        self.show_trails = !self.show_trails;
    }
}
