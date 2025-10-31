//sprite_renderer.rs
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::ray::Ray;
use crate::sprite::Sprite;

/// Information about a sprite transformed to screen space
#[derive(Debug, Clone)]
struct SpriteProjection {
    screen_x: f32,
    distance: f32,
    sprite_height: f32,
    sprite_width: f32,
    sprite_index: usize,
}

pub struct SpriteRenderer {
    screen_width: i32,
    screen_height: i32,
    textures: Vec<Image>,
    texture_width: i32,
    texture_height: i32,
    texture_data: Vec<Vec<Vec<u8>>>, // Cached pixel data [texture][y][x*4 RGBA]
}

impl SpriteRenderer {
    /// Create a new sprite renderer with textures
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

        // Cache texture data
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

        SpriteRenderer {
            screen_width,
            screen_height,
            textures,
            texture_width,
            texture_height,
            texture_data,
        }
    }

    /// Render all sprites with proper depth sorting
    pub fn render_sprites(
        &self,
        framebuffer: &mut Framebuffer,
        sprites: &[Sprite],
        player: &Player,
        rays: &[Ray],
        block_size: usize,
    ) {
        if sprites.is_empty() || rays.is_empty() {
            return;
        }

        // Step 1: Project all active sprites to screen space
        let mut projections: Vec<(SpriteProjection, usize)> = sprites
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_active())
            .filter_map(|(idx, sprite)| {
                self.project_sprite(sprite, player, block_size)
                    .map(|proj| (proj, idx))
            })
            .collect();

        // Step 2: Sort by distance (farthest first for painter's algorithm)
        projections.sort_by(|a, b| b.0.distance.partial_cmp(&a.0.distance).unwrap());

        // Step 3: Render each sprite with depth testing
        for (projection, sprite_idx) in projections {
            let sprite = &sprites[sprite_idx];
            self.render_sprite_projection(framebuffer, sprite, &projection, rays);
        }
    }

    /// Project a sprite to screen space
    fn project_sprite(
        &self,
        sprite: &Sprite,
        player: &Player,
        block_size: usize,
    ) -> Option<SpriteProjection> {
        // Vector from player to sprite
        let sprite_x = sprite.x() - player.x();
        let sprite_y = sprite.y() - player.y();

        // Transform to camera space (rotate by player's view angle)
        let inv_det = 1.0
            / (player.angle_of_view().cos() * player.angle_of_view().sin()
                - (-player.angle_of_view().sin()) * player.angle_of_view().cos());

        let transform_x = inv_det
            * (player.angle_of_view().sin() * sprite_x - player.angle_of_view().cos() * sprite_y);
        let transform_y = inv_det
            * (-(-player.angle_of_view().sin()) * sprite_x
                + player.angle_of_view().cos() * sprite_y);

        // Sprite is behind player
        if transform_y <= 0.1 {
            return None;
        }

        // Calculate screen X position
        let screen_x = (self.screen_width as f32 / 2.0) * (1.0 + transform_x / transform_y);

        // Calculate sprite height on screen
        let sprite_height =
            ((self.screen_height as f32 * block_size as f32) / transform_y).abs() * sprite.scale();

        // Sprite width (assuming square sprites)
        let sprite_width = sprite_height;

        Some(SpriteProjection {
            screen_x,
            distance: transform_y,
            sprite_height,
            sprite_width,
            sprite_index: sprite.texture_index(),
        })
    }

    /// Render a single projected sprite with depth testing
    fn render_sprite_projection(
        &self,
        framebuffer: &mut Framebuffer,
        sprite: &Sprite,
        projection: &SpriteProjection,
        rays: &[Ray],
    ) {
        if projection.sprite_index >= self.texture_data.len() {
            return;
        }

        // Calculate sprite bounds on screen
        let draw_start_x = (-projection.sprite_width / 2.0 + projection.screen_x) as i32;
        let draw_end_x = (projection.sprite_width / 2.0 + projection.screen_x) as i32;
        let draw_start_y =
            (self.screen_height as f32 / 2.0 - projection.sprite_height / 2.0) as i32;
        let draw_end_y = (self.screen_height as f32 / 2.0 + projection.sprite_height / 2.0) as i32;

        // Clamp to screen bounds
        let draw_start_x = draw_start_x.max(0);
        let draw_end_x = draw_end_x.min(self.screen_width);
        let draw_start_y = draw_start_y.max(0);
        let draw_end_y = draw_end_y.min(self.screen_height);

        let texture = &self.texture_data[projection.sprite_index];

        // Draw sprite column by column
        for screen_x in draw_start_x..draw_end_x {
            // Depth test: check if sprite is closer than wall at this column
            let ray_index =
                (screen_x as f32 / self.screen_width as f32 * rays.len() as f32) as usize;
            if ray_index >= rays.len() {
                continue;
            }

            if projection.distance >= rays[ray_index].distance() {
                continue; // Wall is closer, skip this column
            }

            // Calculate texture X coordinate
            let tex_x = ((screen_x as f32 - (projection.screen_x - projection.sprite_width / 2.0))
                * self.texture_width as f32
                / projection.sprite_width) as i32;

            let tex_x = tex_x.clamp(0, self.texture_width - 1);

            // Draw sprite column
            for screen_y in draw_start_y..draw_end_y {
                // Calculate texture Y coordinate
                let d = screen_y
                    - (self.screen_height as f32 / 2.0 - projection.sprite_height / 2.0) as i32;
                let tex_y =
                    (d as f32 * self.texture_height as f32 / projection.sprite_height) as i32;

                let tex_y = tex_y.clamp(0, self.texture_height - 1);

                // Sample texture
                let row = &texture[tex_y as usize];
                let x_offset = (tex_x as usize * 4).min(row.len().saturating_sub(4));

                let r = row[x_offset];
                let g = row[x_offset + 1];
                let b = row[x_offset + 2];
                let a = row[x_offset + 3];

                // Skip transparent pixels (color key or alpha)
                // Common Wolfenstein color key: cyan (0, 255, 255)
                if (r == 0 && g == 255 && b == 255) || a < 128 {
                    continue;
                }

                let color = Color::new(r, g, b, a);
                framebuffer.set_foreground_color(color);
                framebuffer.set_pixel(screen_x, screen_y);
            }
        }
    }

    /// Render sprites on the minimap (top-down view)
    pub fn render_sprites_minimap(
        &self,
        framebuffer: &mut Framebuffer,
        sprites: &[Sprite],
        viewport_x: i32,
        viewport_y: i32,
        cell_size: i32,
        block_size: usize,
    ) {
        framebuffer.set_foreground_color(Color::ORANGE);

        for sprite in sprites.iter().filter(|s| s.is_active()) {
            let grid_x = (sprite.x() / block_size as f32) as i32;
            let grid_y = (sprite.y() / block_size as f32) as i32;

            let x = viewport_x + grid_x * cell_size;
            let y = viewport_y + grid_y * cell_size;

            // Draw a small dot for each sprite
            let size = cell_size / 4;
            for px in x..x + size {
                for py in y..y + size {
                    if px >= 0 && px < framebuffer.width() && py >= 0 && py < framebuffer.height() {
                        framebuffer.set_pixel(px, py);
                    }
                }
            }
        }
    }
}
