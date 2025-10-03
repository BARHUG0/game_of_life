use raylib::color::Color;
use raylib::texture::ImagePalette;

use crate::framebuffer::{self, Framebuffer};
use crate::maze::Maze;
use crate::player::Player;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let mut distance_from_origin = 0.0;

    framebuffer.set_foreground_color(Color::WHITESMOKE);
    loop {
        let cos = distance_from_origin * player.angle_of_view().cos();
        let sin = distance_from_origin * player.angle_of_view().sin();
        let x = (player.pos().x + cos) as usize;
        let y = (player.pos().y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if maze[j][i] != ' ' {
            break;
        }

        framebuffer.set_pixel(x as i32, y as i32);
        distance_from_origin += 10.0;
    }
}
