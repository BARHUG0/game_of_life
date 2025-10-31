//maze_generator.rs
use rand::Rng;
use raylib::prelude::*;

use crate::maze::Maze;

/// Configuration for maze generation
pub struct MazeConfig {
    width: usize,
    height: usize,
    wall_types: Vec<char>, // Different wall characters for variety
}

impl MazeConfig {
    pub fn new(width: usize, height: usize) -> Self {
        MazeConfig {
            width,
            height,
            wall_types: vec!['1', '2', '3', '4'], // 4 different wall types
        }
    }

    pub fn with_wall_types(mut self, wall_types: Vec<char>) -> Self {
        self.wall_types = wall_types;
        self
    }
}

/// Generate a random maze using recursive backtracking algorithm
/// Returns (maze, player_starting_position)
pub fn generate_maze(config: &MazeConfig, block_size: usize) -> (Maze, Vector2) {
    let mut rng = rand::rng();

    // Start with all walls
    let mut maze = vec![vec!['+'; config.width]; config.height];

    // Create the actual maze grid (we'll carve paths through walls)
    let mut visited = vec![vec![false; config.width / 2]; config.height / 2];

    // Starting cell for generation
    let start_x = 1;
    let start_y = 1;

    // Recursive backtracking
    carve_passages(
        &mut maze,
        &mut visited,
        start_x,
        start_y,
        &mut rng,
        &config.wall_types,
    );

    // Add outer border (make sure edges are solid walls)
    for x in 0..config.width {
        maze[0][x] = '+';
        maze[config.height - 1][x] = '+';
    }
    for y in 0..config.height {
        maze[y][0] = '+';
        maze[y][config.width - 1] = '+';
    }

    // Find a good starting position for the player (open space, not too close to edge)
    let player_pos = find_starting_position(&maze, block_size, &mut rng);

    (maze, player_pos)
}

/// Recursive backtracking maze generation
fn carve_passages(
    maze: &mut Maze,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    rng: &mut impl Rng,
    wall_types: &[char],
) {
    visited[y][x] = true;

    // Actual maze coordinates (accounting for walls)
    let maze_x = x * 2 + 1;
    let maze_y = y * 2 + 1;

    maze[maze_y][maze_x] = ' '; // Carve out this cell

    // Randomize direction order
    let mut directions = vec![
        (0, -1), // North
        (1, 0),  // East
        (0, 1),  // South
        (-1, 0), // West
    ];

    // Shuffle directions for randomness
    for i in (1..directions.len()).rev() {
        let j = rng.random_range(0..=i);
        directions.swap(i, j);
    }

    for (dx, dy) in directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx < 0 || ny < 0 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if ny >= visited.len() || nx >= visited[0].len() {
            continue;
        }

        if !visited[ny][nx] {
            // Carve passage between current cell and neighbor
            let wall_x = maze_x as i32 + dx;
            let wall_y = maze_y as i32 + dy;

            if wall_x >= 0
                && wall_y >= 0
                && wall_y < maze.len() as i32
                && wall_x < maze[0].len() as i32
            {
                maze[wall_y as usize][wall_x as usize] = ' ';
            }

            carve_passages(maze, visited, nx, ny, rng, wall_types);
        }
    }
}

/// Find a good starting position for the player
fn find_starting_position(maze: &Maze, block_size: usize, rng: &mut impl Rng) -> Vector2 {
    let mut attempts = 0;
    let max_attempts = 1000;

    loop {
        let x = rng.random_range(2..maze[0].len() - 2);
        let y = rng.random_range(2..maze.len() - 2);

        // Check if it's an open space and has some open neighbors
        if maze[y][x] == ' ' {
            let mut open_neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;
                    if ny < maze.len() && nx < maze[0].len() && maze[ny][nx] == ' ' {
                        open_neighbors += 1;
                    }
                }
            }

            // Good starting position: open space with several open neighbors
            if open_neighbors >= 4 {
                let world_x = (x * block_size + block_size / 2) as f32;
                let world_y = (y * block_size + block_size / 2) as f32;
                return Vector2::new(world_x, world_y);
            }
        }

        attempts += 1;
        if attempts > max_attempts {
            // Fallback: find any open space
            for y in 1..maze.len() - 1 {
                for x in 1..maze[0].len() - 1 {
                    if maze[y][x] == ' ' {
                        let world_x = (x * block_size + block_size / 2) as f32;
                        let world_y = (y * block_size + block_size / 2) as f32;
                        return Vector2::new(world_x, world_y);
                    }
                }
            }
        }
    }
}

/// Generate a larger maze with varied wall textures
pub fn generate_large_maze(block_size: usize) -> (Maze, Vector2) {
    let config = MazeConfig::new(51, 51) // Odd numbers work better for maze generation
        .with_wall_types(vec!['1', '2', '3', '4']);

    let (mut maze, player_pos) = generate_maze(&config, block_size);

    // Add variety: randomly assign different wall types
    let mut rng = rand::rng();
    for row in maze.iter_mut() {
        for cell in row.iter_mut() {
            if *cell != ' ' {
                let wall_idx = rng.random_range(0..config.wall_types.len());
                *cell = config.wall_types[wall_idx];
            }
        }
    }

    (maze, player_pos)
}
