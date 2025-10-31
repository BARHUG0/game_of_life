//enemy.rs
use rand::prelude::*;
use raylib::prelude::*;

/// Type of enemy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Rat, // Melee, fast, low health
         // Future: Guard, Demon, etc.
}

/// Enemy AI state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyState {
    Idle,
    Chase,
    Attack,
    Dead,
}

/// Represents an enemy in the game
#[derive(Debug, Clone)]
pub struct Enemy {
    position: Vector2,
    enemy_type: EnemyType,
    state: EnemyState,
    health: i32,
    max_health: i32,
    texture_index: usize,
    scale: f32,

    // Behavior properties
    detection_radius: f32,
    attack_range: f32,
    move_speed: f32,
    attack_damage: i32,
    attack_cooldown: f32,
    attack_timer: f32,

    // Movement
    facing_angle: f32,
}

impl Enemy {
    /// Create a new rat enemy
    pub fn new_rat(position: Vector2, texture_index: usize) -> Self {
        Enemy {
            position,
            enemy_type: EnemyType::Rat,
            state: EnemyState::Idle,
            health: 20,
            max_health: 20,
            texture_index,
            scale: 0.7,
            detection_radius: 300.0,
            attack_range: 40.0,
            move_speed: 80.0,
            attack_damage: 10,
            attack_cooldown: 1.0,
            attack_timer: 0.0,
            facing_angle: 0.0,
        }
    }

    // Getters
    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn state(&self) -> EnemyState {
        self.state
    }

    pub fn is_alive(&self) -> bool {
        self.state != EnemyState::Dead
    }

    pub fn texture_index(&self) -> usize {
        self.texture_index
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    /// Update enemy behavior
    pub fn update(
        &mut self,
        delta_time: f32,
        player_pos: Vector2,
        maze: &crate::maze::Maze,
        block_size: usize,
    ) -> Option<i32> {
        if !self.is_alive() {
            return None;
        }

        // Update attack timer
        if self.attack_timer > 0.0 {
            self.attack_timer -= delta_time;
        }

        // Calculate distance to player
        let dx = player_pos.x - self.position.x;
        let dy = player_pos.y - self.position.y;
        let distance_to_player = (dx * dx + dy * dy).sqrt();

        // Update facing angle
        self.facing_angle = dy.atan2(dx);

        // State machine
        match self.state {
            EnemyState::Idle => {
                // Check if player is within detection radius
                if distance_to_player < self.detection_radius {
                    // Simple line-of-sight check (can see player through walls for now)
                    self.state = EnemyState::Chase;
                }
            }
            EnemyState::Chase => {
                // Check if in attack range
                if distance_to_player < self.attack_range {
                    self.state = EnemyState::Attack;
                } else if distance_to_player > self.detection_radius * 1.5 {
                    // Lost player
                    self.state = EnemyState::Idle;
                } else {
                    // Move toward player
                    self.move_toward_player(delta_time, player_pos, maze, block_size);
                }
            }
            EnemyState::Attack => {
                // Check if player moved away
                if distance_to_player > self.attack_range * 1.2 {
                    self.state = EnemyState::Chase;
                } else {
                    // Attack if cooldown is ready
                    if self.attack_timer <= 0.0 {
                        self.attack_timer = self.attack_cooldown;
                        return Some(self.attack_damage); // Return damage dealt
                    }
                }
            }
            EnemyState::Dead => {}
        }

        None
    }

    /// Move toward player position
    fn move_toward_player(
        &mut self,
        delta_time: f32,
        player_pos: Vector2,
        maze: &crate::maze::Maze,
        block_size: usize,
    ) {
        let dx = player_pos.x - self.position.x;
        let dy = player_pos.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 1.0 {
            return;
        }

        // Normalize direction
        let dir_x = dx / distance;
        let dir_y = dy / distance;

        // Calculate new position
        let move_distance = self.move_speed * delta_time;
        let new_x = self.position.x + dir_x * move_distance;
        let new_y = self.position.y + dir_y * move_distance;

        // Check collision with walls
        if crate::maze::is_walkable(maze, new_x, new_y, block_size) {
            self.position.x = new_x;
            self.position.y = new_y;
        } else {
            // Try sliding along walls (try X then Y)
            if crate::maze::is_walkable(maze, new_x, self.position.y, block_size) {
                self.position.x = new_x;
            } else if crate::maze::is_walkable(maze, self.position.x, new_y, block_size) {
                self.position.y = new_y;
            }
        }
    }

    /// Take damage
    pub fn take_damage(&mut self, damage: i32) -> bool {
        if !self.is_alive() {
            return false;
        }

        self.health -= damage;
        if self.health <= 0 {
            self.health = 0;
            self.state = EnemyState::Dead;
            return true; // Enemy died
        }

        false
    }

    /// Distance to point
    pub fn distance_to(&self, x: f32, y: f32) -> f32 {
        let dx = self.position.x - x;
        let dy = self.position.y - y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Spawn enemies in the maze
pub fn spawn_enemies(
    maze: &crate::maze::Maze,
    block_size: usize,
    player_pos: Vector2,
    num_enemies: usize,
    rat_texture_index: usize,
) -> Vec<Enemy> {
    let mut enemies = Vec::new();
    let mut rng = rand::rng();

    // Collect valid spawn positions (far from player)
    let mut valid_positions = Vec::new();
    for y in 2..maze.len() - 2 {
        for x in 2..maze[0].len() - 2 {
            if maze[y][x] == ' ' {
                let world_x = (x * block_size + block_size / 2) as f32;
                let world_y = (y * block_size + block_size / 2) as f32;

                // Check distance from player
                let dx = world_x - player_pos.x;
                let dy = world_y - player_pos.y;
                let distance = (dx * dx + dy * dy).sqrt();

                // Must be far enough from player
                if distance > block_size as f32 * 5.0 {
                    valid_positions.push((world_x, world_y));
                }
            }
        }
    }

    // Shuffle positions
    for i in (1..valid_positions.len()).rev() {
        let j = rng.random_range(0..=i);
        valid_positions.swap(i, j);
    }

    // Spawn enemies with spacing
    let min_spacing = block_size as f32 * 3.0;
    for (world_x, world_y) in valid_positions {
        if enemies.len() >= num_enemies {
            break;
        }

        // Check distance to other enemies
        let too_close = enemies.iter().any(|e: &Enemy| {
            let dx = e.x() - world_x;
            let dy = e.y() - world_y;
            (dx * dx + dy * dy).sqrt() < min_spacing
        });

        if !too_close {
            enemies.push(Enemy::new_rat(
                Vector2::new(world_x, world_y),
                rat_texture_index,
            ));
        }
    }

    enemies
}

/// Update all enemies and return total damage dealt to player
pub fn update_enemies(
    enemies: &mut [Enemy],
    delta_time: f32,
    player_pos: Vector2,
    maze: &crate::maze::Maze,
    block_size: usize,
) -> i32 {
    let mut total_damage = 0;

    for enemy in enemies.iter_mut() {
        if let Some(damage) = enemy.update(delta_time, player_pos, maze, block_size) {
            total_damage += damage;
        }
    }

    total_damage
}
