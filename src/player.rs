use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pos: Vector2,
    angle_of_view: f32,
}

impl Player {
    pub fn new(pos: Vector2, angle_of_view: f32) -> Self {
        Player { pos, angle_of_view }
    }

    pub fn pos(&self) -> Vector2 {
        self.pos
    }

    pub fn angle_of_view(&self) -> f32 {
        self.angle_of_view
    }
}
