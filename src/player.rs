//player.rs
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    position: Vector2,
    angle_of_view: f32,
}

impl Player {
    pub fn new(position: Vector2, angle_of_view: f32) -> Self {
        Player {
            position,
            angle_of_view,
        }
    }

    pub fn position(&self) -> Vector2 {
        self.position
    }

    pub fn angle_of_view(&self) -> f32 {
        self.angle_of_view
    }
}
