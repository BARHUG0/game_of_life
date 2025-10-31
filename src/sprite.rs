//sprite.rs
use rand::prelude::*;
use raylib::prelude::*;

/// Represents a sprite in the game world
#[derive(Debug, Clone)]
pub struct Sprite {
    position: Vector2,
    texture_index: usize,
    scale: f32,
    active: bool,
}

impl Sprite {
    pub fn new(position: Vector2, texture_index: usize) -> Self {
        Sprite {
            position,
            texture_index,
            scale: 1.0,
            active: true,
        }
    }

    pub fn with_scale(position: Vector2, texture_index: usize, scale: f32) -> Self {
        Sprite {
            position,
            texture_index,
            scale,
            active: true,
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
}

/// Configuration for sprite spawning
pub struct SpriteSpawnConfig {
    pub num_sprites: usize,
    pub min_spacing: f32, // Minimum distance between sprites
    pub num_textures: usize,
}

impl Default for SpriteSpawnConfig {
    fn default() -> Self {
        SpriteSpawnConfig {
            num_sprites: 20,
            min_spacing: 64.0,
            num_textures: 4,
        }
    }
}

/// Spawn sprites in the maze ensuring good distribution
pub fn spawn_sprites_in_maze(
    maze: &crate::maze::Maze,
    block_size: usize,
    num_sprites: usize,
) -> Vec<Sprite> {
    let config = SpriteSpawnConfig {
        num_sprites,
        min_spacing: block_size as f32 * 1.5,
        num_textures: 4,
    };
    spawn_sprites_with_config(maze, block_size, &config)
}

/// Spawn sprites with custom configuration
pub fn spawn_sprites_with_config(
    maze: &crate::maze::Maze,
    block_size: usize,
    config: &SpriteSpawnConfig,
) -> Vec<Sprite> {
    let mut sprites = Vec::new();
    let mut rng = rand::rng();

    // Collect all valid spawn positions first
    let mut valid_positions = Vec::new();
    for y in 1..maze.len() - 1 {
        for x in 1..maze[0].len() - 1 {
            if maze[y][x] == ' ' {
                // Check if it has at least one wall neighbor (avoid center of large rooms)
                let has_nearby_wall = check_nearby_walls(maze, x, y, 2);
                if has_nearby_wall {
                    let world_x = (x * block_size + block_size / 2) as f32;
                    let world_y = (y * block_size + block_size / 2) as f32;
                    valid_positions.push((world_x, world_y));
                }
            }
        }
    }

    // Shuffle positions for randomness
    for i in (1..valid_positions.len()).rev() {
        let j = rng.random_range(0..=i);
        valid_positions.swap(i, j);
    }

    // Place sprites with minimum spacing
    for (world_x, world_y) in valid_positions {
        if sprites.len() >= config.num_sprites {
            break;
        }

        // Check distance to existing sprites
        let too_close = sprites.iter().any(|s: &Sprite| {
            let dx = s.x() - world_x;
            let dy = s.y() - world_y;
            let dist = (dx * dx + dy * dy).sqrt();
            dist < config.min_spacing
        });

        if !too_close {
            let texture_index = rng.random_range(0..config.num_textures);
            sprites.push(Sprite::new(Vector2::new(world_x, world_y), texture_index));
        }
    }

    sprites
}

/// Check if there are walls nearby (helps avoid placing sprites in large open areas)
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
