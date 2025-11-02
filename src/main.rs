//main.rs
#![allow(warnings)]

mod command;
mod controls;
mod enemy;
mod fog_of_war;
mod framebuffer;
mod game_screen;
mod game_state;
mod maze;
mod maze_generator;
mod player;
mod ray;
mod raycaster;
mod renderer;
mod sprite;
mod sprite_renderer;
mod ui_renderer;
mod wall_renderer;
mod weapon;
mod weapon_renderer;

use rand::Rng;
use raylib::prelude::*;

use std::time::Duration;
use std::{f32::consts::PI, thread};

use command::PlayerCommand;
use controls::process_input;
use enemy::{spawn_enemies, update_enemies};
use fog_of_war::FogOfWar;
use framebuffer::Framebuffer;
use game_screen::{GameScreen, ScreenRenderer};
use game_state::GameState;
use maze_generator::generate_large_maze;
use player::Player;
use raycaster::{cast_rays, cast_single_ray};
use renderer::Renderer;
use sprite::{process_pickups, spawn_sprites_in_maze};
use sprite_renderer::SpriteRenderer;
use ui_renderer::UIRenderer;
use wall_renderer::WallRenderer;
use weapon::{ShotResult, Weapon};
use weapon_renderer::WeaponRenderer;

const WINDOW_WIDTH: i32 = 1900;
const WINDOW_HEIGHT: i32 = 1000;
const HUD_HEIGHT: i32 = 100;

const FREMEBUFFER_WIDTH: i32 = WINDOW_WIDTH;
const FRAMEBUFFER_HEIGHT: i32 = WINDOW_HEIGHT;
const VIEWPORT_HEIGHT: i32 = WINDOW_HEIGHT - HUD_HEIGHT;

// Game configuration
const NUM_ENEMIES: usize = 50;

fn main() {
    game_loop();
}

fn game_loop() {
    let (mut handle, raylib_thread) = raylib::init()
        .undecorated()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("raylib")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(FREMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT, Color::WHITE);
    let (menu_bg, gameover_bg) = load_background_images(&mut handle, &raylib_thread);
    let screen_renderer = ScreenRenderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, menu_bg, gameover_bg);
    let mut current_screen = GameScreen::MainMenu;

    // Game data (will be reset on restart)
    let mut rng = rand::rng();
    let block_size = 64;

    // Initialize game
    let (mut maze, mut player_pos) = generate_large_maze(block_size);
    let mut player = Player::new(player_pos, 0.0);
    let mut game_state = GameState::new();
    let mut pickup_message = String::new();
    let mut message_timer = 0.0;
    let num_sprites = 25;
    let mut sprites = spawn_sprites_in_maze(&maze, block_size, num_sprites);
    let mut enemies = spawn_enemies(&maze, block_size, player_pos, NUM_ENEMIES);
    let vision_radius = block_size as f32 * 4.0;
    let mut fog_of_war = FogOfWar::new(&maze, block_size, vision_radius);

    let renderer = Renderer::new(block_size);

    // Load wall textures
    let textures = load_wolfenstein_textures(&mut handle, &raylib_thread);
    let wall_renderer = WallRenderer::new(WINDOW_WIDTH, VIEWPORT_HEIGHT, textures);

    // Separate sprite textures from enemy textures
    let sprite_textures = load_sprite_textures(&mut handle, &raylib_thread);
    let enemy_textures = load_enemy_textures(&mut handle, &raylib_thread);
    let sprite_renderer = SpriteRenderer::new(
        WINDOW_WIDTH,
        VIEWPORT_HEIGHT,
        sprite_textures,
        enemy_textures,
    );

    let weapon_textures = load_weapon_textures(&mut handle, &raylib_thread);
    let weapon_renderer = WeaponRenderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, weapon_textures);
    let mut weapon = Weapon::new_machine_gun();

    // Create UI renderer
    let ui_renderer = UIRenderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, HUD_HEIGHT);

    let minimap_size = 200;
    let minimap_x = WINDOW_WIDTH - minimap_size - 20;
    let minimap_y = 20;

    while !&handle.window_should_close() {
        let delta_time = handle.get_frame_time();

        // INPUT PHASE - Poll input before drawing
        let enter_pressed = handle.is_key_pressed(KeyboardKey::KEY_ENTER);

        // UPDATE PHASE (only when playing)
        if current_screen == GameScreen::Playing {
            let input = process_input(&handle);
            for command in input.movement_commands {
                player.execute_command(command, &maze, block_size);
            }

            if input.is_shooting {
                // Check ammo BEFORE firing
                if game_state.ammo() > 0 {
                    if weapon.try_fire() {
                        game_state.use_ammo(1);

                        // Raycast to check what we hit
                        if let Some(ray) = cast_single_ray(&player, &maze, block_size) {
                            // Check if we hit any enemies
                            for enemy in enemies.iter_mut() {
                                if enemy.is_alive() {
                                    let dx = enemy.x() - player.x();
                                    let dy = enemy.y() - player.y();
                                    let angle_to_enemy = dy.atan2(dx);

                                    // Fix angle wrapping issue
                                    let mut angle_diff =
                                        (angle_to_enemy - player.angle_of_view()).abs();
                                    if angle_diff > std::f32::consts::PI {
                                        angle_diff = 2.0 * std::f32::consts::PI - angle_diff;
                                    }

                                    let distance = enemy.distance_to(player.x(), player.y());

                                    // Hit if enemy is close to crosshair and within range
                                    if angle_diff < 0.1 && distance < ray.distance() {
                                        if enemy.take_damage(weapon.damage()) {
                                            game_state.add_kill();
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    weapon.stop_firing();
                }
            } else {
                weapon.stop_firing();
            }

            weapon.update(delta_time);

            // Update enemies and handle damage to player
            let damage_taken = update_enemies(
                &mut enemies,
                delta_time,
                Vector2::new(player.x(), player.y()),
                &maze,
                block_size,
            );

            if damage_taken > 0 {
                game_state.take_damage(damage_taken);
                pickup_message = format!("-{} HP", damage_taken);
                message_timer = 1.0;
            }

            // Check if player died
            if game_state.is_dead() {
                current_screen = GameScreen::GameOver;
            }

            // Process pickups
            let collected = process_pickups(&mut sprites, player.x(), player.y());
            for pickup in collected {
                let message = game_state.collect_pickup(pickup);
                pickup_message = message;
                message_timer = 2.0;
            }

            // Update message timer
            if message_timer > 0.0 {
                message_timer -= delta_time;
            }

            // RENDER TO FRAMEBUFFER
            framebuffer.clear();

            // Render 3D view
            wall_renderer.render_floor_ceiling(&mut framebuffer);

            // Cast rays for raycasting
            let fov = PI / 3.0;
            let num_rays = 320;
            let rays = cast_rays(&player, &maze, block_size, fov, num_rays);

            // Render walls
            wall_renderer.render_3d_view(&mut framebuffer, &rays, &player, block_size);

            // Render sprites (AFTER walls, with depth testing)
            sprite_renderer.render_sprites(&mut framebuffer, &sprites, &player, &rays, block_size);

            // Render enemies (AFTER sprites)
            sprite_renderer.render_enemies(&mut framebuffer, &enemies, &player, &rays, block_size);

            weapon_renderer.render_weapon(&mut framebuffer, &weapon, HUD_HEIGHT);

            // Render HUD at the bottom
            ui_renderer.render_hud(&mut framebuffer, &game_state);

            // Update fog of war
            fog_of_war.update(&player, &rays, &maze);

            // Render minimap
            renderer.render_minimap(
                &mut framebuffer,
                &maze,
                &fog_of_war,
                &player,
                minimap_x,
                minimap_y,
                minimap_size,
            );

            // Render sprites on minimap
            let cell_size = minimap_size / maze[0].len().max(maze.len()) as i32;
            sprite_renderer.render_sprites_minimap(
                &mut framebuffer,
                &sprites,
                minimap_x,
                minimap_y,
                cell_size,
                block_size,
            );

            // Render enemies on minimap
            sprite_renderer.render_enemies_minimap(
                &mut framebuffer,
                &enemies,
                minimap_x,
                minimap_y,
                cell_size,
                block_size,
            );
        }

        // PREPARE TEXTURE BEFORE DRAWING (if playing)
        let game_texture = if current_screen == GameScreen::Playing {
            Some(
                handle
                    .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
                    .expect("The texture loaded from the color buffer should be valid"),
            )
        } else {
            None
        };

        // DRAW PHASE
        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            match current_screen {
                GameScreen::MainMenu => {
                    screen_renderer.render_main_menu(&mut draw_handle);

                    // Check for Enter key using captured value
                    if enter_pressed {
                        // Reset game
                        let (new_maze, new_player_pos) = generate_large_maze(block_size);
                        maze = new_maze;
                        player_pos = new_player_pos;
                        player = Player::new(player_pos, 0.0);
                        game_state = GameState::new();
                        sprites = spawn_sprites_in_maze(&maze, block_size, num_sprites);
                        enemies = spawn_enemies(&maze, block_size, player_pos, NUM_ENEMIES);
                        fog_of_war = FogOfWar::new(&maze, block_size, vision_radius);
                        weapon = Weapon::new_machine_gun();
                        pickup_message = String::new();
                        message_timer = 0.0;

                        current_screen = GameScreen::Playing;
                    }
                }

                GameScreen::Playing => {
                    // Use the pre-loaded texture
                    if let Some(ref texture) = game_texture {
                        draw_handle.draw_texture(texture, 0, 0, Color::LIGHTGRAY);
                    }

                    // Draw pickup message on top of everything
                    if message_timer > 0.0 {
                        let msg_x = WINDOW_WIDTH / 2 - 100;
                        let msg_y = VIEWPORT_HEIGHT / 2;
                        draw_handle.draw_text(&pickup_message, msg_x, msg_y, 30, Color::YELLOW);
                    }
                }

                GameScreen::GameOver => {
                    screen_renderer.render_game_over(
                        &mut draw_handle,
                        game_state.score(),
                        game_state.kills(),
                    );

                    if enter_pressed {
                        current_screen = GameScreen::MainMenu;
                    }
                }
            }
        }
    }
}

fn load_background_images(
    handle: &mut RaylibHandle,
    thread: &RaylibThread,
) -> (Option<Texture2D>, Option<Texture2D>) {
    let menu_bg = Image::load_image("assets/menu_background.png")
        .ok()
        .and_then(|img| handle.load_texture_from_image(thread, &img).ok());

    let gameover_bg = Image::load_image("assets/gameover_background.png")
        .ok()
        .and_then(|img| handle.load_texture_from_image(thread, &img).ok());

    (menu_bg, gameover_bg)
}

fn load_wolfenstein_textures(handle: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Image> {
    vec![
        Image::load_image("assets/level1_default_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco1_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco2_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco3_bright.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_default_dark.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco1_dark.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco2_dark.png").expect("Failed to load texture"),
        Image::load_image("assets/level1_deco3_dark.png").expect("Failed to load texture"),
    ]
}

fn load_sprite_textures(handle: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Image> {
    vec![
        // Decorations (0-3)
        Image::load_image("assets/sprites/deco/barrel.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::BROWN)),
        Image::load_image("assets/sprites/deco/pillar.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GRAY)),
        Image::load_image("assets/sprites/deco/well.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::YELLOW)),
        Image::load_image("assets/sprites/deco/plant.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GREEN)),
        // Pickups (4-7)
        Image::load_image("assets/sprites/pickups/health.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::RED)),
        Image::load_image("assets/sprites/pickups/ammo.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::ORANGE)),
        Image::load_image("assets/sprites/pickups/key.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GOLD)),
        Image::load_image("assets/sprites/pickups/treasure.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::SKYBLUE)),
    ]
}

fn load_enemy_textures(handle: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Image> {
    vec![
        // Rat walk animation (0-3)
        Image::load_image("assets/sprites/rat/walk_1.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::DARKBROWN)),
        Image::load_image("assets/sprites/rat/walk_2.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::DARKBROWN)),
        Image::load_image("assets/sprites/rat/walk_3.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::DARKBROWN)),
        Image::load_image("assets/sprites/rat/walk_4.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::DARKBROWN)),
        // Rat attack animation (4-6)
        Image::load_image("assets/sprites/rat/attack_1.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::RED)),
        Image::load_image("assets/sprites/rat/attack_2.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::RED)),
        Image::load_image("assets/sprites/rat/attack_3.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::RED)),
        // Rat death animation (7-9)
        Image::load_image("assets/sprites/rat/death_1.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GRAY)),
        Image::load_image("assets/sprites/rat/death_2.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GRAY)),
        Image::load_image("assets/sprites/rat/death_3.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::GRAY)),
    ]
}

fn load_weapon_textures(handle: &mut RaylibHandle, thread: &RaylibThread) -> Vec<Image> {
    vec![
        // Machine gun idle (0)
        Image::load_image("assets/sprites/machine_gun/idle.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::DARKGRAY)),
        // Machine gun firing frames (1-3)
        Image::load_image("assets/sprites/machine_gun/shoot_1.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::ORANGE)),
        Image::load_image("assets/sprites/machine_gun/shoot_2.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::YELLOW)),
        Image::load_image("assets/sprites/machine_gun/shoot_3.png")
            .unwrap_or_else(|_| Image::gen_image_color(64, 64, Color::RED)),
    ]
}
