use raylib::prelude::*;
use std::path::Path;

#[derive(Clone)]
pub struct Texture {
    data: Vec<Vector3>, // Changed to Vector3 for normalized RGB
    width: u32,
    height: u32,
}

impl Texture {
    /// Load a texture from a file path
    pub fn load(path: &str) -> Self {
        let mut image =
            Image::load_image(path).unwrap_or_else(|_| panic!("Failed to load texture: {}", path));

        let width = image.width as u32;
        let height = image.height as u32;

        image.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);

        // Convert to normalized Vector3
        let colors = image.get_image_data();
        let data: Vec<Vector3> = colors
            .iter()
            .map(|c| Vector3::new(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0))
            .collect();

        println!("Loaded texture: {} ({}x{})", path, width, height);

        Texture {
            data,
            width,
            height,
        }
    }

    /// Sample texture at UV coordinates with bilinear filtering
    #[inline(always)]
    pub fn sample(&self, u: f32, v: f32) -> Color {
        // Clamp UV to [0, 1]
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Convert to texture coordinates
        let x = u * (self.width - 1) as f32;
        let y = v * (self.height - 1) as f32;

        // Bilinear interpolation
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = x - x0 as f32;
        let fy = y - y0 as f32;

        let c00 = self.get_pixel(x0, y0);
        let c10 = self.get_pixel(x1, y0);
        let c01 = self.get_pixel(x0, y1);
        let c11 = self.get_pixel(x1, y1);

        self.bilinear_interpolate(c00, c10, c01, c11, fx, fy)
    }

    /// Sample texture with nearest-neighbor (faster, useful for debugging)
    #[inline(always)]
    pub fn sample_nearest(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = (u * (self.width - 1) as f32).round() as u32;
        let y = (v * (self.height - 1) as f32).round() as u32;

        let vec = self.get_pixel(x, y);
        Color::new(
            (vec.x * 255.0) as u8,
            (vec.y * 255.0) as u8,
            (vec.z * 255.0) as u8,
            255,
        )
    }

    /// Sample normal map and convert to tangent-space normal vector
    #[inline(always)]
    pub fn sample_normal(&self, u: f32, v: f32) -> Vector3 {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = (u * (self.width - 1) as f32).round() as u32;
        let y = (v * (self.height - 1) as f32).round() as u32;

        let color = self.get_pixel(x, y);

        // Convert RGB [0, 1] to normal components
        let nx = color.x * 2.0 - 1.0;
        let ny = -(color.y * 2.0 - 1.0);

        // Reconstruct Z to ensure normalized tangent-space normal
        let nz = (1.0 - nx * nx - ny * ny).max(0.0).sqrt();

        Vector3::new(nx, ny, nz).normalized()
    }

    /// Sample specular map and return intensity value (grayscale conversion)
    #[inline(always)]
    pub fn sample_specular(&self, u: f32, v: f32) -> f32 {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = (u * (self.width - 1) as f32).round() as u32;
        let y = (v * (self.height - 1) as f32).round() as u32;

        let color = self.get_pixel(x, y);

        // Convert RGB to grayscale intensity (luminance)
        (color.x + color.y + color.z) / 3.0
    }

    #[inline(always)]
    fn get_pixel(&self, x: u32, y: u32) -> Vector3 {
        let idx = (y * self.width + x) as usize;
        self.data[idx]
    }

    #[inline(always)]
    fn bilinear_interpolate(
        &self,
        c00: Vector3,
        c10: Vector3,
        c01: Vector3,
        c11: Vector3,
        fx: f32,
        fy: f32,
    ) -> Color {
        let r = self.lerp_channel(c00.x, c10.x, c01.x, c11.x, fx, fy);
        let g = self.lerp_channel(c00.y, c10.y, c01.y, c11.y, fx, fy);
        let b = self.lerp_channel(c00.z, c10.z, c01.z, c11.z, fx, fy);

        Color::new(
            (r * 255.0).clamp(0.0, 255.0) as u8,
            (g * 255.0).clamp(0.0, 255.0) as u8,
            (b * 255.0).clamp(0.0, 255.0) as u8,
            255,
        )
    }

    #[inline(always)]
    fn lerp_channel(&self, c00: f32, c10: f32, c01: f32, c11: f32, fx: f32, fy: f32) -> f32 {
        let c0 = c00 * (1.0 - fx) + c10 * fx;
        let c1 = c01 * (1.0 - fx) + c11 * fx;
        c0 * (1.0 - fy) + c1 * fy
    }
}

/// A texture with optional normal and specular maps
#[derive(Clone)]
pub struct TextureWithNormal {
    pub diffuse: Texture,
    pub normal: Option<Texture>,
    pub specular: Option<Texture>, // NEW: Optional specular map
}

impl TextureWithNormal {
    pub fn new(diffuse: Texture, normal: Option<Texture>, specular: Option<Texture>) -> Self {
        TextureWithNormal {
            diffuse,
            normal,
            specular,
        }
    }

    pub fn load(
        diffuse_path: &str,
        normal_path: Option<&str>,
        specular_path: Option<&str>,
    ) -> Self {
        let diffuse = Texture::load(diffuse_path);
        let normal = normal_path.map(|path| Texture::load(path));
        let specular = specular_path.map(|path| Texture::load(path));

        TextureWithNormal {
            diffuse,
            normal,
            specular,
        }
    }
}
