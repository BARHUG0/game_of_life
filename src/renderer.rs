use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::intersection::Intersect;
use crate::light::Light;
use crate::objects::{Object, RayIntersect};
use raylib::prelude::*;

const BACKGROUND_COLOR: Color = Color::new(4, 12, 36, 255);
const SHADOW_BIAS: f32 = 0.001;
const MAX_RECURSION_DEPTH: u32 = 4;
const LIGHT_ATTENUTATION: f32 = 0.7;

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Object],
    camera: &Camera,
    lights: &[Light],
) {
    let width = framebuffer.width() as f32;
    let height = framebuffer.height() as f32;

    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan() as f32;

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0);

            framebuffer.set_foreground_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Object],
    lights: &[Light],
    depth: u32,
) -> Color {
    if depth >= MAX_RECURSION_DEPTH {
        return BACKGROUND_COLOR;
    }

    let mut intersection = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting() && tmp.distance() < zbuffer {
            zbuffer = tmp.distance;
            intersection = tmp;
        }
    }

    if !intersection.is_intersecting {
        return BACKGROUND_COLOR;
    }

    let material = intersection.material();

    // Determine if ray is entering (front face) or exiting (back face)
    let is_entering = intersection.normal.dot(*ray_direction) < 0.0;

    // Ensure normal always points against the ray direction
    let outward_normal = if is_entering {
        intersection.normal
    } else {
        -intersection.normal
    };

    let view_dir = ray_direction.scale_by(-1.0);

    // In cast_ray(), for emissive materials:
    let surface_color = if material.emission_strength > 0.5 {
        // Emissive objects use their diffuse color directly (no external lighting needed)
        material.diffuse
    } else {
        // Normal objects get full lighting calculation
        calculate_lighting(&intersection, &outward_normal, &view_dir, lights, objects)
    };

    // For opaque materials, handle only reflection
    if material.transparency < 0.01 {
        if material.reflectivity > 0.0 {
            let reflect_dir = reflect(&ray_direction, &outward_normal);
            let reflect_origin = intersection.point + outward_normal * SHADOW_BIAS;
            let reflection_color =
                cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1);
            return color_blend(surface_color, reflection_color, material.reflectivity);
        }
        return surface_color;
    }

    // For transparent materials, handle both reflection and refraction
    let (n1, n2) = if is_entering {
        (1.0, material.refractive_index) // Air to material
    } else {
        (material.refractive_index, 1.0) // Material to air
    };

    // Calculate Fresnel coefficient (determines reflection vs refraction ratio)
    let kr = fresnel(&ray_direction, &outward_normal, n1, n2);

    let mut final_color = surface_color;

    // Add reflection component
    if kr > 0.0 {
        let reflect_dir = reflect(&ray_direction, &outward_normal);
        let reflect_origin = intersection.point + outward_normal * SHADOW_BIAS;
        let reflection_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1);

        final_color =
            color_blend_weighted(final_color, reflection_color, kr * material.transparency);
    }

    // Add refraction component (if not total internal reflection)
    let kt = 1.0 - kr; // Transmission coefficient
    if kt > 0.0 {
        if let Some(refract_dir) = refract(&ray_direction, &outward_normal, n1, n2) {
            // For entering: push ray inward (subtract bias)
            // For exiting: push ray outward (add bias)
            let refract_origin = if is_entering {
                intersection.point - outward_normal * SHADOW_BIAS
            } else {
                intersection.point + outward_normal * SHADOW_BIAS
            };

            let refraction_color =
                cast_ray(&refract_origin, &refract_dir, objects, lights, depth + 1);
            final_color =
                color_blend_weighted(final_color, refraction_color, kt * material.transparency);
        }
    }

    // Add emission (always additive, independent of lighting)
    if material.emission_strength > 0.0 {
        let emission_color = color_multiply(material.emission, material.emission_strength);
        final_color = color_add(final_color, emission_color);
    }

    final_color
}

fn calculate_lighting(
    intersection: &Intersect,
    normal: &Vector3,
    view_dir: &Vector3,
    lights: &[Light],
    objects: &[Object],
) -> Color {
    let material = intersection.material();

    // Ambient
    let ambient = 0.1;
    let mut diffuse_color = color_multiply(material.diffuse, ambient * material.albedo[0]);
    let mut specular_color = Color::new(0, 0, 0, 255);

    // 1. Process regular lights
    for light in lights {
        let light_dir = (light.position - intersection.point).normalized();
        let light_distance = (light.position - intersection.point).length();
        let shadow_origin = intersection.point + *normal * SHADOW_BIAS;

        if !cast_shadow_ray(&shadow_origin, &light_dir, light_distance, objects) {
            let light_intensity = light.get_intensity_at(&intersection.point);

            // Diffuse
            let diffuse_intensity = normal.dot(light_dir).max(0.0) * light_intensity;
            let diffuse_contribution =
                color_multiply(light.color, diffuse_intensity * material.albedo[0]);
            diffuse_color = color_add(diffuse_color, diffuse_contribution);

            // Specular
            let reflect_dir = reflect(&light_dir, normal);
            let specular_intensity =
                view_dir.dot(reflect_dir).max(0.0).powf(material.specular) * light_intensity;
            let specular_contribution =
                color_multiply(light.color, specular_intensity * material.albedo[1]);
            specular_color = color_add(specular_color, specular_contribution);
        }
    }

    // 2. Process emissive objects as light sources
    for (idx, object) in objects.iter().enumerate() {
        let obj_material = match object {
            Object::Sphere(sphere) => sphere.material(),
            Object::Cube(_) => continue,
        };

        if obj_material.emission_strength < 0.5 {
            continue;
        }

        let emissive_pos = match object {
            Object::Sphere(sphere) => sphere.center(),
            Object::Cube(_) => continue,
        };

        let light_dir = (emissive_pos - intersection.point).normalized();
        let light_distance = (emissive_pos - intersection.point).length();
        let shadow_origin = intersection.point + *normal * SHADOW_BIAS;

        // KEY FIX: Check shadows but exclude this specific emissive object
        if !cast_shadow_ray_excluding(&shadow_origin, &light_dir, light_distance, objects, idx) {
            let base_intensity = obj_material.emission_strength;
            let light_intensity =
                base_intensity / (1.0 + light_distance * light_distance * LIGHT_ATTENUTATION);

            // Diffuse from emissive object
            let diffuse_intensity = normal.dot(light_dir).max(0.0) * light_intensity;
            let diffuse_contribution = color_multiply(
                obj_material.emission,
                diffuse_intensity * material.albedo[0],
            );
            diffuse_color = color_add(diffuse_color, diffuse_contribution);

            // Specular from emissive object
            let reflect_dir = reflect(&light_dir, normal);
            let specular_intensity =
                view_dir.dot(reflect_dir).max(0.0).powf(material.specular) * light_intensity;
            let specular_contribution = color_multiply(
                obj_material.emission,
                specular_intensity * material.albedo[1],
            );
            specular_color = color_add(specular_color, specular_contribution);
        }
    }

    color_add(diffuse_color, specular_color)
}

fn cast_shadow_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    light_distance: f32,
    objects: &[Object],
) -> bool {
    for object in objects {
        let intersection = object.ray_intersect(ray_origin, ray_direction);
        // Ignore transparent objects in shadow calculation (light passes through)
        if intersection.is_intersecting()
            && intersection.distance() < light_distance
            && intersection.material().transparency < 0.5
        {
            return true;
        }
    }
    false
}

// Add this new function below cast_shadow_ray:
fn cast_shadow_ray_excluding(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    light_distance: f32,
    objects: &[Object],
    exclude_idx: usize,
) -> bool {
    for (idx, object) in objects.iter().enumerate() {
        if idx == exclude_idx {
            continue; // Skip the emissive object itself
        }

        let intersection = object.ray_intersect(ray_origin, ray_direction);
        if intersection.is_intersecting()
            && intersection.distance() < light_distance
            && intersection.material().transparency < 0.5
        {
            return true;
        }
    }
    false
}

// Snell's Law: Calculate refraction direction
fn refract(incident: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> Option<Vector3> {
    let eta = n1 / n2;
    let cos_i = -normal.dot(*incident);
    let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);

    // Total internal reflection occurs
    if sin_t2 > 1.0 {
        return None;
    }

    let cos_t = (1.0 - sin_t2).sqrt();
    Some(*incident * eta + *normal * (eta * cos_i - cos_t))
}

// Fresnel equations: Calculate reflection coefficient using Schlick's approximation
fn fresnel(incident: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> f32 {
    let mut cos_i = -normal.dot(*incident).max(-1.0).min(1.0);

    let (eta_i, eta_t) = (n1, n2);

    // Check for total internal reflection
    let sin_t = (eta_i / eta_t) * (1.0 - cos_i * cos_i).max(0.0).sqrt();

    if sin_t >= 1.0 {
        return 1.0; // Total internal reflection
    }

    let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();

    // Prevent division by zero
    let denom_s = (eta_t * cos_i) + (eta_i * cos_t);
    let denom_p = (eta_i * cos_i) + (eta_t * cos_t);

    if denom_s.abs() < 1e-6 || denom_p.abs() < 1e-6 {
        return 1.0;
    }

    let rs = ((eta_t * cos_i) - (eta_i * cos_t)) / denom_s;
    let rp = ((eta_i * cos_i) - (eta_t * cos_t)) / denom_p;

    ((rs * rs + rp * rp) / 2.0).max(0.0).min(1.0)
}

fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * normal.dot(*incident)
}

fn color_multiply(color: Color, intensity: f32) -> Color {
    Color::new(
        (color.r as f32 * intensity).min(255.0).max(0.0) as u8,
        (color.g as f32 * intensity).min(255.0).max(0.0) as u8,
        (color.b as f32 * intensity).min(255.0).max(0.0) as u8,
        color.a,
    )
}

fn color_add(a: Color, b: Color) -> Color {
    Color::new(
        (a.r as u16 + b.r as u16).min(255) as u8,
        (a.g as u16 + b.g as u16).min(255) as u8,
        (a.b as u16 + b.b as u16).min(255) as u8,
        a.a,
    )
}

fn color_blend(surface: Color, other: Color, weight: f32) -> Color {
    let surface_weight = 1.0 - weight;
    Color::new(
        ((surface.r as f32 * surface_weight + other.r as f32 * weight).min(255.0)) as u8,
        ((surface.g as f32 * surface_weight + other.g as f32 * weight).min(255.0)) as u8,
        ((surface.b as f32 * surface_weight + other.b as f32 * weight).min(255.0)) as u8, // FIXED: was other.g
        surface.a,
    )
}

// Additive blending for accumulating reflection/refraction
fn color_blend_weighted(base: Color, add: Color, weight: f32) -> Color {
    Color::new(
        ((base.r as f32 + add.r as f32 * weight).min(255.0)) as u8,
        ((base.g as f32 + add.g as f32 * weight).min(255.0)) as u8,
        ((base.b as f32 + add.b as f32 * weight).min(255.0)) as u8,
        base.a,
    )
}

pub fn apply_bloom_optimized(
    framebuffer: &mut Framebuffer,
    threshold: f32,
    blur_radius: i32,
    intensity: f32,
    downsample: i32,
) {
    let width = (framebuffer.width() / downsample) as usize;
    let height = (framebuffer.height() / downsample) as usize;

    // Get ALL pixel data once (this is the key - one operation instead of millions)
    let original_pixels = framebuffer.color_buffer.get_image_data();

    // Extract bright pixels at lower resolution
    let mut small_bright = vec![Color::new(0, 0, 0, 255); width * height];

    for y in 0..height {
        for x in 0..width {
            let orig_x = (x * downsample as usize).min(framebuffer.width() as usize - 1);
            let orig_y = (y * downsample as usize).min(framebuffer.height() as usize - 1);
            let idx = orig_y * framebuffer.width() as usize + orig_x;

            let color = original_pixels[idx];
            let brightness = (color.r as f32 + color.g as f32 + color.b as f32) / (3.0 * 255.0);

            if brightness > threshold {
                small_bright[y * width + x] = color;
            }
        }
    }

    // Blur at lower resolution
    let blurred = blur_pixels(&small_bright, width, height, blur_radius);

    // Blend back
    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let small_x = ((x / downsample).min(width as i32 - 1)) as usize;
            let small_y = ((y / downsample).min(height as i32 - 1)) as usize;
            let idx = y as usize * framebuffer.width() as usize + x as usize;

            let original = original_pixels[idx];
            let bloom = blurred[small_y * width + small_x];

            let final_color = Color::new(
                ((original.r as f32 + bloom.r as f32 * intensity).min(255.0)) as u8,
                ((original.g as f32 + bloom.g as f32 * intensity).min(255.0)) as u8,
                ((original.b as f32 + bloom.b as f32 * intensity).min(255.0)) as u8,
                255,
            );

            framebuffer.color_buffer.draw_pixel(x, y, final_color);
        }
    }
}

fn blur_pixels(pixels: &[Color], width: usize, height: usize, radius: i32) -> Vec<Color> {
    let mut blurred = vec![Color::new(0, 0, 0, 255); width * height];

    for y in 0..height {
        for x in 0..width {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut count = 0u32;

            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = ((x as i32 + dx).clamp(0, width as i32 - 1)) as usize;
                    let ny = ((y as i32 + dy).clamp(0, height as i32 - 1)) as usize;

                    let color = pixels[ny * width + nx];
                    r_sum += color.r as u32;
                    g_sum += color.g as u32;
                    b_sum += color.b as u32;
                    count += 1;
                }
            }

            blurred[y * width + x] = Color::new(
                (r_sum / count) as u8,
                (g_sum / count) as u8,
                (b_sum / count) as u8,
                255,
            );
        }
    }

    blurred
}
