//wall_renderer.rs
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::ray::{Ray, Side};

/// Handles 3D wall rendering from raycasting data
pub struct WallRenderer {
    screen_width: i32,
    screen_height: i32,
    textures: Vec<Image>, // Wall textures loaded from files
    texture_width: i32,
    texture_height: i32,
    texture_data: Vec<Vec<Vec<u8>>>, // Cached raw pixel data [texture_idx][y][x*4 RGBA]
}

impl WallRenderer {
    /// Create a new wall renderer
    /// textures: Vec of Images in order [0-3 bright, 4-7 dark variants]
    pub fn new(screen_width: i32, screen_height: i32, mut textures: Vec<Image>) -> Self {
        let texture_width = if !textures.is_empty() {
            textures[0].width
        } else {
            64
        };

        let texture_height = if !textures.is_empty() {
            textures[0].height
        } else {
            64
        };

        // Pre-cache texture data to avoid mutable borrow issues
        let mut texture_data = Vec::new();
        for texture in &mut textures {
            let mut tex_pixels = Vec::new();
            for y in 0..texture.height {
                let mut row = Vec::new();
                for x in 0..texture.width {
                    let color = texture.get_color(x, y);
                    row.push(color.r);
                    row.push(color.g);
                    row.push(color.b);
                    row.push(color.a);
                }
                tex_pixels.push(row);
            }
            texture_data.push(tex_pixels);
        }

        WallRenderer {
            screen_width,
            screen_height,
            textures,
            texture_width,
            texture_height,
            texture_data,
        }
    }

    /// Create wall renderer without textures (uses solid colors)
    pub fn new_untextured(screen_width: i32, screen_height: i32) -> Self {
        WallRenderer {
            screen_width,
            screen_height,
            textures: Vec::new(),
            texture_width: 64,
            texture_height: 64,
            texture_data: Vec::new(),
        }
    }

    /// Render a 3D view from raycasting results
    pub fn render_3d_view(
        &self,
        framebuffer: &mut Framebuffer,
        rays: &[Ray],
        player: &Player,
        block_size: usize,
    ) {
        let num_rays = rays.len();
        if num_rays == 0 {
            return;
        }

        // Calculate width of each vertical strip
        let strip_width = self.screen_width as f32 / num_rays as f32;

        for (i, ray) in rays.iter().enumerate() {
            self.render_wall_strip(framebuffer, ray, i, strip_width, block_size);
        }
    }

    /// Render a single vertical strip of wall
    fn render_wall_strip(
        &self,
        framebuffer: &mut Framebuffer,
        ray: &Ray,
        strip_index: usize,
        strip_width: f32,
        block_size: usize,
    ) {
        // Calculate wall height based on distance
        let distance = ray.distance();
        if distance < 0.1 {
            return; // Too close, skip
        }

        // Calculate wall height (inverse of distance)
        let wall_height = (self.screen_height as f32 * block_size as f32) / distance;

        // Calculate top and bottom of wall slice
        let wall_top = (self.screen_height as f32 / 2.0 - wall_height / 2.0) as i32;
        let wall_bottom = (self.screen_height as f32 / 2.0 + wall_height / 2.0) as i32;

        // Clamp to screen bounds
        let draw_start = wall_top.max(0);
        let draw_end = wall_bottom.min(self.screen_height);

        // Calculate texture X coordinate
        let texture_x = self.calculate_texture_x(ray, block_size);

        // Get texture index based on wall type and side
        let texture_index = self.get_texture_index(ray);

        // Calculate strip bounds
        let strip_start = (strip_index as f32 * strip_width) as i32;
        let strip_end = ((strip_index + 1) as f32 * strip_width).ceil() as i32;

        // Draw the vertical strip
        if self.textures.is_empty() {
            // No textures: draw solid colors
            self.render_untextured_strip(
                framebuffer,
                strip_start,
                strip_end,
                draw_start,
                draw_end,
                ray,
            );
        } else {
            // With textures: sample and draw texture
            self.render_textured_strip(
                framebuffer,
                strip_start,
                strip_end,
                draw_start,
                draw_end,
                wall_top,
                wall_bottom,
                texture_index,
                texture_x,
            );
        }
    }

    /// Calculate which horizontal pixel of the texture to use
    fn calculate_texture_x(&self, ray: &Ray, block_size: usize) -> i32 {
        let hit_x = ray.hit_point().x;
        let hit_y = ray.hit_point().y;

        // Calculate where exactly the wall was hit (0.0 to 1.0)
        let wall_x = match ray.side_hit() {
            Side::Vertical => (hit_y % block_size as f32) / block_size as f32,
            Side::Horizontal => (hit_x % block_size as f32) / block_size as f32,
        };

        // Convert to texture coordinate
        let mut tex_x = (wall_x * self.texture_width as f32) as i32;

        // Flip texture for certain sides to avoid mirroring issues
        if ray.side_hit() == Side::Vertical {
            tex_x = self.texture_width - tex_x - 1;
        }

        tex_x.clamp(0, self.texture_width - 1)
    }

    /// Get the appropriate texture index based on wall type and lighting
    fn get_texture_index(&self, ray: &Ray) -> usize {
        let wall_type = ray.wall_type();

        // Map wall character to base texture (0-3)
        let base_texture = match wall_type {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            _ => 0,
        };

        // Apply Wolfenstein-style lighting based on wall orientation
        let texture_index = match ray.side_hit() {
            Side::Vertical => base_texture,       // Bright variant (0-3)
            Side::Horizontal => base_texture + 4, // Dark variant (4-7)
        };

        texture_index.min(self.textures.len().saturating_sub(1))
    }

    /// Render a textured vertical strip
    fn render_textured_strip(
        &self,
        framebuffer: &mut Framebuffer,
        strip_start: i32,
        strip_end: i32,
        draw_start: i32,
        draw_end: i32,
        wall_top: i32,
        wall_bottom: i32,
        texture_index: usize,
        texture_x: i32,
    ) {
        if texture_index >= self.texture_data.len() {
            return;
        }

        let texture_pixels = &self.texture_data[texture_index];
        let wall_height = wall_bottom - wall_top;

        // Draw each pixel in the strip
        for screen_y in draw_start..draw_end {
            // Calculate texture Y coordinate
            let d = screen_y - wall_top;
            let texture_y =
                ((d * self.texture_height) / wall_height).clamp(0, self.texture_height - 1);

            // Sample texture color from cached data
            let row = &texture_pixels[texture_y as usize];
            let x_offset = (texture_x as usize * 4).min(row.len().saturating_sub(4));

            let color = Color::new(
                row[x_offset],
                row[x_offset + 1],
                row[x_offset + 2],
                row[x_offset + 3],
            );

            // Draw the strip width
            for x in strip_start..strip_end {
                if x >= 0 && x < framebuffer.width() {
                    framebuffer.set_foreground_color(color);
                    framebuffer.set_pixel(x, screen_y);
                }
            }
        }
    }

    /// Render an untextured vertical strip (solid colors based on wall type and side)
    fn render_untextured_strip(
        &self,
        framebuffer: &mut Framebuffer,
        strip_start: i32,
        strip_end: i32,
        draw_start: i32,
        draw_end: i32,
        ray: &Ray,
    ) {
        // Choose color based on wall type
        let base_color = match ray.wall_type() {
            '1' => Color::new(180, 50, 50, 255),  // Red
            '2' => Color::new(50, 180, 50, 255),  // Green
            '3' => Color::new(50, 50, 180, 255),  // Blue
            '4' => Color::new(180, 180, 50, 255), // Yellow
            _ => Color::new(128, 128, 128, 255),  // Gray
        };

        // Apply Wolfenstein-style shading
        let color = match ray.side_hit() {
            Side::Vertical => base_color,
            Side::Horizontal => Color::new(
                base_color.r / 2,
                base_color.g / 2,
                base_color.b / 2,
                base_color.a,
            ),
        };

        framebuffer.set_foreground_color(color);

        // Draw the strip
        for y in draw_start..draw_end {
            for x in strip_start..strip_end {
                if x >= 0 && x < framebuffer.width() {
                    framebuffer.set_pixel(x, y);
                }
            }
        }
    }

    /// Render floor and ceiling (simple flat colors)
    pub fn render_floor_ceiling(&self, framebuffer: &mut Framebuffer) {
        let half_height = self.screen_height / 2;

        // Draw ceiling (top half)
        framebuffer.set_foreground_color(Color::new(60, 60, 60, 255));
        for y in 0..half_height {
            for x in 0..self.screen_width {
                framebuffer.set_pixel(x, y);
            }
        }

        // Draw floor (bottom half)
        framebuffer.set_foreground_color(Color::new(80, 80, 80, 255));
        for y in half_height..self.screen_height {
            for x in 0..self.screen_width {
                framebuffer.set_pixel(x, y);
            }
        }
    }
}
