use raylib::prelude::*;

pub struct Uniforms {
    time: f32,
    light_direction: Vector3,
    camera_position: Vector3,
}

impl Uniforms {
    pub fn new(time: f32, light_direction: Vector3, camera_position: Vector3) -> Self {
        Uniforms {
            time,
            light_direction,
            camera_position,
        }
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn light_direction(&self) -> Vector3 {
        self.light_direction
    }

    pub fn camera_position(&self) -> Vector3 {
        self.camera_position
    }
}
