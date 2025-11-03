//sprite.rs
use rand::prelude::*;
use raylib::prelude::*;

/// Type of sprite in the game
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpriteType {
    Decoration,
    Pickup(PickupType),
}

/// Different types of pickable items
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PickupType {
    Health,
    Ammo,
    Key,
    Treasure,
}

/// Represents a sprite in the game world
#[derive(Debug, Clone)]
pub struct Sprite {
    position: Vector2,
    texture_index: usize,
    scale: f32,
    active: bool,
    sprite_type: SpriteType,
    pickup_radius: f32, // Distance at which pickup can occur
}

impl Sprite {
    pub fn new(position: Vector2, texture_index: usize) -> Self {
        Sprite {
            position,
            texture_index,
            scale: 1.0,
            active: true,
            sprite_type: SpriteType::Decoration,
            pickup_radius: 32.0,
        }
    }

    pub fn with_scale(position: Vector2, texture_index: usize, scale: f32) -> Self {
        Sprite {
            position,
            texture_index,
            scale,
            active: true,
            sprite_type: SpriteType::Decoration,
            pickup_radius: 32.0,
        }
    }

    /// Create a pickup sprite
    pub fn new_pickup(position: Vector2, texture_index: usize, pickup_type: PickupType) -> Self {
        Sprite {
            position,
            texture_index,
            scale: 0.8, // Pickups slightly smaller
            active: true,
            sprite_type: SpriteType::Pickup(pickup_type),
            pickup_radius: 48.0, // Larger pickup radius than decorations
        }
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn texture_index(&self) -> usize {
        self.texture_index
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn sprite_type(&self) -> SpriteType {
        self.sprite_type
    }

    pub fn pickup_radius(&self) -> f32 {
        self.pickup_radius
    }

    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    pub fn set_texture_index(&mut self, index: usize) {
        self.texture_index = index;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn distance_to(&self, player_x: f32, player_y: f32) -> f32 {
        let dx = self.position.x - player_x;
        let dy = self.position.y - player_y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if player can pick up this sprite
    pub fn can_pickup(&self, player_x: f32, player_y: f32) -> bool {
        if !self.active {
            return false;
        }

        match self.sprite_type {
            SpriteType::Decoration => false,
            SpriteType::Pickup(_) => self.distance_to(player_x, player_y) <= self.pickup_radius,
        }
    }

    /// Collect this pickup (deactivates it)
    pub fn collect(&mut self) -> Option<PickupType> {
        if !self.active {
            return None;
        }

        match self.sprite_type {
            SpriteType::Decoration => None,
            SpriteType::Pickup(pickup_type) => {
                self.active = false;
                Some(pickup_type)
            }
        }
    }

    /// Check if this is a pickup sprite
    pub fn is_pickup(&self) -> bool {
        matches!(self.sprite_type, SpriteType::Pickup(_))
    }
}

/// Configuration for sprite spawning
pub struct SpriteSpawnConfig {
    pub num_decorations: usize,
    pub num_pickups: usize,
    pub min_spacing: f32,
    pub num_textures: usize,
}

impl Default for SpriteSpawnConfig {
    fn default() -> Self {
        SpriteSpawnConfig {
            num_decorations: 15,
            num_pickups: 10,
            min_spacing: 64.0,
            num_textures: 4,
        }
    }
}

/// Spawn mixed sprites (decorations and pickups) in the maze
pub fn spawn_sprites_in_maze(
    maze: &crate::maze::Maze,
    block_size: usize,
    num_sprites: usize,
) -> Vec<Sprite> {
    let config = SpriteSpawnConfig {
        num_decorations: num_sprites * 3 / 5, // 60% decorations
        num_pickups: num_sprites * 2 / 5,     // 40% pickups
        min_spacing: block_size as f32 * 1.5,
        num_textures: 4,
    };
    spawn_sprites_with_config(maze, block_size, &config)
}

pub fn spawn_sprites_with_config(
    maze: &crate::maze::Maze,
    block_size: usize,
    config: &SpriteSpawnConfig,
) -> Vec<Sprite> {
    let mut sprites = Vec::new();
    let mut rng = rand::rng();
    let mut key_spawned = false;

    // Collect all valid spawn positions
    let mut valid_positions = Vec::new();
    for y in 1..maze.len() - 1 {
        for x in 1..maze[0].len() - 1 {
            if maze[y][x] == ' ' {
                let has_nearby_wall = check_nearby_walls(maze, x, y, 2);
                if has_nearby_wall {
                    let world_x = (x * block_size + block_size / 2) as f32;
                    let world_y = (y * block_size + block_size / 2) as f32;
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

    // Place decorations first
    for (world_x, world_y) in &valid_positions {
        if sprites.len() >= config.num_decorations {
            break;
        }

        let too_close = sprites.iter().any(|s: &Sprite| {
            let dx = s.x() - world_x;
            let dy = s.y() - world_y;
            (dx * dx + dy * dy).sqrt() < config.min_spacing
        });

        if !too_close {
            let texture_index = rng.random_range(0..config.num_textures);
            sprites.push(Sprite::new(Vector2::new(*world_x, *world_y), texture_index));
        }
    }

    // Then place pickups - ONE KEY ONLY
    let pickup_start_pos = config.num_decorations;
    for (world_x, world_y) in valid_positions.iter().skip(pickup_start_pos) {
        if sprites.iter().filter(|s| s.is_pickup()).count() >= config.num_pickups {
            break;
        }

        let too_close = sprites.iter().any(|s: &Sprite| {
            let dx = s.x() - world_x;
            let dy = s.y() - world_y;
            (dx * dx + dy * dy).sqrt() < config.min_spacing
        });

        if !too_close {
            // Distribute pickup types - ensuring ONLY ONE KEY
            let pickup_type = if !key_spawned && rng.random_range(0..4) == 2 {
                key_spawned = true;
                PickupType::Key
            } else {
                match rng.random_range(0..3) {
                    0 => PickupType::Health,
                    1 => PickupType::Ammo,
                    _ => PickupType::Treasure,
                }
            };

            let texture_index = match pickup_type {
                PickupType::Health => 4,
                PickupType::Ammo => 5,
                PickupType::Key => 6,
                PickupType::Treasure => 7,
            };

            sprites.push(Sprite::new_pickup(
                Vector2::new(*world_x, *world_y),
                texture_index,
                pickup_type,
            ));
        }
    }

    // Force spawn key if it wasn't placed yet
    if !key_spawned {
        for (world_x, world_y) in valid_positions.iter().skip(pickup_start_pos) {
            let too_close = sprites.iter().any(|s: &Sprite| {
                let dx = s.x() - world_x;
                let dy = s.y() - world_y;
                (dx * dx + dy * dy).sqrt() < config.min_spacing * 0.5
            });

            if !too_close {
                sprites.push(Sprite::new_pickup(
                    Vector2::new(*world_x, *world_y),
                    6,
                    PickupType::Key,
                ));
                break;
            }
        }
    }

    sprites
}

/// Check if there are walls nearby
fn check_nearby_walls(maze: &crate::maze::Maze, x: usize, y: usize, radius: usize) -> bool {
    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || ny < 0 || ny >= maze.len() as i32 || nx >= maze[0].len() as i32 {
                continue;
            }

            if maze[ny as usize][nx as usize] != ' ' {
                return true;
            }
        }
    }
    false
}

/// Process pickups for the player - returns list of collected items
pub fn process_pickups(sprites: &mut [Sprite], player_x: f32, player_y: f32) -> Vec<PickupType> {
    let mut collected = Vec::new();

    for sprite in sprites.iter_mut() {
        if sprite.can_pickup(player_x, player_y) {
            if let Some(pickup_type) = sprite.collect() {
                collected.push(pickup_type);
            }
        }
    }

    collected
}
