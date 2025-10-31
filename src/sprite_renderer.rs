//sprite_renderer.rs
use raylib::prelude::*;

use crate::enemy::Enemy;
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
    transform_y: f32,
}

pub struct SpriteRenderer {
    screen_width: i32,
    screen_height: i32,
    textures: Vec<Image>,
    texture_width: i32,
    texture_height: i32,
    texture_data: Vec<Vec<Vec<u8>>>,
}

impl SpriteRenderer {
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

        let mut projections: Vec<(SpriteProjection, usize)> = sprites
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_active())
            .filter_map(|(idx, sprite)| {
                self.project_sprite(sprite, player, block_size)
                    .map(|proj| (proj, idx))
            })
            .collect();

        projections.sort_by(|a, b| b.0.distance.partial_cmp(&a.0.distance).unwrap());

        for (projection, sprite_idx) in projections {
            let sprite = &sprites[sprite_idx];
            self.render_sprite_projection(framebuffer, sprite, &projection, rays);
        }
    }

    fn project_sprite(
        &self,
        sprite: &Sprite,
        player: &Player,
        block_size: usize,
    ) -> Option<SpriteProjection> {
        let sprite_x = sprite.x() - player.x();
        let sprite_y = sprite.y() - player.y();

        let dir_x = player.angle_of_view().cos();
        let dir_y = player.angle_of_view().sin();

        let plane_x = -dir_y * 0.66;
        let plane_y = dir_x * 0.66;

        let inv_det = 1.0 / (plane_x * dir_y - dir_x * plane_y);

        let transform_x = inv_det * (dir_y * sprite_x - dir_x * sprite_y);
        let transform_y = inv_det * (-plane_y * sprite_x + plane_x * sprite_y);

        if transform_y <= 0.5 {
            return None;
        }

        let screen_x = (self.screen_width as f32 / 2.0) * (1.0 + transform_x / transform_y);

        if screen_x < -100.0 || screen_x > self.screen_width as f32 + 100.0 {
            return None;
        }

        let sprite_height =
            ((self.screen_height as f32 * block_size as f32) / transform_y).abs() * sprite.scale();

        let sprite_width = sprite_height;

        Some(SpriteProjection {
            screen_x,
            distance: transform_y,
            sprite_height,
            sprite_width,
            sprite_index: sprite.texture_index(),
            transform_y,
        })
    }

    fn render_sprite_projection(
        &self,
        framebuffer: &mut Framebuffer,
        _sprite: &Sprite,
        projection: &SpriteProjection,
        rays: &[Ray],
    ) {
        if projection.sprite_index >= self.texture_data.len() {
            return;
        }

        let draw_start_x = (-projection.sprite_width / 2.0 + projection.screen_x) as i32;
        let draw_end_x = (projection.sprite_width / 2.0 + projection.screen_x) as i32;
        let draw_start_y =
            (self.screen_height as f32 / 2.0 - projection.sprite_height / 2.0) as i32;
        let draw_end_y = (self.screen_height as f32 / 2.0 + projection.sprite_height / 2.0) as i32;

        let draw_start_x = draw_start_x.max(0);
        let draw_end_x = draw_end_x.min(self.screen_width);
        let draw_start_y = draw_start_y.max(0);
        let draw_end_y = draw_end_y.min(self.screen_height);

        if draw_start_x >= self.screen_width || draw_end_x <= 0 {
            return;
        }

        let texture = &self.texture_data[projection.sprite_index];

        for screen_x in draw_start_x..draw_end_x {
            let ray_index = ((screen_x as f32 / self.screen_width as f32) * rays.len() as f32)
                .max(0.0)
                .min((rays.len() - 1) as f32) as usize;

            if projection.transform_y >= rays[ray_index].distance() + 1.0 {
                continue;
            }

            let d = screen_x as f32 - (projection.screen_x - projection.sprite_width / 2.0);
            let tex_x = (d * self.texture_width as f32 / projection.sprite_width) as i32;

            if tex_x < 0 || tex_x >= self.texture_width {
                continue;
            }

            for screen_y in draw_start_y..draw_end_y {
                let d = screen_y
                    - (self.screen_height as f32 / 2.0 - projection.sprite_height / 2.0) as i32;
                let tex_y =
                    (d as f32 * self.texture_height as f32 / projection.sprite_height) as i32;

                if tex_y < 0 || tex_y >= self.texture_height {
                    continue;
                }

                let row = &texture[tex_y as usize];
                let x_offset = (tex_x as usize * 4).min(row.len().saturating_sub(4));

                let r = row[x_offset];
                let g = row[x_offset + 1];
                let b = row[x_offset + 2];
                let a = row[x_offset + 3];

                if (r == 0 && g == 255 && b == 255) || a < 128 {
                    continue;
                }

                let color = Color::new(r, g, b, a);
                framebuffer.set_foreground_color(color);
                framebuffer.set_pixel(screen_x, screen_y);
            }
        }
    }

    pub fn render_enemies(
        &self,
        framebuffer: &mut Framebuffer,
        enemies: &[Enemy],
        player: &Player,
        rays: &[Ray],
        block_size: usize,
    ) {
        if enemies.is_empty() || rays.is_empty() {
            return;
        }

        let mut projections: Vec<SpriteProjection> = enemies
            .iter()
            .filter(|e| e.is_alive())
            .filter_map(|enemy| {
                let position = enemy.position();
                let texture_index = enemy.texture_index();
                let scale = enemy.scale();

                self.project_at_position(position, texture_index, scale, player, block_size)
            })
            .collect();

        projections.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

        for projection in projections {
            self.render_projection_direct(framebuffer, &projection, rays);
        }
    }

    fn project_at_position(
        &self,
        position: Vector2,
        texture_index: usize,
        scale: f32,
        player: &Player,
        block_size: usize,
    ) -> Option<SpriteProjection> {
        let sprite_x = position.x - player.x();
        let sprite_y = position.y - player.y();

        let dir_x = player.angle_of_view().cos();
        let dir_y = player.angle_of_view().sin();

        let plane_x = -dir_y * 0.66;
        let plane_y = dir_x * 0.66;

        let inv_det = 1.0 / (plane_x * dir_y - dir_x * plane_y);

        let transform_x = inv_det * (dir_y * sprite_x - dir_x * sprite_y);
        let transform_y = inv_det * (-plane_y * sprite_x + plane_x * sprite_y);

        if transform_y <= 0.5 {
            return None;
        }

        let screen_x = (self.screen_width as f32 / 2.0) * (1.0 + transform_x / transform_y);

        if screen_x < -100.0 || screen_x > self.screen_width as f32 + 100.0 {
            return None;
        }

        let sprite_height =
            ((self.screen_height as f32 * block_size as f32) / transform_y).abs() * scale;
        let sprite_width = sprite_height;

        Some(SpriteProjection {
            screen_x,
            distance: transform_y,
            sprite_height,
            sprite_width,
            sprite_index: texture_index,
            transform_y,
        })
    }

    fn render_projection_direct(
        &self,
        framebuffer: &mut Framebuffer,
        projection: &SpriteProjection,
        rays: &[Ray],
    ) {
        if projection.sprite_index >= self.texture_data.len() {
            return;
        }

        let draw_start_x = (-projection.sprite_width / 2.0 + projection.screen_x) as i32;
        let draw_end_x = (projection.sprite_width / 2.0 + projection.screen_x) as i32;
        let draw_start_y =
            (self.screen_height as f32 / 2.0 - projection.sprite_height / 2.0) as i32;
        let draw_end_y = (self.screen_height as f32 / 2.0 + projection.sprite_height / 2.0) as i32;

        let draw_start_x = draw_start_x.max(0);
        let draw_end_x = draw_end_x.min(self.screen_width);
        let draw_start_y = draw_start_y.max(0);
        let draw_end_y = draw_end_y.min(self.screen_height);

        if draw_start_x >= self.screen_width || draw_end_x <= 0 {
            return;
        }

        let texture = &self.texture_data[projection.sprite_index];

        for screen_x in draw_start_x..draw_end_x {
            let ray_index = ((screen_x as f32 / self.screen_width as f32) * rays.len() as f32)
                .max(0.0)
                .min((rays.len() - 1) as f32) as usize;

            if projection.transform_y >= rays[ray_index].distance() + 1.0 {
                continue;
            }

            let d = screen_x as f32 - (projection.screen_x - projection.sprite_width / 2.0);
            let tex_x = (d * self.texture_width as f32 / projection.sprite_width) as i32;

            if tex_x < 0 || tex_x >= self.texture_width {
                continue;
            }

            for screen_y in draw_start_y..draw_end_y {
                let d = screen_y
                    - (self.screen_height as f32 / 2.0 - projection.sprite_height / 2.0) as i32;
                let tex_y =
                    (d as f32 * self.texture_height as f32 / projection.sprite_height) as i32;

                if tex_y < 0 || tex_y >= self.texture_height {
                    continue;
                }

                let row = &texture[tex_y as usize];
                let x_offset = (tex_x as usize * 4).min(row.len().saturating_sub(4));

                let r = row[x_offset];
                let g = row[x_offset + 1];
                let b = row[x_offset + 2];
                let a = row[x_offset + 3];

                if (r == 0 && g == 255 && b == 255) || a < 128 {
                    continue;
                }

                let color = Color::new(r, g, b, a);
                framebuffer.set_foreground_color(color);
                framebuffer.set_pixel(screen_x, screen_y);
            }
        }
    }

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

    pub fn render_enemies_minimap(
        &self,
        framebuffer: &mut Framebuffer,
        enemies: &[Enemy],
        viewport_x: i32,
        viewport_y: i32,
        cell_size: i32,
        block_size: usize,
    ) {
        framebuffer.set_foreground_color(Color::RED);

        for enemy in enemies.iter().filter(|e| e.is_alive()) {
            let grid_x = (enemy.x() / block_size as f32) as i32;
            let grid_y = (enemy.y() / block_size as f32) as i32;

            let x = viewport_x + grid_x * cell_size;
            let y = viewport_y + grid_y * cell_size;

            let size = cell_size / 3;
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
