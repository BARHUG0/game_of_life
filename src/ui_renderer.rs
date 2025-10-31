//ui_renderer.rs
use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::game_state::GameState;

pub struct UIRenderer {
    hud_height: i32,
    screen_width: i32,
    screen_height: i32,
}

impl UIRenderer {
    pub fn new(screen_width: i32, screen_height: i32, hud_height: i32) -> Self {
        UIRenderer {
            hud_height,
            screen_width,
            screen_height,
        }
    }

    pub fn hud_height(&self) -> i32 {
        self.hud_height
    }

    /// Render the complete HUD bar at the bottom of the screen
    pub fn render_hud(&self, framebuffer: &mut Framebuffer, game_state: &GameState) {
        let hud_y = self.screen_height - self.hud_height;

        // Draw HUD background
        self.draw_hud_background(framebuffer, hud_y);

        // Draw dividing lines
        self.draw_dividers(framebuffer, hud_y);

        // Draw stats in sections
        self.draw_score_section(framebuffer, hud_y, game_state);
        self.draw_health_section(framebuffer, hud_y, game_state);
        self.draw_ammo_section(framebuffer, hud_y, game_state);
        self.draw_keys_section(framebuffer, hud_y, game_state);
    }

    /// Draw the dark background for the HUD
    fn draw_hud_background(&self, framebuffer: &mut Framebuffer, hud_y: i32) {
        framebuffer.set_foreground_color(Color::new(40, 40, 40, 255));

        for y in hud_y..self.screen_height {
            for x in 0..self.screen_width {
                framebuffer.set_pixel(x, y);
            }
        }
    }

    /// Draw vertical dividing lines between sections
    fn draw_dividers(&self, framebuffer: &mut Framebuffer, hud_y: i32) {
        framebuffer.set_foreground_color(Color::new(80, 80, 80, 255));

        let section_width = self.screen_width / 4;

        for i in 1..4 {
            let x = section_width * i;
            for y in hud_y..self.screen_height {
                framebuffer.set_pixel(x, y);
                framebuffer.set_pixel(x + 1, y);
            }
        }
    }

    /// Draw score in the leftmost section
    fn draw_score_section(
        &self,
        framebuffer: &mut Framebuffer,
        hud_y: i32,
        game_state: &GameState,
    ) {
        let section_width = self.screen_width / 4;
        let center_x = section_width / 2;

        // Label
        self.draw_small_text(framebuffer, "SCORE", center_x - 25, hud_y + 10, Color::GRAY);

        // Value
        let score_text = format!("{}", game_state.score());
        self.draw_large_text(
            framebuffer,
            &score_text,
            center_x - 30,
            hud_y + 35,
            Color::YELLOW,
        );
    }

    /// Draw health with colored indicator
    fn draw_health_section(
        &self,
        framebuffer: &mut Framebuffer,
        hud_y: i32,
        game_state: &GameState,
    ) {
        let section_width = self.screen_width / 4;
        let center_x = section_width + section_width / 2;

        // Label
        self.draw_small_text(
            framebuffer,
            "HEALTH",
            center_x - 30,
            hud_y + 10,
            Color::GRAY,
        );

        // Health bar background
        let bar_width = 80;
        let bar_height = 20;
        let bar_x = center_x - bar_width / 2;
        let bar_y = hud_y + 35;

        // Background (empty bar)
        framebuffer.set_foreground_color(Color::new(60, 20, 20, 255));
        for y in bar_y..bar_y + bar_height {
            for x in bar_x..bar_x + bar_width {
                framebuffer.set_pixel(x, y);
            }
        }

        // Health fill
        let health_percent = game_state.health() as f32 / game_state.max_health() as f32;
        let fill_width = (bar_width as f32 * health_percent) as i32;

        let health_color = if health_percent > 0.6 {
            Color::new(0, 200, 0, 255)
        } else if health_percent > 0.3 {
            Color::new(200, 200, 0, 255)
        } else {
            Color::new(200, 0, 0, 255)
        };

        framebuffer.set_foreground_color(health_color);
        for y in bar_y..bar_y + bar_height {
            for x in bar_x..bar_x + fill_width {
                framebuffer.set_pixel(x, y);
            }
        }

        // Health number
        let health_text = format!("{}", game_state.health());
        self.draw_medium_text(
            framebuffer,
            &health_text,
            center_x - 15,
            bar_y + 3,
            Color::WHITE,
        );
    }

    /// Draw ammo counter
    fn draw_ammo_section(&self, framebuffer: &mut Framebuffer, hud_y: i32, game_state: &GameState) {
        let section_width = self.screen_width / 4;
        let center_x = section_width * 2 + section_width / 2;

        // Label
        self.draw_small_text(framebuffer, "AMMO", center_x - 20, hud_y + 10, Color::GRAY);

        // Value
        let ammo_text = format!("{}", game_state.ammo());
        self.draw_large_text(
            framebuffer,
            &ammo_text,
            center_x - 30,
            hud_y + 35,
            Color::ORANGE,
        );
    }

    /// Draw keys and treasure indicators
    fn draw_keys_section(&self, framebuffer: &mut Framebuffer, hud_y: i32, game_state: &GameState) {
        let section_width = self.screen_width / 4;
        let center_x = section_width * 3 + section_width / 2;

        // Keys
        self.draw_small_text(framebuffer, "KEYS", center_x - 20, hud_y + 10, Color::GRAY);

        let keys = game_state.keys();
        let key_size = 15;
        let key_spacing = 20;
        let start_x = center_x - (keys.min(3) * key_spacing) / 2;

        for i in 0..keys.min(3) {
            let key_x = start_x + i * key_spacing;
            let key_y = hud_y + 35;

            // Draw simple key icon (colored square)
            framebuffer.set_foreground_color(Color::GOLD);
            for y in key_y..key_y + key_size {
                for x in key_x..key_x + key_size {
                    framebuffer.set_pixel(x, y);
                }
            }
        }

        // Treasure count
        if game_state.treasure() > 0 {
            let treasure_text = format!("${}", game_state.treasure());
            self.draw_small_text(
                framebuffer,
                &treasure_text,
                center_x - 20,
                hud_y + 65,
                Color::SKYBLUE,
            );
        }
    }

    /// Draw small text (label size)
    fn draw_small_text(
        &self,
        framebuffer: &mut Framebuffer,
        text: &str,
        x: i32,
        y: i32,
        color: Color,
    ) {
        framebuffer.set_foreground_color(color);
        self.draw_simple_text(framebuffer, text, x, y, 1);
    }

    /// Draw medium text
    fn draw_medium_text(
        &self,
        framebuffer: &mut Framebuffer,
        text: &str,
        x: i32,
        y: i32,
        color: Color,
    ) {
        framebuffer.set_foreground_color(color);
        self.draw_simple_text(framebuffer, text, x, y, 2);
    }

    /// Draw large text (numbers)
    fn draw_large_text(
        &self,
        framebuffer: &mut Framebuffer,
        text: &str,
        x: i32,
        y: i32,
        color: Color,
    ) {
        framebuffer.set_foreground_color(color);
        self.draw_simple_text(framebuffer, text, x, y, 3);
    }

    /// Simple pixel-based text rendering (very basic, blocky style)
    fn draw_simple_text(
        &self,
        framebuffer: &mut Framebuffer,
        text: &str,
        x: i32,
        y: i32,
        scale: i32,
    ) {
        let mut cursor_x = x;

        for ch in text.chars() {
            self.draw_char(framebuffer, ch, cursor_x, y, scale);
            cursor_x += (6 * scale) + scale; // Character width + spacing
        }
    }

    /// Draw a single character using a simple 5x7 pixel font
    fn draw_char(&self, framebuffer: &mut Framebuffer, ch: char, x: i32, y: i32, scale: i32) {
        let pattern = self.get_char_pattern(ch);

        for (row_idx, row) in pattern.iter().enumerate() {
            for col_idx in 0..5 {
                if (row >> (4 - col_idx)) & 1 == 1 {
                    // Draw scaled pixel
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let px = x + col_idx * scale + dx;
                            let py = y + row_idx as i32 * scale + dy;
                            if px >= 0
                                && px < framebuffer.width()
                                && py >= 0
                                && py < framebuffer.height()
                            {
                                framebuffer.set_pixel(px, py);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get 5x7 bitmap pattern for a character (very simplified font)
    fn get_char_pattern(&self, ch: char) -> [u8; 7] {
        match ch {
            '0' => [
                0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
            ],
            '1' => [
                0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
            ],
            '2' => [
                0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
            ],
            '3' => [
                0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110,
            ],
            '4' => [
                0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
            ],
            '5' => [
                0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
            ],
            '6' => [
                0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
            ],
            '7' => [
                0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
            ],
            '8' => [
                0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
            ],
            '9' => [
                0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
            ],
            'A' => [
                0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
            ],
            'B' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
            ],
            'C' => [
                0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
            ],
            'D' => [
                0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
            ],
            'E' => [
                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
            ],
            'F' => [
                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
            ],
            'G' => [
                0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
            ],
            'H' => [
                0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
            ],
            'I' => [
                0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
            ],
            'K' => [
                0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
            ],
            'L' => [
                0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
            ],
            'M' => [
                0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
            ],
            'N' => [
                0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001,
            ],
            'O' => [
                0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
            ],
            'R' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
            ],
            'S' => [
                0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
            ],
            'T' => [
                0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
            ],
            'U' => [
                0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
            ],
            'Y' => [
                0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100,
            ],
            '$' => [
                0b00100, 0b01111, 0b10100, 0b01110, 0b00101, 0b11110, 0b00100,
            ],
            _ => [
                0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
            ],
        }
    }
}
