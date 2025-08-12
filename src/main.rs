#![allow(warnings)]

mod conway;
mod framebuffer;

use rand::Rng;
use raylib::prelude::*;

use std::thread;
use std::time::Duration;

use conway::Cell;
use conway::Matrix;
use framebuffer::Framebuffer;

const MATRIX_WIDTH: usize = WINDOW_WIDTH as usize;
const MATRIX_HEIGHT: usize = WINDOW_HEIGHT as usize;

const WINDOW_WIDTH: i32 = 1900;
const WINDOW_HEIGHT: i32 = 1000;

const FREMEBUFFER_WIDTH: i32 = WINDOW_WIDTH;
const FRAMEBUFFER_HEIGHT: i32 = WINDOW_HEIGHT;
const MATRIX_CELL_SCALLING_FACTOR: usize = 1;

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

    let mut game_of_life = Matrix::new(MATRIX_WIDTH, MATRIX_HEIGHT);

    //Initial values
    //for i in 20..=30 {
    //    game_of_life.set_cell(i, 25, Cell::Alive);
    //}
    //
    let mut rng = rand::rng();

    for i in 0..MATRIX_WIDTH {
        for j in 0..MATRIX_HEIGHT {
            let random_number = rng.random_range(0..100);
            game_of_life.set_cell(
                i,
                j,
                if random_number < 25 {
                    Cell::Dead
                } else {
                    Cell::Alive
                },
            );
        }
    }

    while !&handle.window_should_close() {
        let mut current_cell: Cell;
        let mut current_color: Color;

        let mut fx: usize;
        let mut fy: usize;

        for i in 0..MATRIX_WIDTH {
            for j in 0..MATRIX_HEIGHT {
                current_cell = game_of_life.get_cell(i, j);

                current_color = match current_cell {
                    Cell::Alive => Color::GOLD,
                    Cell::Dead => Color::DARKBLUE,
                };

                framebuffer.set_foreground_color(current_color);

                fx = i * MATRIX_CELL_SCALLING_FACTOR;
                fy = j * MATRIX_CELL_SCALLING_FACTOR;

                for px in fx..fx + MATRIX_CELL_SCALLING_FACTOR {
                    for py in fy..fy + MATRIX_CELL_SCALLING_FACTOR {
                        framebuffer.set_pixel(
                            (framebuffer.width()
                                - MATRIX_WIDTH as i32 * MATRIX_CELL_SCALLING_FACTOR as i32)
                                / 2
                                + px as i32,
                            (framebuffer.height()
                                - MATRIX_HEIGHT as i32 * MATRIX_CELL_SCALLING_FACTOR as i32)
                                / 2
                                + py as i32,
                        );
                    }
                }
            }
        }

        //let mouse_position: raylib::prelude::Vector2 = handle.get_mouse_position();
        //let rectangle = Rectangle::new(85.0, 70.0, 250.0, 100.0);

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            //draw_handle.clear_background(Color::WHITE);

            draw_handle.draw_texture(&texture, 0, 0, Color::LIGHTGRAY);

            //draw_handle.draw_circle_v(mouse_position, 40.0, Color::INDIANRED);

            //draw_handle.gui_button(rectangle, "Hello Word");
        }

        game_of_life = game_of_life.calculate_next_generation();

        thread::sleep(Duration::from_millis(125));
    }
}

/*fn test_matrix() {
    let mut m = Matrix::new(MATRIX_WIDTH, MATRIX_HEIGHT);

    m.set(0, 0, Cell::Alive);
    m.set(0, 1, Cell::Alive);
    m.set(1, 0, Cell::Alive);

    println!("{m}");

    m = m.calculate_next_generation();

    println!("{m}");
}*/
