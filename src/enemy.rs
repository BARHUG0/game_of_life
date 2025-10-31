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

/// Animation types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationType {
    Walk,
    Attack,
    Death,
}

/// Animation state
#[derive(Debug, Clone)]
struct AnimationState {
    current_type: AnimationType,
    frame_index: usize,
    frame_timer: f32,
    frame_duration: f32,
    num_frames: usize,
    base_index: usize,
    loop_animation: bool,
    finished: bool,
}

impl AnimationState {
    fn new(anim_type: AnimationType, frame_duration: f32) -> Self {
        let (num_frames, base_index, loop_animation) = match anim_type {
            AnimationType::Walk => (4, 0, true),   // Indices 0-3
            AnimationType::Attack => (3, 4, true), // Indices 4-6
            AnimationType::Death => (3, 7, false), // Indices 7-9, play once
        };

        AnimationState {
            current_type: anim_type,
            frame_index: 0,
            frame_timer: 0.0,
            frame_duration,
            num_frames,
            base_index,
            loop_animation,
            finished: false,
        }
    }

    fn update(&mut self, delta_time: f32) {
        if self.finished {
            return;
        }

        self.frame_timer += delta_time;

        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.frame_index += 1;

            if self.frame_index >= self.num_frames {
                if self.loop_animation {
                    self.frame_index = 0;
                } else {
                    // Non-looping animation finished, stay on last frame
                    self.frame_index = self.num_frames - 1;
                    self.finished = true;
                }
            }
        }
    }

    fn set_animation(&mut self, anim_type: AnimationType) {
        if self.current_type == anim_type && !self.finished {
            return; // Already playing this animation
        }

        let (num_frames, base_index, loop_animation) = match anim_type {
            AnimationType::Walk => (4, 0, true),
            AnimationType::Attack => (3, 4, true),
            AnimationType::Death => (3, 7, false),
        };

        self.current_type = anim_type;
        self.frame_index = 0;
        self.frame_timer = 0.0;
        self.num_frames = num_frames;
        self.base_index = base_index;
        self.loop_animation = loop_animation;
        self.finished = false;
    }

    fn current_texture_index(&self) -> usize {
        self.base_index + self.frame_index
    }
}

/// Represents an enemy in the game
#[derive(Debug, Clone)]
pub struct Enemy {
    position: Vector2,
    enemy_type: EnemyType,
    state: EnemyState,
    health: i32,
    max_health: i32,
    scale: f32,

    // Animation
    animation: AnimationState,

    // Damage flash effect
    damage_flash_timer: f32,
    damage_flash_duration: f32,

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
    pub fn new_rat(position: Vector2) -> Self {
        Enemy {
            position,
            enemy_type: EnemyType::Rat,
            state: EnemyState::Idle,
            health: 20,
            max_health: 20,
            scale: 0.7,
            animation: AnimationState::new(AnimationType::Walk, 0.15),
            damage_flash_timer: 0.0,
            damage_flash_duration: 0.2, // Flash for 0.2 seconds
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
        self.animation.current_texture_index()
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    /// Check if enemy is taking damage (for red flash effect)
    pub fn is_flashing(&self) -> bool {
        self.damage_flash_timer > 0.0
    }

    /// Update enemy behavior
    pub fn update(
        &mut self,
        delta_time: f32,
        player_pos: Vector2,
        maze: &crate::maze::Maze,
        block_size: usize,
    ) -> Option<i32> {
        // Always update animation
        self.animation.update(delta_time);

        // Update damage flash timer
        if self.damage_flash_timer > 0.0 {
            self.damage_flash_timer -= delta_time;
        }

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
                self.animation.set_animation(AnimationType::Walk);

                // Check if player is within detection radius
                if distance_to_player < self.detection_radius {
                    self.state = EnemyState::Chase;
                }
            }
            EnemyState::Chase => {
                self.animation.set_animation(AnimationType::Walk);

                // Check if in attack range
                if distance_to_player < self.attack_range {
                    self.state = EnemyState::Attack;
                    self.animation.set_animation(AnimationType::Attack);
                } else if distance_to_player > self.detection_radius * 1.5 {
                    // Lost player
                    self.state = EnemyState::Idle;
                } else {
                    // Move toward player
                    self.move_toward_player(delta_time, player_pos, maze, block_size);
                }
            }
            EnemyState::Attack => {
                self.animation.set_animation(AnimationType::Attack);

                // Check if player moved away
                if distance_to_player > self.attack_range * 1.2 {
                    self.state = EnemyState::Chase;
                } else {
                    // Attack if cooldown is ready and attack animation is at damage frame
                    if self.attack_timer <= 0.0 {
                        // Deal damage when attack animation reaches frame 1 (middle of attack)
                        if self.animation.frame_index == 1 {
                            self.attack_timer = self.attack_cooldown;
                            return Some(self.attack_damage); // Return damage dealt
                        }
                    }
                }
            }
            EnemyState::Dead => {
                self.animation.set_animation(AnimationType::Death);
            }
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
        self.damage_flash_timer = self.damage_flash_duration; // Start flash effect

        if self.health <= 0 {
            self.health = 0;
            self.state = EnemyState::Dead;
            self.animation.set_animation(AnimationType::Death);
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
            enemies.push(Enemy::new_rat(Vector2::new(world_x, world_y)));
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
