//game_screen.rs
use raylib::prelude::*;

/// Current game screen state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameScreen {
    MainMenu,
    Playing,
    GameOver,
}

pub struct ScreenRenderer {
    screen_width: i32,
    screen_height: i32,
    menu_background: Option<Texture2D>,
    gameover_background: Option<Texture2D>,
}

impl ScreenRenderer {
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        menu_background: Option<Texture2D>,
        gameover_background: Option<Texture2D>,
    ) -> Self {
        ScreenRenderer {
            screen_width,
            screen_height,
            menu_background,
            gameover_background,
        }
    }

    /// Render the main menu directly to the screen
    pub fn render_main_menu(&self, d: &mut RaylibDrawHandle) {
        // Draw background if available
        if let Some(bg) = &self.menu_background {
            d.draw_texture_pro(
                bg,
                Rectangle::new(0.0, 0.0, bg.width as f32, bg.height as f32),
                Rectangle::new(
                    0.0,
                    0.0,
                    self.screen_width as f32,
                    self.screen_height as f32,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        } else {
            // Fallback: dark background
            d.clear_background(Color::new(20, 20, 30, 255));
        }

        // Semi-transparent overlay for better text readability
        d.draw_rectangle(
            0,
            0,
            self.screen_width,
            self.screen_height,
            Color::new(0, 0, 0, 150),
        );

        // Title
        let title = "WOLFENSTEIN CLONE";
        let title_size = 80;
        let title_width = d.measure_text(title, title_size);
        let title_x = (self.screen_width - title_width) / 2;
        let title_y = self.screen_height / 4;

        // Shadow effect
        d.draw_text(title, title_x + 3, title_y + 3, title_size, Color::BLACK);
        d.draw_text(title, title_x, title_y, title_size, Color::RED);

        // Instructions
        let instructions = vec![
            "CONTROLS:",
            "",
            "W/A/S/D - Move",
            "LEFT/RIGHT ARROWS - Turn",
            "SPACE - Shoot",
            "",
            "GOAL: Collect treasure and survive!",
        ];

        let mut y = self.screen_height / 2 - 80;
        let font_size = 30;

        for line in instructions {
            let line_width = d.measure_text(line, font_size);
            let x = (self.screen_width - line_width) / 2;
            // Shadow
            d.draw_text(line, x + 2, y + 2, font_size, Color::BLACK);
            d.draw_text(line, x, y, font_size, Color::WHITE);
            y += 40;
        }

        // Start prompt
        let prompt = "Press ENTER to start";
        let prompt_size = 40;
        let prompt_width = d.measure_text(prompt, prompt_size);
        let prompt_x = (self.screen_width - prompt_width) / 2;
        let prompt_y = self.screen_height * 3 / 4;

        d.draw_text(
            prompt,
            prompt_x + 2,
            prompt_y + 2,
            prompt_size,
            Color::BLACK,
        );
        d.draw_text(prompt, prompt_x, prompt_y, prompt_size, Color::YELLOW);
    }

    /// Render the game over screen directly to the screen
    pub fn render_game_over(&self, d: &mut RaylibDrawHandle, score: i32, kills: i32) {
        // Draw background if available
        if let Some(bg) = &self.gameover_background {
            d.draw_texture_pro(
                bg,
                Rectangle::new(0.0, 0.0, bg.width as f32, bg.height as f32),
                Rectangle::new(
                    0.0,
                    0.0,
                    self.screen_width as f32,
                    self.screen_height as f32,
                ),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        } else {
            // Fallback: dark red background
            d.clear_background(Color::new(40, 10, 10, 255));
        }

        // Semi-transparent overlay
        d.draw_rectangle(
            0,
            0,
            self.screen_width,
            self.screen_height,
            Color::new(0, 0, 0, 180),
        );

        // Game Over title
        let title = "GAME OVER";
        let title_size = 90;
        let title_width = d.measure_text(title, title_size);
        let title_x = (self.screen_width - title_width) / 2;
        let title_y = self.screen_height / 3;

        d.draw_text(title, title_x + 3, title_y + 3, title_size, Color::BLACK);
        d.draw_text(
            title,
            title_x,
            title_y,
            title_size,
            Color::new(200, 50, 50, 255),
        );

        // Stats
        let font_size = 40;
        let score_text = format!("Final Score: {}", score);
        let kills_text = format!("Enemies Killed: {}", kills);

        let mut y = self.screen_height / 2 + 20;

        // Score
        let score_width = d.measure_text(&score_text, font_size);
        let score_x = (self.screen_width - score_width) / 2;
        d.draw_text(&score_text, score_x + 2, y + 2, font_size, Color::BLACK);
        d.draw_text(&score_text, score_x, y, font_size, Color::ORANGE);

        y += 60;

        // Kills
        let kills_width = d.measure_text(&kills_text, font_size);
        let kills_x = (self.screen_width - kills_width) / 2;
        d.draw_text(&kills_text, kills_x + 2, y + 2, font_size, Color::BLACK);
        d.draw_text(
            &kills_text,
            kills_x,
            y,
            font_size,
            Color::new(200, 200, 200, 255),
        );

        // Restart prompt
        let prompt = "Press ENTER to restart";
        let prompt_size = 35;
        let prompt_width = d.measure_text(prompt, prompt_size);
        let prompt_x = (self.screen_width - prompt_width) / 2;
        let prompt_y = self.screen_height * 3 / 4;

        d.draw_text(
            prompt,
            prompt_x + 2,
            prompt_y + 2,
            prompt_size,
            Color::BLACK,
        );
        d.draw_text(prompt, prompt_x, prompt_y, prompt_size, Color::YELLOW);
    }
}
