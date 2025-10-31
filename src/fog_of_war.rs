//fog_of_war.rs
use raylib::prelude::*;

use crate::maze::Maze;
use crate::player::Player;
use crate::ray::Ray;

/// Tracks which areas of the maze have been explored
pub struct FogOfWar {
    explored: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    block_size: usize,
    vision_radius: f32, // How far the player can "see" to reveal fog
}

impl FogOfWar {
    /// Create a new fog of war grid matching the maze dimensions
    pub fn new(maze: &Maze, block_size: usize, vision_radius: f32) -> Self {
        let height = maze.len();
        let width = if height > 0 { maze[0].len() } else { 0 };

        let explored = vec![vec![false; width]; height];

        FogOfWar {
            explored,
            width,
            height,
            block_size,
            vision_radius,
        }
    }

    // Getters
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn vision_radius(&self) -> f32 {
        self.vision_radius
    }

    // Setters
    pub fn set_vision_radius(&mut self, radius: f32) {
        self.vision_radius = radius;
    }

    /// Check if a grid cell has been explored
    pub fn is_explored(&self, grid_x: usize, grid_y: usize) -> bool {
        if grid_y >= self.height || grid_x >= self.width {
            return false;
        }
        self.explored[grid_y][grid_x]
    }

    /// Mark a grid cell as explored
    pub fn mark_explored(&mut self, grid_x: usize, grid_y: usize) {
        if grid_y < self.height && grid_x < self.width {
            self.explored[grid_y][grid_x] = true;
        }
    }

    /// Update fog of war based on player position with line-of-sight check
    /// Reveals cells in a radius around the player, but only if there's no wall blocking
    pub fn update_from_position(&mut self, player: &Player, maze: &Maze) {
        let player_grid_x = (player.x() / self.block_size as f32) as i32;
        let player_grid_y = (player.y() / self.block_size as f32) as i32;

        let radius_in_cells = (self.vision_radius / self.block_size as f32).ceil() as i32;

        // Check cells in a square area (we'll filter by distance and line-of-sight)
        for dy in -radius_in_cells..=radius_in_cells {
            for dx in -radius_in_cells..=radius_in_cells {
                let grid_x = player_grid_x + dx;
                let grid_y = player_grid_y + dy;

                // Check if within bounds
                if grid_x < 0 || grid_y < 0 {
                    continue;
                }

                let gx = grid_x as usize;
                let gy = grid_y as usize;

                if gx >= self.width || gy >= self.height {
                    continue;
                }

                // Check if within vision radius (circular area)
                let cell_center_x = (gx * self.block_size + self.block_size / 2) as f32;
                let cell_center_y = (gy * self.block_size + self.block_size / 2) as f32;

                let dist = ((cell_center_x - player.x()).powi(2)
                    + (cell_center_y - player.y()).powi(2))
                .sqrt();

                if dist > self.vision_radius {
                    continue;
                }

                // Check line-of-sight using Bresenham-like algorithm
                if self.has_line_of_sight(player, gx, gy, maze) {
                    self.mark_explored(gx, gy);
                }
            }
        }
    }

    /// Check if there's a clear line of sight from player to target cell
    /// Uses DDA-like algorithm to trace a line and check for walls
    fn has_line_of_sight(
        &self,
        player: &Player,
        target_x: usize,
        target_y: usize,
        maze: &Maze,
    ) -> bool {
        let player_grid_x = (player.x() / self.block_size as f32) as usize;
        let player_grid_y = (player.y() / self.block_size as f32) as usize;

        // If checking the cell the player is in, always visible
        if player_grid_x == target_x && player_grid_y == target_y {
            return true;
        }

        let target_center_x = (target_x * self.block_size + self.block_size / 2) as f32;
        let target_center_y = (target_y * self.block_size + self.block_size / 2) as f32;

        // Direction vector from player to target
        let dx = target_center_x - player.x();
        let dy = target_center_y - player.y();
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 0.001 {
            return true;
        }

        // Normalize direction
        let dir_x = dx / distance;
        let dir_y = dy / distance;

        // Step along the ray
        let step_size = (self.block_size as f32) / 2.0;
        let num_steps = (distance / step_size).ceil() as i32;

        for i in 1..num_steps {
            let check_x = player.x() + dir_x * step_size * i as f32;
            let check_y = player.y() + dir_y * step_size * i as f32;

            let check_grid_x = (check_x / self.block_size as f32) as usize;
            let check_grid_y = (check_y / self.block_size as f32) as usize;

            // Out of bounds
            if check_grid_y >= maze.len() || check_grid_x >= maze[0].len() {
                return false;
            }

            // Hit a wall before reaching target
            if maze[check_grid_y][check_grid_x] != ' ' {
                // If this is the target cell itself, it's visible (we can see the wall)
                if check_grid_x == target_x && check_grid_y == target_y {
                    return true;
                }
                return false;
            }
        }

        true
    }

    /// Update fog of war based on raycasting results
    /// Reveals all cells that were hit by rays (more realistic line-of-sight)
    pub fn update_from_rays(&mut self, rays: &[Ray]) {
        for ray in rays {
            let hit_grid_x = (ray.hit_point().x / self.block_size as f32) as usize;
            let hit_grid_y = (ray.hit_point().y / self.block_size as f32) as usize;

            self.mark_explored(hit_grid_x, hit_grid_y);
        }
    }

    /// Hybrid approach: reveal by position AND by raycasting
    /// This gives a good balance between exploration and line-of-sight
    pub fn update(&mut self, player: &Player, rays: &[Ray], maze: &Maze) {
        self.update_from_position(player, maze);
        self.update_from_rays(rays);
    }

    /// Reset all explored areas (useful for debugging or new game)
    pub fn reset(&mut self) {
        for row in &mut self.explored {
            for cell in row {
                *cell = false;
            }
        }
    }

    /// Get a reference to the explored grid (for custom rendering)
    pub fn explored_grid(&self) -> &Vec<Vec<bool>> {
        &self.explored
    }
}
