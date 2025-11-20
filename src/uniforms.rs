use crate::texture_manager::TextureData;
use raylib::prelude::*;

pub struct Uniforms<'a> {
    time: f32,
    light_direction: Vector3,
    camera_position: Vector3,
    texture: Option<&'a TextureData>,
}

impl<'a> Uniforms<'a> {
    pub fn new(time: f32, light_direction: Vector3, camera_position: Vector3) -> Self {
        Uniforms {
            time,
            light_direction,
            camera_position,
            texture: None,
        }
    }

    pub fn new_with_texture(
        time: f32,
        light_direction: Vector3,
        camera_position: Vector3,
        texture: &'a TextureData,
    ) -> Self {
        Uniforms {
            time,
            light_direction,
            camera_position,
            texture: Some(texture),
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

    pub fn texture(&self) -> Option<&TextureData> {
        self.texture
    }
}
