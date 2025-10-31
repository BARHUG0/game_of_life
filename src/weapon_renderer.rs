//weapon_renderer.rs
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::weapon::Weapon;

pub struct WeaponRenderer {
    screen_width: i32,
    screen_height: i32,
    textures: Vec<Image>,
    texture_width: i32,
    texture_height: i32,
    texture_data: Vec<Vec<Vec<u8>>>,
}

impl WeaponRenderer {
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

        WeaponRenderer {
            screen_width,
            screen_height,
            textures,
            texture_width,
            texture_height,
            texture_data,
        }
    }

    /// Render weapon sprite at bottom center of screen
    pub fn render_weapon(&self, framebuffer: &mut Framebuffer, weapon: &Weapon, hud_height: i32) {
        let texture_index = weapon.texture_index();

        if texture_index >= self.texture_data.len() {
            return;
        }

        let texture = &self.texture_data[texture_index];

        // Scale factor for weapon (make it bigger)
        let scale = 8;

        // Position at bottom-center of viewport (above HUD)
        let weapon_width = self.texture_width * scale;
        let weapon_height = self.texture_height * scale;

        let weapon_x = (self.screen_width - weapon_width) / 2;
        let weapon_y = self.screen_height - hud_height - weapon_height;

        // Render the weapon sprite
        for tex_y in 0..self.texture_height {
            for tex_x in 0..self.texture_width {
                let row = &texture[tex_y as usize];
                let x_offset = (tex_x as usize * 4).min(row.len().saturating_sub(4));

                let r = row[x_offset];
                let g = row[x_offset + 1];
                let b = row[x_offset + 2];
                let a = row[x_offset + 3];

                // Skip transparent pixels (magenta key color and alpha)
                if (r == 0 && g == 255 && b == 255) || a < 128 {
                    continue;
                }

                // Draw scaled pixel
                let color = Color::new(r, g, b, a);
                framebuffer.set_foreground_color(color);

                for dy in 0..scale {
                    for dx in 0..scale {
                        let screen_x = weapon_x + tex_x * scale + dx;
                        let screen_y = weapon_y + tex_y * scale + dy;

                        if screen_x >= 0
                            && screen_x < framebuffer.width()
                            && screen_y >= 0
                            && screen_y < framebuffer.height()
                        {
                            framebuffer.set_pixel(screen_x, screen_y);
                        }
                    }
                }
            }
        }
    }
}
