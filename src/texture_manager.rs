use raylib::prelude::*;

pub struct TextureData {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl TextureData {
    pub fn from_image(image: &mut Image) -> Self {
        let width = image.width as usize;
        let height = image.height as usize;

        let mut pixels = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let color = image.get_color(x as i32, y as i32);
                pixels.push(color);
            }
        }

        TextureData {
            pixels,
            width,
            height,
        }
    }

    /// Sample texture using nearest-neighbor filtering
    /// u, v are in range [0, 1]
    /// Wrapping mode: repeat
    pub fn sample(&self, u: f32, v: f32) -> Color {
        // Wrap coordinates (repeat mode)
        let u_wrapped = u - u.floor();
        let v_wrapped = v - v.floor();

        // Convert to pixel coordinates
        let x = (u_wrapped * self.width as f32) as usize;
        let y = (v_wrapped * self.height as f32) as usize;

        // Clamp to texture bounds (safety)
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        // Return pixel color
        self.pixels[y * self.width + x]
    }

    /// Sample texture with clamping instead of wrapping
    /// Useful for textures that shouldn't repeat
    pub fn sample_clamped(&self, u: f32, v: f32) -> Color {
        // Clamp coordinates to [0, 1]
        let u_clamped = u.max(0.0).min(1.0);
        let v_clamped = v.max(0.0).min(1.0);

        // Convert to pixel coordinates
        let x = (u_clamped * self.width as f32) as usize;
        let y = (v_clamped * self.height as f32) as usize;

        // Clamp to texture bounds
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        self.pixels[y * self.width + x]
    }

    /// Get texture dimensions
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Sample and return as grayscale value [0, 1]
    /// Useful for noise textures used as height/displacement maps
    pub fn sample_grayscale(&self, u: f32, v: f32) -> f32 {
        let color = self.sample(u, v);
        // Convert to grayscale using standard weights
        (color.r as f32 * 0.299 + color.g as f32 * 0.587 + color.b as f32 * 0.114) / 255.0
    }
}
