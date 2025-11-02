//game_state.rs
use crate::sprite::PickupType;

/// Tracks player inventory and game progress
#[derive(Debug, Clone)]
pub struct GameState {
    health: i32,
    max_health: i32,
    ammo: i32,
    keys: i32,
    treasure: i32,
    score: i32,
    kills: i32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            health: 100,
            max_health: 100,
            ammo: 200,
            keys: 0,
            treasure: 0,
            score: 0,
            kills: 0,
        }
    }

    /// Process a collected pickup
    pub fn collect_pickup(&mut self, pickup_type: PickupType) -> String {
        match pickup_type {
            PickupType::Health => {
                let old_health = self.health;
                self.health = (self.health + 25).min(self.max_health);
                let gained = self.health - old_health;
                format!("Health +{}", gained)
            }
            PickupType::Ammo => {
                self.ammo += 20;
                self.score += 10;
                format!("Ammo +20")
            }
            PickupType::Key => {
                self.keys += 1;
                self.score += 50;
                format!("Key collected!")
            }
            PickupType::Treasure => {
                self.treasure += 1;
                self.score += 100;
                format!("Treasure +100 points!")
            }
        }
    }

    // Getters
    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn max_health(&self) -> i32 {
        self.max_health
    }

    pub fn ammo(&self) -> i32 {
        self.ammo
    }

    pub fn keys(&self) -> i32 {
        self.keys
    }

    pub fn treasure(&self) -> i32 {
        self.treasure
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn kills(&self) -> i32 {
        self.kills
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    /// Check if player is dead
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    /// Add a kill
    pub fn add_kill(&mut self) {
        self.kills += 1;
        self.score += 25;
    }

    /// Format stats as string for UI
    pub fn format_stats(&self) -> String {
        format!(
            "Health: {}/{} | Ammo: {} | Keys: {} | Treasure: {} | Score: {}",
            self.health, self.max_health, self.ammo, self.keys, self.treasure, self.score
        )
    }

    pub fn use_ammo(&mut self, amount: i32) {
        self.ammo = (self.ammo - amount).max(0);
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
