use crate::texture_manager::TextureData;
use raylib::prelude::*;

pub struct Uniforms<'a> {
    time: f32,
    light_direction: Vector3,
    camera_position: Vector3,
    texture_0: Option<&'a TextureData>,
    texture_1: Option<&'a TextureData>,
    texture_2: Option<&'a TextureData>,
}

impl<'a> Uniforms<'a> {
    pub fn new(time: f32, light_direction: Vector3, camera_position: Vector3) -> Self {
        Uniforms {
            time,
            light_direction,
            camera_position,
            texture_0: None,
            texture_1: None,
            texture_2: None,
        }
    }

    pub fn new_with_textures(
        time: f32,
        light_direction: Vector3,
        camera_position: Vector3,
        texture_0: Option<&'a TextureData>,
        texture_1: Option<&'a TextureData>,
        texture_2: Option<&'a TextureData>,
    ) -> Self {
        Uniforms {
            time,
            light_direction,
            camera_position,
            texture_0,
            texture_1,
            texture_2,
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

    // Legacy method for backward compatibility
    pub fn texture(&self) -> Option<&TextureData> {
        self.texture_0
    }

    pub fn texture_0(&self) -> Option<&TextureData> {
        self.texture_0
    }

    pub fn texture_1(&self) -> Option<&TextureData> {
        self.texture_1
    }

    pub fn texture_2(&self) -> Option<&TextureData> {
        self.texture_2
    }
}
