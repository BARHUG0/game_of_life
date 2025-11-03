//renderer.rs
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::ray::Ray;

pub struct Renderer {
    block_size: usize,
}

impl Renderer {
    pub fn new(block_size: usize) -> Self {
        Renderer { block_size }
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
    }

    /// Renders the full maze in top-down view
    pub fn render_maze(&self, framebuffer: &mut Framebuffer, maze: &Maze) {
        for (row_index, row) in maze.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                let xo = col_index * self.block_size;
                let yo = row_index * self.block_size;
                self.draw_cell(framebuffer, xo, yo, cell);
            }
        }
    }

    /// Renders the player as a square in top-down view
    pub fn render_player(&self, framebuffer: &mut Framebuffer, player: &Player) {
        framebuffer.set_foreground_color(Color::GREENYELLOW);

        let size = 5;
        let cx = player.x() as i32;
        let cy = player.y() as i32;

        for x in cx - size..=cx + size {
            for y in cy - size..=cy + size {
                framebuffer.set_pixel(x, y);
            }
        }
    }

    /// Renders debug rays (the line from player to wall hit)
    pub fn render_debug_ray(&self, framebuffer: &mut Framebuffer, player: &Player, ray: &Ray) {
        framebuffer.set_foreground_color(Color::GREENYELLOW);

        let x0 = player.x() as i32;
        let y0 = player.y() as i32;
        let x1 = ray.hit_point().x as i32;
        let y1 = ray.hit_point().y as i32;

        framebuffer.line(x0, y0, x1, y1);
    }

    /// Renders multiple debug rays
    pub fn render_debug_rays(&self, framebuffer: &mut Framebuffer, player: &Player, rays: &[Ray]) {
        for ray in rays {
            self.render_debug_ray(framebuffer, player, ray);
        }
    }

    /// Renders a minimap with fog of war
    /// viewport_x, viewport_y: top-left corner of minimap on screen
    /// viewport_size: size of the minimap in pixels
    pub fn render_minimap(
        &self,
        framebuffer: &mut Framebuffer,
        maze: &Maze,
        fog_of_war: &crate::fog_of_war::FogOfWar,
        player: &Player,
        viewport_x: i32,
        viewport_y: i32,
        viewport_size: i32,
    ) {
        let cell_size = viewport_size / maze[0].len().max(maze.len()) as i32;

        // Render each cell
        for (row_index, row) in maze.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                let x = viewport_x + (col_index as i32 * cell_size);
                let y = viewport_y + (row_index as i32 * cell_size);

                // Check if explored
                if fog_of_war.is_explored(col_index, row_index) {
                    // Render explored areas normally
                    match cell {
                        ' ' => framebuffer.set_foreground_color(Color::new(220, 220, 220, 255)),
                        'E' => framebuffer.set_foreground_color(Color::new(255, 215, 0, 255)),
                        _ => framebuffer.set_foreground_color(Color::new(100, 50, 150, 255)),
                    }
                } else {
                    // Render unexplored areas as dark
                    framebuffer.set_foreground_color(Color::new(20, 20, 20, 255));
                }

                // Draw the cell
                for px in x..x + cell_size {
                    for py in y..y + cell_size {
                        if px < framebuffer.width() && py < framebuffer.height() {
                            framebuffer.set_pixel(px, py);
                        }
                    }
                }
            }
        }

        // Draw player position on minimap
        let player_grid_x = (player.x() / self.block_size as f32) as usize;
        let player_grid_y = (player.y() / self.block_size as f32) as usize;

        let player_x = viewport_x + (player_grid_x as i32 * cell_size);
        let player_y = viewport_y + (player_grid_y as i32 * cell_size);

        framebuffer.set_foreground_color(Color::YELLOW);
        let size = cell_size / 3;
        for px in player_x..player_x + size {
            for py in player_y..player_y + size {
                if px < framebuffer.width() && py < framebuffer.height() {
                    framebuffer.set_pixel(px, py);
                }
            }
        }
    }

    /// Helper: draws a single cell at position (xo, yo)
    fn draw_cell(&self, framebuffer: &mut Framebuffer, xo: usize, yo: usize, cell: char) {
        match cell {
            ' ' => framebuffer.set_foreground_color(Color::WHITESMOKE),
            _ => framebuffer.set_foreground_color(Color::PURPLE),
        }

        let xf = if xo + self.block_size < framebuffer.width() as usize {
            xo + self.block_size
        } else {
            framebuffer.width() as usize
        };

        let yf = if yo + self.block_size < framebuffer.height() as usize {
            yo + self.block_size
        } else {
            framebuffer.height() as usize
        };

        for x in xo..xf {
            for y in yo..yf {
                framebuffer.set_pixel(x as i32, y as i32);
            }
        }
    }
}
