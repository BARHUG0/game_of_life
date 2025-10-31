//weapon.rs
use raylib::prelude::*;

/// Type of weapon
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponType {
    MachineGun,
    // Future: Pistol, Shotgun, etc.
}

/// Weapon animation state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponAnimationType {
    Idle,
    Firing,
}

/// Weapon animation state manager
#[derive(Debug, Clone)]
struct WeaponAnimation {
    current_type: WeaponAnimationType,
    frame_index: usize,
    frame_timer: f32,
    frame_duration: f32,
    num_frames: usize,
    base_index: usize,
    loop_animation: bool,
}

impl WeaponAnimation {
    fn new(anim_type: WeaponAnimationType, frame_duration: f32) -> Self {
        let (num_frames, base_index, loop_animation) = match anim_type {
            WeaponAnimationType::Idle => (1, 0, true), // Single idle frame at index 0
            WeaponAnimationType::Firing => (3, 1, true), // Firing frames at indices 1-3
        };

        WeaponAnimation {
            current_type: anim_type,
            frame_index: 0,
            frame_timer: 0.0,
            frame_duration,
            num_frames,
            base_index,
            loop_animation,
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.frame_timer += delta_time;

        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.frame_index += 1;

            if self.frame_index >= self.num_frames {
                if self.loop_animation {
                    self.frame_index = 0;
                } else {
                    self.frame_index = self.num_frames - 1;
                }
            }
        }
    }

    fn set_animation(&mut self, anim_type: WeaponAnimationType) {
        if self.current_type == anim_type {
            return; // Already playing this animation
        }

        let (num_frames, base_index, loop_animation) = match anim_type {
            WeaponAnimationType::Idle => (1, 0, true),
            WeaponAnimationType::Firing => (3, 1, true),
        };

        self.current_type = anim_type;
        self.frame_index = 0;
        self.frame_timer = 0.0;
        self.num_frames = num_frames;
        self.base_index = base_index;
        self.loop_animation = loop_animation;
    }

    fn current_texture_index(&self) -> usize {
        self.base_index + self.frame_index
    }

    fn reset(&mut self) {
        self.frame_index = 0;
        self.frame_timer = 0.0;
    }
}

/// Weapon instance
#[derive(Debug, Clone)]
pub struct Weapon {
    weapon_type: WeaponType,
    animation: WeaponAnimation,

    // Weapon stats
    damage: i32,
    fire_rate: f32,  // Time between shots in seconds
    fire_timer: f32, // Cooldown timer

    // State
    is_firing: bool,
}

impl Weapon {
    /// Create a new machine gun
    pub fn new_machine_gun() -> Self {
        Weapon {
            weapon_type: WeaponType::MachineGun,
            animation: WeaponAnimation::new(WeaponAnimationType::Idle, 0.05), // Fast animation
            damage: 5,
            fire_rate: 0.05, // 10 shots per second
            fire_timer: 0.0,
            is_firing: false,
        }
    }

    /// Update weapon state
    pub fn update(&mut self, delta_time: f32) {
        // Update fire cooldown
        if self.fire_timer > 0.0 {
            self.fire_timer -= delta_time;
        }

        // Update animation
        if self.is_firing {
            self.animation.set_animation(WeaponAnimationType::Firing);
        } else {
            self.animation.set_animation(WeaponAnimationType::Idle);
        }

        self.animation.update(delta_time);
    }

    /// Try to fire the weapon, returns true if shot was fired
    pub fn try_fire(&mut self) -> bool {
        if self.fire_timer <= 0.0 {
            self.fire_timer = self.fire_rate;
            self.is_firing = true;
            self.animation.reset(); // Reset animation to start of firing sequence
            return true;
        }
        false
    }

    /// Stop firing (called when mouse button released)
    pub fn stop_firing(&mut self) {
        self.is_firing = false;
    }

    /// Get current texture index for rendering
    pub fn texture_index(&self) -> usize {
        self.animation.current_texture_index()
    }

    /// Get weapon damage
    pub fn damage(&self) -> i32 {
        self.damage
    }

    /// Check if weapon is currently firing
    pub fn is_firing(&self) -> bool {
        self.is_firing
    }
}

/// Result of a weapon shot
#[derive(Debug, Clone)]
pub struct ShotResult {
    pub hit: bool,
    pub distance: f32,
    pub hit_position: Vector2,
    pub damage: i32,
}

impl ShotResult {
    pub fn new(hit: bool, distance: f32, hit_position: Vector2, damage: i32) -> Self {
        ShotResult {
            hit,
            distance,
            hit_position,
            damage,
        }
    }

    pub fn miss() -> Self {
        ShotResult {
            hit: false,
            distance: f32::MAX,
            hit_position: Vector2::zero(),
            damage: 0,
        }
    }
}
