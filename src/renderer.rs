use crate::bvh::BVH; // NEW: Import BVH
use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::intersection::Intersect;
use crate::light::Light;
use crate::objects::{Object, RayIntersect};
use crate::skybox::Skybox;
use crate::texture_pack::TextureManager;
use rand::Rng;
use raylib::prelude::*;
use rayon::prelude::*; // NEW: For parallel iteration
//

use std::f32::consts::PI;
use std::sync::Mutex; // NEW: For thread-safe framebuffer access

const MAX_RECURSION_DEPTH: u32 = 4;

const LIGHT_ATTENUTATION: f32 = 0.7;

const SHADOW_BIAS: f32 = 0.001;

// OPTIMIZED: Reduced shadow samples for better performance
const SHADOW_SAMPLES_MIN: usize = 1; // Changed from 1
const SHADOW_SAMPLES_MAX: usize = 4; // Changed from 8

const DISTANCE_NEAR: f32 = 5.0;
const DISTANCE_FAR: f32 = 20.0;
const SAMPLES_NEAR: usize = 4; // Changed from 8
const SAMPLES_FAR: usize = 1; // Changed from 2

// NEW: Early termination threshold for recursion
const MIN_RAY_CONTRIBUTION: f32 = 0.01; // Stop if ray contributes less than 1% to final color

// NEW: Pre-collected emissive light source
#[derive(Clone, Copy)]
struct EmissiveLight {
    position: Vector3,
    color: Color,
    strength: f32,
}

pub fn render(
    framebuffer: &mut Framebuffer,
    bvh: &BVH, // NEW: Changed from objects slice to BVH
    camera: &Camera,
    lights: &[Light],
    skybox: &Skybox,
    texture_manager: &TextureManager,
) {
    // Pre-collect emissive objects into light sources
    let emissive_lights = collect_emissive_lights(bvh.objects()); // NEW: Get objects from BVH

    let width = framebuffer.width() as f32;
    let height = framebuffer.height() as f32;

    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan() as f32;

    // Create a thread-safe wrapper for framebuffer access
    let fb_width = framebuffer.width();
    let fb_height = framebuffer.height();

    // Pre-allocate pixel buffer for parallel writes
    let pixel_count = (fb_width * fb_height) as usize;
    let mut pixels: Vec<Color> = vec![Color::new(0, 0, 0, 255); pixel_count];

    // Parallel iteration over all pixels
    pixels
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            let x = (index as i32) % fb_width;
            let y = (index as i32) / fb_width;

            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(
                &camera.eye,
                &rotated_direction,
                bvh, // NEW: Pass BVH instead of objects
                lights,
                &emissive_lights,
                skybox,
                0,
                texture_manager,
            );

            *pixel = pixel_color;
        });

    // Write all pixels to framebuffer at once (single-threaded, but fast)
    for (index, pixel) in pixels.iter().enumerate() {
        let x = (index as i32) % fb_width;
        let y = (index as i32) / fb_width;
        framebuffer.set_foreground_color(*pixel);
        framebuffer.set_pixel(x, y);
    }
}

fn collect_emissive_lights(objects: &[Object]) -> Vec<EmissiveLight> {
    let mut emissive_lights = Vec::new();

    for object in objects {
        let (material, position) = match object {
            Object::Sphere(sphere) => (sphere.material(), sphere.center()),
            Object::Cube(cube) => {
                // For cubes, we need to check if ANY face is emissive
                // Use the first face's material and the cube's center
                let materials = cube.materials;
                let has_emission = materials.iter().any(|m| m.emission_strength > 0.5);

                if !has_emission {
                    continue;
                }

                // Use the material with strongest emission
                let best_material = materials
                    .iter()
                    .max_by(|a, b| {
                        a.emission_strength
                            .partial_cmp(&b.emission_strength)
                            .unwrap()
                    })
                    .unwrap();

                (*best_material, cube.center)
            }
        };

        // Only collect objects with meaningful emission
        if material.emission_strength > 0.5 {
            emissive_lights.push(EmissiveLight {
                position,
                color: material.emission,
                strength: material.emission_strength,
            });
        }
    }

    emissive_lights
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    bvh: &BVH,
    lights: &[Light],
    emissive_lights: &[EmissiveLight],
    skybox: &Skybox,
    depth: u32,
    texture_manager: &TextureManager, // NEW
) -> Color {
    cast_ray_weighted(
        ray_origin,
        ray_direction,
        bvh,
        lights,
        emissive_lights,
        skybox,
        depth,
        1.0,
        texture_manager, // NEW
    )
}

// NEW: Internal function with weight tracking for early termination
fn cast_ray_weighted(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    bvh: &BVH,
    lights: &[Light],
    emissive_lights: &[EmissiveLight],
    skybox: &Skybox,
    depth: u32,
    weight: f32,
    texture_manager: &TextureManager, // NEW: Pass texture manager
) -> Color {
    if depth >= MAX_RECURSION_DEPTH || weight < MIN_RAY_CONTRIBUTION {
        return skybox.sample(ray_direction);
    }

    let intersection = bvh.intersect(ray_origin, ray_direction);

    if !intersection.is_intersecting {
        return skybox.sample(ray_direction);
    }

    let material = intersection.material();

    // NEW: Handle alpha-based transparency modulation
    let texture_alpha = if let Some(texture_id) = material.texture_id {
        if let Some(pack) = texture_manager.active_pack() {
            if let Some(texture) = pack.get_texture(texture_id) {
                let sampled = texture.diffuse.sample(intersection.uv.x, intersection.uv.y);
                sampled.a as f32 / 255.0
            } else {
                1.0
            }
        } else {
            1.0
        }
    } else {
        1.0
    };

    // Modulate material transparency with texture alpha
    let effective_transparency = material.transparency * texture_alpha;

    let is_entering = intersection.normal.dot(*ray_direction) < 0.0;

    let outward_normal = if is_entering {
        intersection.normal
    } else {
        -intersection.normal
    };

    let view_dir = ray_direction.scale_by(-1.0);

    let surface_color = calculate_lighting(
        &intersection,
        &outward_normal,
        &view_dir,
        lights,
        emissive_lights,
        bvh,
        skybox,
        texture_manager, // NEW: Pass texture manager
    );

    // Rest of the function remains the same, but use effective_transparency
    // instead of material.transparency everywhere

    // Handle non-transparent materials with reflection
    if effective_transparency < 0.01 {
        if material.reflectivity > 0.0 {
            let reflect_dir = reflect(&ray_direction, &outward_normal);
            let reflect_origin = intersection.point + outward_normal * SHADOW_BIAS;

            let new_weight = weight * material.reflectivity;
            let reflection_color = cast_ray_weighted(
                &reflect_origin,
                &reflect_dir,
                bvh,
                lights,
                emissive_lights,
                skybox,
                depth + 1,
                new_weight,
                texture_manager,
            );

            let blended = color_blend(surface_color, reflection_color, material.reflectivity);

            if material.emission_strength > 0.0 {
                let emission_color = color_multiply(material.emission, material.emission_strength);
                return color_add(blended, emission_color);
            }

            return blended;
        }

        if material.emission_strength > 0.0 {
            let emission_color = color_multiply(material.emission, material.emission_strength);
            return color_add(surface_color, emission_color);
        }

        return surface_color;
    }

    // Handle transparent materials (same as before but use effective_transparency)
    let (n1, n2) = if is_entering {
        (1.0, material.refractive_index)
    } else {
        (material.refractive_index, 1.0)
    };

    let kr = fresnel(&ray_direction, &outward_normal, n1, n2);
    let kt = 1.0 - kr;

    let mut final_color = color_multiply(surface_color, 1.0 - effective_transparency);

    if kr > 0.0 && effective_transparency > 0.0 {
        let reflection_weight = kr * effective_transparency;

        if weight * reflection_weight >= MIN_RAY_CONTRIBUTION {
            let reflect_dir = reflect(&ray_direction, &outward_normal);
            let reflect_origin = intersection.point + outward_normal * SHADOW_BIAS;

            let reflection_color = cast_ray_weighted(
                &reflect_origin,
                &reflect_dir,
                bvh,
                lights,
                emissive_lights,
                skybox,
                depth + 1,
                weight * reflection_weight,
                texture_manager,
            );

            final_color = color_add(
                final_color,
                color_multiply(reflection_color, reflection_weight),
            );
        }
    }

    if kt > 0.0 && effective_transparency > 0.0 {
        let refraction_weight = kt * effective_transparency;

        if weight * refraction_weight >= MIN_RAY_CONTRIBUTION {
            if let Some(refract_dir) = refract(&ray_direction, &outward_normal, n1, n2) {
                let refract_origin = if is_entering {
                    intersection.point - outward_normal * SHADOW_BIAS
                } else {
                    intersection.point + outward_normal * SHADOW_BIAS
                };

                let refraction_color = cast_ray_weighted(
                    &refract_origin,
                    &refract_dir,
                    bvh,
                    lights,
                    emissive_lights,
                    skybox,
                    depth + 1,
                    weight * refraction_weight,
                    texture_manager,
                );

                final_color = color_add(
                    final_color,
                    color_multiply(refraction_color, refraction_weight),
                );
            }
        }
    }

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
    emissive_lights: &[EmissiveLight],
    bvh: &BVH,
    skybox: &Skybox,
    texture_manager: &TextureManager,
) -> Color {
    let material = intersection.material();

    // Sample texture if material has one
    let (diffuse_color, final_normal, final_specular) = if let Some(texture_id) =
        material.texture_id
    {
        if let Some(pack) = texture_manager.active_pack() {
            if let Some(texture) = pack.get_texture(texture_id) {
                // 1. Sample diffuse texture
                let sampled_color = texture.diffuse.sample(intersection.uv.x, intersection.uv.y);

                // 2. Sample normal map if available
                let final_normal = if let Some(ref normal_map) = texture.normal {
                    let tangent_normal =
                        normal_map.sample_normal(intersection.uv.x, intersection.uv.y);

                    let (tangent, bitangent, _) =
                        calculate_tangent_basis_for_cube(normal, intersection.face_index);

                    tangent_to_world(&tangent_normal, &tangent, &bitangent, normal)
                } else {
                    *normal
                };

                // 3. NEW: Sample specular map if available
                let final_specular = if let Some(ref specular_map) = texture.specular {
                    // Sample specular map and get intensity
                    let intensity =
                        specular_map.sample_specular(intersection.uv.x, intersection.uv.y);
                    // Apply exponential scaling for more dramatic variation
                    material.specular * (intensity * intensity)
                } else {
                    // Fallback: derive from diffuse brightness
                    let brightness =
                        (sampled_color.r as f32 + sampled_color.g as f32 + sampled_color.b as f32)
                            / (3.0 * 255.0);
                    let smoothness = 1.0 - brightness; // Invert: bright = rough, dark = smooth
                    material.specular * (smoothness * smoothness) // Exponential
                };

                (sampled_color, final_normal, final_specular)
            } else {
                (material.diffuse, *normal, material.specular)
            }
        } else {
            (material.diffuse, *normal, material.specular)
        }
    } else {
        (material.diffuse, *normal, material.specular)
    };

    // Use final_normal and final_specular for all lighting calculations below
    let normal = &final_normal;

    // Skybox-driven ambient lighting
    let ambient_dir = *normal;
    let ambient_sample = skybox.sample(&ambient_dir);
    let ambient_intensity = 0.2;
    let mut lighting_color = color_multiply(ambient_sample, ambient_intensity * material.albedo[0]);

    let mut specular_color = Color::new(0, 0, 0, 255);

    // Process regular lights
    for light in lights {
        let visibility = if light.radius > 0.01 {
            calculate_adaptive_visibility_with_distance(
                &intersection.point,
                normal,
                light,
                bvh,
                intersection.distance(),
            )
        } else {
            let light_dir = (light.position - intersection.point).normalized();
            let light_distance = (light.position - intersection.point).length();
            let shadow_origin = intersection.point + *normal * SHADOW_BIAS;

            if !cast_shadow_ray(&shadow_origin, &light_dir, light_distance, bvh) {
                1.0
            } else {
                0.0
            }
        };

        if visibility > 0.0 {
            let light_dir = (light.position - intersection.point).normalized();
            let light_intensity = light.get_intensity_at(&intersection.point);

            let diffuse_intensity = normal.dot(light_dir).max(0.0) * light_intensity * visibility;
            let diffuse_contribution =
                color_multiply(light.color, diffuse_intensity * material.albedo[0]);
            lighting_color = color_add(lighting_color, diffuse_contribution);

            let reflect_dir = reflect(&light_dir, normal);
            // NEW: Use final_specular instead of material.specular
            let specular_intensity = view_dir.dot(reflect_dir).max(0.0).powf(final_specular)
                * light_intensity
                * visibility;
            let specular_contribution =
                color_multiply(light.color, specular_intensity * material.albedo[1]);
            specular_color = color_add(specular_color, specular_contribution);
        }
    }

    // Process emissive lights
    for emissive_light in emissive_lights {
        let light_dir = (emissive_light.position - intersection.point).normalized();
        let light_distance = (emissive_light.position - intersection.point).length();
        let shadow_origin = intersection.point + *normal * SHADOW_BIAS;

        let is_shadowed = cast_shadow_ray(&shadow_origin, &light_dir, light_distance, bvh);

        if !is_shadowed {
            let base_intensity = emissive_light.strength;
            let light_intensity =
                base_intensity / (1.0 + light_distance * light_distance * LIGHT_ATTENUTATION);

            let diffuse_intensity = normal.dot(light_dir).max(0.0) * light_intensity;
            let diffuse_contribution =
                color_multiply(emissive_light.color, diffuse_intensity * material.albedo[0]);
            lighting_color = color_add(lighting_color, diffuse_contribution);

            let reflect_dir = reflect(&light_dir, normal);
            // NEW: Use final_specular instead of material.specular
            let specular_intensity =
                view_dir.dot(reflect_dir).max(0.0).powf(final_specular) * light_intensity;
            let specular_contribution = color_multiply(
                emissive_light.color,
                specular_intensity * material.albedo[1],
            );
            specular_color = color_add(specular_color, specular_contribution);
        }
    }

    // Combine lighting with texture color
    let lit_color = color_add(lighting_color, specular_color);
    let final_color = color_modulate(diffuse_color, lit_color);

    final_color
}

fn calculate_adaptive_visibility_with_distance(
    point: &Vector3,
    normal: &Vector3,
    light: &Light,
    bvh: &BVH,
    distance_from_camera: f32,
) -> f32 {
    let sample_count = calculate_sample_count_for_distance(distance_from_camera);

    let mut hit_count = 0;
    let mut total_samples = 0;

    // OPTIMIZED: Early exit strategy - sample minimum first
    for sample_idx in 0..SHADOW_SAMPLES_MIN {
        let sample_offset = get_stratified_sample_on_sphere(sample_idx, sample_count, light.radius);
        let sample_position = light.position + sample_offset;

        let light_dir = (sample_position - *point).normalized();
        let light_distance = (sample_position - *point).length();
        let shadow_origin = *point + *normal * SHADOW_BIAS;

        if !cast_shadow_ray(&shadow_origin, &light_dir, light_distance, bvh) {
            hit_count += 1;
        }
        total_samples += 1;
    }

    // OPTIMIZED: If all samples hit or all miss, early exit
    if hit_count == SHADOW_SAMPLES_MIN {
        return 1.0; // Fully lit
    }

    if hit_count == 0 {
        return 0.0; // Fully shadowed
    }

    // Only continue sampling if we got partial shadow
    for sample_idx in SHADOW_SAMPLES_MIN..sample_count {
        let sample_offset = get_stratified_sample_on_sphere(sample_idx, sample_count, light.radius);
        let sample_position = light.position + sample_offset;

        let light_dir = (sample_position - *point).normalized();
        let light_distance = (sample_position - *point).length();
        let shadow_origin = *point + *normal * SHADOW_BIAS;

        if !cast_shadow_ray(&shadow_origin, &light_dir, light_distance, bvh) {
            hit_count += 1;
        }
        total_samples += 1;
    }

    hit_count as f32 / total_samples as f32
}

fn calculate_sample_count_for_distance(distance: f32) -> usize {
    if distance <= DISTANCE_NEAR {
        SAMPLES_NEAR
    } else if distance >= DISTANCE_FAR {
        SAMPLES_FAR
    } else {
        let t = (distance - DISTANCE_NEAR) / (DISTANCE_FAR - DISTANCE_NEAR);
        let interpolated = SAMPLES_NEAR as f32 * (1.0 - t) + SAMPLES_FAR as f32 * t;
        (interpolated.round() as usize).clamp(SAMPLES_FAR, SAMPLES_NEAR)
    }
}

fn cast_shadow_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    light_distance: f32,
    bvh: &BVH,
) -> bool {
    let intersection = bvh.intersect(ray_origin, ray_direction);

    if intersection.is_intersecting() {
        let material = intersection.material();
        let distance = intersection.distance();

        // If we hit an emissive surface before or at the target distance,
        // we've reached a light source - not shadowed
        if material.emission_strength > 0.5 && distance <= light_distance + 0.01 {
            return false; // Hit a light source
        }

        // If we hit an opaque object before reaching the light, it's blocked
        if distance < light_distance && material.transparency < 0.5 {
            return true; // Shadowed by opaque object
        }
    }

    false // Not shadowed
}

fn refract(incident: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> Option<Vector3> {
    let eta = n1 / n2;
    let cos_i = -normal.dot(*incident);
    let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);

    if sin_t2 > 1.0 {
        return None;
    }

    let cos_t = (1.0 - sin_t2).sqrt();
    Some(*incident * eta + *normal * (eta * cos_i - cos_t))
}

fn fresnel(incident: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> f32 {
    let mut cos_i = -normal.dot(*incident).max(-1.0).min(1.0);

    let (eta_i, eta_t) = (n1, n2);

    let sin_t = (eta_i / eta_t) * (1.0 - cos_i * cos_i).max(0.0).sqrt();

    if sin_t >= 1.0 {
        return 1.0;
    }

    let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();

    let denom_s = (eta_t * cos_i) + (eta_i * cos_t);
    let denom_p = (eta_i * cos_i) + (eta_t * cos_t);

    if denom_s.abs() < 1e-6 || denom_p.abs() < 1e-6 {
        return 1.0;
    }

    let rs = ((eta_t * cos_i) - (eta_i * cos_t)) / denom_s;
    let rp = ((eta_i * cos_i) - (eta_t * cos_t)) / denom_p;

    ((rs * rs + rp * rp) / 2.0).max(0.0).min(1.0)
}

#[inline(always)]
fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * normal.dot(*incident)
}

#[inline(always)]
fn color_multiply(color: Color, intensity: f32) -> Color {
    Color::new(
        (color.r as f32 * intensity).min(255.0).max(0.0) as u8,
        (color.g as f32 * intensity).min(255.0).max(0.0) as u8,
        (color.b as f32 * intensity).min(255.0).max(0.0) as u8,
        color.a,
    )
}

#[inline(always)]
fn color_add(a: Color, b: Color) -> Color {
    Color::new(
        (a.r as u16 + b.r as u16).min(255) as u8,
        (a.g as u16 + b.g as u16).min(255) as u8,
        (a.b as u16 + b.b as u16).min(255) as u8,
        a.a,
    )
}

#[inline(always)]
fn color_blend(surface: Color, other: Color, weight: f32) -> Color {
    let surface_weight = 1.0 - weight;
    Color::new(
        ((surface.r as f32 * surface_weight + other.r as f32 * weight).min(255.0)) as u8,
        ((surface.g as f32 * surface_weight + other.g as f32 * weight).min(255.0)) as u8,
        ((surface.b as f32 * surface_weight + other.b as f32 * weight).min(255.0)) as u8,
        surface.a,
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

    let original_pixels = framebuffer.color_buffer.get_image_data();

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

    let blurred = blur_pixels(&small_bright, width, height, blur_radius);

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

#[inline(always)]
pub fn get_stratified_sample_on_sphere(index: usize, total_samples: usize, radius: f32) -> Vector3 {
    const PHI: f32 = 1.618033988749895;

    let i = index as f32;
    let n = total_samples as f32;

    let y = 1.0 - (i / (n - 1.0)) * 2.0;

    let radius_at_y = (1.0 - y * y).sqrt();

    let theta = 2.0 * PI * i / PHI;

    let x = theta.cos() * radius_at_y;
    let z = theta.sin() * radius_at_y;

    Vector3::new(x, y, z) * radius
}

pub fn get_grid_sample_on_sphere(index: usize, total_samples: usize, radius: f32) -> Vector3 {
    let sample = match total_samples {
        4 => match index {
            0 => Vector3::new(1.0, 0.0, 0.0),
            1 => Vector3::new(-1.0, 0.0, 0.0),
            2 => Vector3::new(0.0, 1.0, 0.0),
            3 => Vector3::new(0.0, -1.0, 0.0),
            _ => Vector3::new(0.0, 0.0, 1.0),
        },
        8 => {
            let x = if index & 1 == 0 { 1.0 } else { -1.0 };
            let y = if index & 2 == 0 { 1.0 } else { -1.0 };
            let z = if index & 4 == 0 { 1.0 } else { -1.0 };
            Vector3::new(x, y, z).normalized()
        }
        16 => {
            let layer = index / 4;
            let corner = index % 4;

            let y = match layer {
                0 => 1.0,
                1 | 2 => 0.0,
                _ => -1.0,
            };

            let angle = corner as f32 * PI / 2.0;
            let x = angle.cos();
            let z = angle.sin();

            Vector3::new(x, y, z).normalized()
        }
        _ => {
            let i = index as f32;
            let n = total_samples as f32;
            let y = 1.0 - (i / (n - 1.0).max(1.0)) * 2.0;
            let radius_at_y = (1.0 - y * y).sqrt();
            let theta = 2.0 * PI * i / 1.618033988749895;
            let x = theta.cos() * radius_at_y;
            let z = theta.sin() * radius_at_y;
            Vector3::new(x, y, z)
        }
    };

    sample * radius
}

#[inline(always)]
fn calculate_tangent_basis_for_cube(
    normal: &Vector3,
    face_index: usize,
) -> (Vector3, Vector3, Vector3) {
    // Tangent points in the direction of increasing U
    // Bitangent points in the direction of increasing V
    let (tangent, bitangent) = match face_index {
        0 => (
            // +X face (right)
            Vector3::new(0.0, 0.0, -1.0), // U increases as Z decreases
            Vector3::new(0.0, -1.0, 0.0), // V increases as Y decreases
        ),
        1 => (
            // -X face (left)
            Vector3::new(0.0, 0.0, 1.0),  // U increases as Z increases
            Vector3::new(0.0, -1.0, 0.0), // V increases as Y decreases
        ),
        2 => (
            // +Y face (top)
            Vector3::new(1.0, 0.0, 0.0),  // U increases as X increases
            Vector3::new(0.0, 0.0, -1.0), // V increases as Z decreases
        ),
        3 => (
            // -Y face (bottom)
            Vector3::new(1.0, 0.0, 0.0), // U increases as X increases
            Vector3::new(0.0, 0.0, 1.0), // V increases as Z increases
        ),
        4 => (
            // +Z face (front)
            Vector3::new(1.0, 0.0, 0.0),  // U increases as X increases
            Vector3::new(0.0, -1.0, 0.0), // V increases as Y decreases
        ),
        5 => (
            // -Z face (back)
            Vector3::new(-1.0, 0.0, 0.0), // U increases as X decreases
            Vector3::new(0.0, -1.0, 0.0), // V increases as Y decreases
        ),
        _ => (Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0)),
    };

    (tangent.normalized(), bitangent.normalized(), *normal)
}
// Add this helper function to calculate tangent space basis

// Add this helper to transform normal from tangent space to world space
#[inline(always)]
fn tangent_to_world(
    tangent_normal: &Vector3,
    tangent: &Vector3,
    bitangent: &Vector3,
    normal: &Vector3,
) -> Vector3 {
    let x = tangent.scale_by(tangent_normal.x);
    let y = bitangent.scale_by(tangent_normal.y);
    let z = normal.scale_by(tangent_normal.z);

    (x + y + z).normalized()
}

#[inline(always)]
fn color_modulate(a: Color, b: Color) -> Color {
    Color::new(
        ((a.r as u16 * b.r as u16) / 255) as u8,
        ((a.g as u16 * b.g as u16) / 255) as u8,
        ((a.b as u16 * b.b as u16) / 255) as u8,
        a.a,
    )
}
