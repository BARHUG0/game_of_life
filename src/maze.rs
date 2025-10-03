use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{Framebuffer, framebuffer};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).expect("Error opening file");

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Error reading line").chars().collect())
        .collect()
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if let ' ' = cell {
        framebuffer.set_foreground_color(Color::WHITESMOKE);
    } else {
        framebuffer.set_foreground_color(Color::PURPLE);
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
