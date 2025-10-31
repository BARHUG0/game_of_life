//maze_generator.rs
use rand::Rng;
use raylib::prelude::*;

use crate::maze::Maze;

/// Configuration for maze generation
pub struct MazeConfig {
    width: usize,
    height: usize,
    wall_types: Vec<char>,
    num_rooms: usize,     // Number of rooms to generate
    min_room_size: usize, // Minimum room dimension
    max_room_size: usize, // Maximum room dimension
}

impl MazeConfig {
    pub fn new(width: usize, height: usize) -> Self {
        MazeConfig {
            width,
            height,
            wall_types: vec!['1', '2', '3', '4'],
            num_rooms: 5,
            min_room_size: 3,
            max_room_size: 7,
        }
    }

    pub fn with_wall_types(mut self, wall_types: Vec<char>) -> Self {
        self.wall_types = wall_types;
        self
    }

    pub fn with_rooms(mut self, num_rooms: usize, min_size: usize, max_size: usize) -> Self {
        self.num_rooms = num_rooms;
        self.min_room_size = min_size;
        self.max_room_size = max_size;
        self
    }
}

/// Represents a rectangular room in the maze
#[derive(Debug, Clone)]
struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Room {
    fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Room {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the center point of the room
    fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Check if this room overlaps with another room
    fn overlaps(&self, other: &Room, margin: usize) -> bool {
        self.x < other.x + other.width + margin
            && self.x + self.width + margin > other.x
            && self.y < other.y + other.height + margin
            && self.y + self.height + margin > other.y
    }

    /// Carve this room into the maze
    fn carve(&self, maze: &mut Maze) {
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                if y < maze.len() && x < maze[0].len() {
                    maze[y][x] = ' ';
                }
            }
        }
    }
}

/// Generate a random maze with rooms using recursive backtracking
pub fn generate_maze(config: &MazeConfig, block_size: usize) -> (Maze, Vector2) {
    let mut rng = rand::rng();
    let mut maze = vec![vec!['+'; config.width]; config.height];

    // Step 1: Generate rooms
    let mut rooms = Vec::new();
    for _ in 0..config.num_rooms {
        let width = rng.random_range(config.min_room_size..=config.max_room_size);
        let height = rng.random_range(config.min_room_size..=config.max_room_size);

        let x = rng.random_range(1..config.width.saturating_sub(width + 1));
        let y = rng.random_range(1..config.height.saturating_sub(height + 1));

        let new_room = Room::new(x, y, width, height);

        // Check if room overlaps with existing rooms
        let overlaps = rooms.iter().any(|room: &Room| new_room.overlaps(room, 2));

        if !overlaps {
            new_room.carve(&mut maze);
            rooms.push(new_room);
        }
    }

    // Step 2: Connect rooms with corridors using recursive backtracking
    if !rooms.is_empty() {
        let mut visited = vec![vec![false; config.width / 2]; config.height / 2];

        // Start carving from the first room's center
        let (start_x, start_y) = rooms[0].center();
        carve_passages(
            &mut maze,
            &mut visited,
            start_x / 2,
            start_y / 2,
            &mut rng,
            &config.wall_types,
        );
    }

    // Step 3: Add outer border
    for x in 0..config.width {
        maze[0][x] = '+';
        maze[config.height - 1][x] = '+';
    }
    for y in 0..config.height {
        maze[y][0] = '+';
        maze[y][config.width - 1] = '+';
    }

    // Step 4: Find starting position (preferably in a room)
    let player_pos = if !rooms.is_empty() {
        let room = &rooms[rng.random_range(0..rooms.len())];
        let (cx, cy) = room.center();
        let world_x = (cx * block_size) as f32;
        let world_y = (cy * block_size) as f32;
        Vector2::new(world_x, world_y)
    } else {
        find_starting_position(&maze, block_size, &mut rng)
    };

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
    if y >= visited.len() || x >= visited[0].len() {
        return;
    }

    visited[y][x] = true;

    let maze_x = x * 2 + 1;
    let maze_y = y * 2 + 1;

    if maze_y < maze.len() && maze_x < maze[0].len() {
        maze[maze_y][maze_x] = ' ';
    }

    let mut directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

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

        if ny >= visited.len() || nx >= visited[0].len() || visited[ny][nx] {
            continue;
        }

        let wall_x = maze_x as i32 + dx;
        let wall_y = maze_y as i32 + dy;

        if wall_x >= 0 && wall_y >= 0 && wall_y < maze.len() as i32 && wall_x < maze[0].len() as i32
        {
            maze[wall_y as usize][wall_x as usize] = ' ';
        }

        carve_passages(maze, visited, nx, ny, rng, wall_types);
    }
}

fn find_starting_position(maze: &Maze, block_size: usize, rng: &mut impl Rng) -> Vector2 {
    for _ in 0..1000 {
        let x = rng.random_range(2..maze[0].len() - 2);
        let y = rng.random_range(2..maze.len() - 2);

        if maze[y][x] == ' ' {
            let world_x = (x * block_size + block_size / 2) as f32;
            let world_y = (y * block_size + block_size / 2) as f32;
            return Vector2::new(world_x, world_y);
        }
    }

    // Fallback
    Vector2::new((block_size * 2) as f32, (block_size * 2) as f32)
}

/// Generate a medium-sized maze with rooms and varied wall textures
pub fn generate_large_maze(block_size: usize) -> (Maze, Vector2) {
    let config = MazeConfig::new(35, 35)
        .with_wall_types(vec!['1', '2', '3', '4'])
        .with_rooms(6, 4, 8); // 6 rooms, size 4-8 cells

    let (mut maze, player_pos) = generate_maze(&config, block_size);

    // Add texture variety
    let mut rng = rand::rng();
    for row in maze.iter_mut() {
        for cell in row.iter_mut() {
            if *cell != ' ' {
                let roll = rng.random_range(0..100);
                *cell = if roll < 65 {
                    '1'
                } else if roll < 75 {
                    '2'
                } else if roll < 95 {
                    '3'
                } else {
                    '4'
                };
            }
        }
    }

    (maze, player_pos)
}
