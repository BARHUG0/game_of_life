//maze.rs
use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::framebuffer::Framebuffer;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str, block_size: usize) -> (Maze, Vector2) {
    let file = File::open(filename).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut maze: Maze = Vec::new();
    let mut player_pos = None;

    for (row_index, line) in reader.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (col_index, ch) in line.expect("Error reading line").chars().enumerate() {
            if ch == 'p' {
                let x = (col_index * block_size + block_size / 2) as f32;
                let y = (row_index * block_size + block_size / 2) as f32;
                player_pos = Some(Vector2 { x, y });
                row.push(' ');
            } else {
                row.push(ch);
            }
        }
        maze.push(row);
    }

    let position = player_pos.expect("No player position ('p') found in maze");
    (maze, position)
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    match cell {
        ' ' => framebuffer.set_foreground_color(Color::WHITESMOKE),
        _ => framebuffer.set_foreground_color(Color::PURPLE),
    }

    let xf = if xo + block_size < framebuffer.width() as usize {
        xo + block_size
    } else {
        framebuffer.width() as usize
    };

    let yf = if yo + block_size < framebuffer.height() as usize {
        yo + block_size
    } else {
        framebuffer.height() as usize
    };

    for x in xo..xf {
        for y in yo..yf {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze, block_size: usize) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
}

pub fn render_player(framebuffer: &mut Framebuffer, position: Vector2) {
    framebuffer.set_foreground_color(Color::GREENYELLOW);

    let size = 5; // radius of the square
    let cx = position.x as i32;
    let cy = position.y as i32;

    for x in cx - size..=cx + size {
        for y in cy - size..=cy + size {
            framebuffer.set_pixel(x, y);
        }
    }
}
