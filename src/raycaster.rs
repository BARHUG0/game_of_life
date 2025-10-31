//raycaster.rs
use raylib::prelude::*;
use std::f32::consts::PI;

use crate::maze::Maze;
use crate::player::Player;
use crate::ray::{Ray, Side};

/// DDA (Digital Differential Analyzer) raycasting implementation
/// This is significantly faster than your original incremental approach
pub fn cast_ray(player: &Player, maze: &Maze, block_size: usize, angle: f32) -> Option<Ray> {
    let pos_x = player.x();
    let pos_y = player.y();

    // Ray direction
    let dir_x = angle.cos();
    let dir_y = angle.sin();

    // Which grid cell we're in
    let mut map_x = (pos_x / block_size as f32) as i32;
    let mut map_y = (pos_y / block_size as f32) as i32;

    // Length of ray from one x or y-side to next x or y-side
    let delta_dist_x = if dir_x == 0.0 {
        f32::MAX
    } else {
        (1.0 / dir_x).abs()
    };
    let delta_dist_y = if dir_y == 0.0 {
        f32::MAX
    } else {
        (1.0 / dir_y).abs()
    };

    // What direction to step in x or y-direction (either +1 or -1)
    let step_x: i32;
    let step_y: i32;

    // Length of ray from current position to next x or y-side
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    // Calculate step and initial sideDist
    if dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (pos_x / block_size as f32 - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f32 + 1.0 - pos_x / block_size as f32) * delta_dist_x;
    }

    if dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (pos_y / block_size as f32 - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f32 + 1.0 - pos_y / block_size as f32) * delta_dist_y;
    }

    // Perform DDA
    let mut hit = false;
    let mut side = Side::Vertical;
    let max_iterations = 100; // Prevent infinite loops
    let mut iterations = 0;

    while !hit && iterations < max_iterations {
        // Jump to next map square, either in x-direction, or in y-direction
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = Side::Vertical;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = Side::Horizontal;
        }

        // Check if ray has hit a wall
        if map_y < 0 || map_x < 0 {
            break;
        }

        let grid_y = map_y as usize;
        let grid_x = map_x as usize;

        if grid_y >= maze.len() || grid_x >= maze[0].len() {
            break;
        }

        if maze[grid_y][grid_x] != ' ' {
            hit = true;
        }

        iterations += 1;
    }

    if !hit {
        return None;
    }

    // Calculate distance (perpendicular distance to avoid fisheye effect)
    let perp_wall_dist = if side == Side::Vertical {
        (map_x as f32 - pos_x / block_size as f32 + (1.0 - step_x as f32) / 2.0) / dir_x
    } else {
        (map_y as f32 - pos_y / block_size as f32 + (1.0 - step_y as f32) / 2.0) / dir_y
    };

    let distance = perp_wall_dist * block_size as f32;

    // Calculate exact hit point in world coordinates
    let hit_x = pos_x + dir_x * distance;
    let hit_y = pos_y + dir_y * distance;

    let wall_type = maze[map_y as usize][map_x as usize];

    Some(Ray::new(
        distance,
        Vector2::new(hit_x, hit_y),
        wall_type,
        side,
    ))
}

/// Cast multiple rays for a full field of view
pub fn cast_rays(
    player: &Player,
    maze: &Maze,
    block_size: usize,
    fov: f32,
    num_rays: usize,
) -> Vec<Ray> {
    let mut rays = Vec::with_capacity(num_rays);
    let angle_step = fov / num_rays as f32;
    let start_angle = player.angle_of_view() - fov / 2.0;

    for i in 0..num_rays {
        let ray_angle = start_angle + i as f32 * angle_step;
        if let Some(ray) = cast_ray(player, maze, block_size, ray_angle) {
            rays.push(ray);
        }
    }

    rays
}

/// Cast a single ray in the direction the player is facing (for debugging)
pub fn cast_single_ray(player: &Player, maze: &Maze, block_size: usize) -> Option<Ray> {
    cast_ray(player, maze, block_size, player.angle_of_view())
}
