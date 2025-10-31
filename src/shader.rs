use crate::fragment::Fragment;
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;
use raylib::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    None,
    Rocky,
    GasGiant,
    Ringed,
    Magenta,    // Add this
    WaterWorld, // Add this
}

impl ShaderType {
    pub fn fragment_shader(&self, fragment: &Fragment, uniforms: &Uniforms) -> Option<Color> {
        match self {
            ShaderType::None => None,
            ShaderType::Rocky => Some(rocky_fragment_shader(fragment, uniforms)),
            ShaderType::GasGiant => Some(gas_giant_fragment_shader(fragment, uniforms)),
            ShaderType::Ringed => Some(ringed_fragment_shader(fragment, uniforms)),
            ShaderType::Magenta => Some(magenta_fragment_shader(fragment, uniforms)), // Add this
            ShaderType::WaterWorld => Some(water_world_fragment_shader(fragment, uniforms)), // Add this
        }
    }

    pub fn vertex_shader(&self, vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
        match self {
            ShaderType::Ringed => ringed_vertex_shader(vertex, uniforms),
            _ => vertex.clone(),
        }
    }
}

// Helper function for smooth noise-like patterns
fn smooth_noise(x: f32, y: f32, z: f32) -> f32 {
    ((x * 3.7).sin() * (y * 5.3).cos() * (z * 4.1).sin()
        + (x * 7.1).cos() * (y * 2.9).sin() * (z * 6.7).cos())
        * 0.5
}

// Rocky Planet Shader - Mars/Earth-like with continents, oceans, and weather
fn rocky_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.object_position(); // Use object space!
    let world_pos = fragment.world_position(); // Keep for view-dependent effects
    let normal = fragment.normal();

    // Normalize normal
    let normal_length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    let normalized_normal = Vector3::new(
        normal.x / normal_length,
        normal.y / normal_length,
        normal.z / normal_length,
    );

    // LAYER 1: Large-scale continental features (low frequency)
    let continents = ((pos.x * 2.0 + uniforms.time() * 0.05).sin()
        * (pos.y * 1.8).cos()
        * (pos.z * 2.2 + uniforms.time() * 0.03).sin())
    .abs();

    // LAYER 2: Medium-scale terrain variation (mountain ranges, valleys)
    let terrain = smooth_noise(
        pos.x * 8.0 + uniforms.time() * 0.1,
        pos.y * 7.0,
        pos.z * 8.5,
    );

    // LAYER 3: Fine surface detail (rocks, craters)
    let detail = ((pos.x * 35.0).sin() * (pos.y * 32.0).cos() * (pos.z * 38.0).sin()).abs();

    // LAYER 4: Animated cloud/atmosphere layer
    let clouds = ((pos.x * 12.0 + uniforms.time() * 0.4).sin()
        * (pos.y * 10.0 - uniforms.time() * 0.3).cos()
        * (pos.z * 11.0 + uniforms.time() * 0.35).sin()
        + 1.0)
        * 0.5;

    // LAYER 5: Polar ice caps
    let polar_factor = (pos.y.abs() - 0.7).max(0.0) * 3.0; // Ice at poles

    // Combine noise layers with different amplitudes
    let combined_height = continents * 0.5 + terrain * 0.3 + detail * 0.15;

    // Define color zones based on elevation
    let deep_ocean = Vector3::new(0.05, 0.1, 0.3);
    let ocean = Vector3::new(0.1, 0.25, 0.55);
    let beach = Vector3::new(0.76, 0.7, 0.5);
    let lowland = Vector3::new(0.2, 0.5, 0.15);
    let highland = Vector3::new(0.4, 0.45, 0.25);
    let mountain = Vector3::new(0.5, 0.48, 0.45);
    let snow = Vector3::new(0.9, 0.92, 0.95);

    // Map height to colors with smooth transitions
    let base_color = if combined_height < 0.25 {
        // Deep ocean
        let t = combined_height / 0.25;
        Vector3::new(
            deep_ocean.x + (ocean.x - deep_ocean.x) * t,
            deep_ocean.y + (ocean.y - deep_ocean.y) * t,
            deep_ocean.z + (ocean.z - deep_ocean.z) * t,
        )
    } else if combined_height < 0.35 {
        // Ocean to beach
        let t = (combined_height - 0.25) / 0.1;
        Vector3::new(
            ocean.x + (beach.x - ocean.x) * t,
            ocean.y + (beach.y - ocean.y) * t,
            ocean.z + (beach.z - ocean.z) * t,
        )
    } else if combined_height < 0.45 {
        // Beach to lowland
        let t = (combined_height - 0.35) / 0.1;
        Vector3::new(
            beach.x + (lowland.x - beach.x) * t,
            beach.y + (lowland.y - beach.y) * t,
            beach.z + (lowland.z - beach.z) * t,
        )
    } else if combined_height < 0.6 {
        // Lowland to highland
        let t = (combined_height - 0.45) / 0.15;
        Vector3::new(
            lowland.x + (highland.x - lowland.x) * t,
            lowland.y + (highland.y - lowland.y) * t,
            lowland.z + (highland.z - lowland.z) * t,
        )
    } else if combined_height < 0.75 {
        // Highland to mountain
        let t = (combined_height - 0.6) / 0.15;
        Vector3::new(
            highland.x + (mountain.x - highland.x) * t,
            highland.y + (mountain.y - highland.y) * t,
            highland.z + (mountain.z - highland.z) * t,
        )
    } else {
        // Mountain peaks with snow
        let t = (combined_height - 0.75) / 0.25;
        Vector3::new(
            mountain.x + (snow.x - mountain.x) * t,
            mountain.y + (snow.y - mountain.y) * t,
            mountain.z + (snow.z - mountain.z) * t,
        )
    };

    // Add polar ice caps (blend toward white at poles)
    let ice_color = Vector3::new(0.85, 0.9, 0.95);
    let with_ice = Vector3::new(
        base_color.x + (ice_color.x - base_color.x) * polar_factor.min(1.0),
        base_color.y + (ice_color.y - base_color.y) * polar_factor.min(1.0),
        base_color.z + (ice_color.z - base_color.z) * polar_factor.min(1.0),
    );

    // Apply cloud layer (additive, brightens surface)
    let cloud_brightness = (clouds - 0.6).max(0.0) * 2.5;
    let cloud_color = Vector3::new(0.95, 0.95, 0.98);
    let with_clouds = Vector3::new(
        with_ice.x + cloud_color.x * cloud_brightness * 0.4,
        with_ice.y + cloud_color.y * cloud_brightness * 0.4,
        with_ice.z + cloud_color.z * cloud_brightness * 0.4,
    );

    // Calculate lighting
    let light_dir = uniforms.light_direction();
    let light_intensity = (normalized_normal.x * light_dir.x
        + normalized_normal.y * light_dir.y
        + normalized_normal.z * light_dir.z)
        .max(0.0);

    // Add specular highlights (water reflection) - uses world_pos for view direction
    let view_dir = Vector3::new(
        uniforms.camera_position().x - world_pos.x,
        uniforms.camera_position().y - world_pos.y,
        uniforms.camera_position().z - world_pos.z,
    );
    let view_length =
        (view_dir.x * view_dir.x + view_dir.y * view_dir.y + view_dir.z * view_dir.z).sqrt();
    let normalized_view = Vector3::new(
        view_dir.x / view_length,
        view_dir.y / view_length,
        view_dir.z / view_length,
    );

    let reflect_dot = 2.0
        * (normalized_normal.x * light_dir.x
            + normalized_normal.y * light_dir.y
            + normalized_normal.z * light_dir.z);
    let reflect = Vector3::new(
        reflect_dot * normalized_normal.x - light_dir.x,
        reflect_dot * normalized_normal.y - light_dir.y,
        reflect_dot * normalized_normal.z - light_dir.z,
    );

    let spec_dot = (reflect.x * normalized_view.x
        + reflect.y * normalized_view.y
        + reflect.z * normalized_view.z)
        .max(0.0);

    // Stronger specular on water (low elevation)
    let water_factor = (0.35 - combined_height).max(0.0) * 3.0;
    let specular = spec_dot.powf(20.0) * 0.6 * water_factor.min(1.0);

    // Final lighting combination
    let ambient = 0.15;
    let final_intensity = ambient + light_intensity * 0.85 + specular;

    let r = (with_clouds.x * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;
    let g = (with_clouds.y * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;
    let b = (with_clouds.z * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;

    Color::new(r, g, b, 255)
}

// Gas Giant Shader - Jupiter-like with complex storm systems
fn gas_giant_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.object_position(); // Use object space!
    let world_pos = fragment.world_position(); // Keep for view-dependent effects
    let normal = fragment.normal();

    // Normalize normal
    let normal_length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    let normalized_normal = Vector3::new(
        normal.x / normal_length,
        normal.y / normal_length,
        normal.z / normal_length,
    );

    // LAYER 1: Large horizontal bands (main atmospheric zones)
    let large_bands = (pos.y * 6.0 + uniforms.time() * 0.25).sin();

    // LAYER 2: Medium turbulent bands with longitude variation (jet streams)
    let jet_streams = (pos.y * 18.0 + pos.x * 3.0 + uniforms.time() * 0.6).sin()
        * (pos.z * 2.5 - uniforms.time() * 0.4).cos();

    // LAYER 3: Small-scale turbulence and eddies
    let turbulence = smooth_noise(
        pos.x * 25.0 + uniforms.time() * 0.8,
        pos.y * 20.0,
        pos.z * 25.0 - uniforms.time() * 0.7,
    );

    // LAYER 4: The Great Red Spot (large storm system) - NOW IN OBJECT SPACE
    let spot_center_x = 0.4;
    let spot_center_y = -0.2;
    let spot_x = pos.x - spot_center_x;
    let spot_y = pos.y - spot_center_y;
    let spot_distance = (spot_x * spot_x * 4.0 + spot_y * spot_y * 9.0).sqrt();
    let storm_spiral =
        (spot_distance * 15.0 - uniforms.time() * 3.0 + spot_x.atan2(spot_y) * 3.0).sin();
    let storm = ((1.0 - (spot_distance * 2.5).min(1.0)) * (0.7 + storm_spiral * 0.3)).max(0.0);

    // LAYER 5: Secondary smaller storms
    let small_storm_x = pos.x + 0.5;
    let small_storm_y = pos.y + 0.3;
    let small_storm_dist =
        (small_storm_x * small_storm_x * 6.0 + small_storm_y * small_storm_y * 6.0).sqrt();
    let small_storm = ((1.0 - (small_storm_dist * 4.0).min(1.0)) * 0.5).max(0.0);

    // LAYER 6: Polar aurora-like effects
    let polar_glow = ((pos.y.abs() - 0.6).max(0.0) * 2.5).min(1.0);
    let aurora_shimmer = ((uniforms.time() * 4.0 + pos.x * 10.0).sin() * 0.5 + 0.5);

    // Combine all layers
    let band_value = large_bands * 0.4 + jet_streams * 0.3 + turbulence * 0.3;
    let t = (band_value + 1.0) * 0.5;

    // Define color palette (Jupiter-inspired)
    let dark_orange = Vector3::new(0.7, 0.35, 0.15);
    let light_orange = Vector3::new(0.95, 0.6, 0.25);
    let cream = Vector3::new(0.98, 0.88, 0.75);
    let white_band = Vector3::new(0.95, 0.92, 0.88);
    let storm_red = Vector3::new(0.85, 0.25, 0.15);
    let storm_dark = Vector3::new(0.5, 0.15, 0.1);
    let polar_blue = Vector3::new(0.3, 0.5, 0.8);

    // Create banded color with more variation
    let base = if t < 0.25 {
        let local_t = t / 0.25;
        Vector3::new(
            dark_orange.x + (light_orange.x - dark_orange.x) * local_t,
            dark_orange.y + (light_orange.y - dark_orange.y) * local_t,
            dark_orange.z + (light_orange.z - dark_orange.z) * local_t,
        )
    } else if t < 0.5 {
        let local_t = (t - 0.25) / 0.25;
        Vector3::new(
            light_orange.x + (cream.x - light_orange.x) * local_t,
            light_orange.y + (cream.y - light_orange.y) * local_t,
            light_orange.z + (cream.z - light_orange.z) * local_t,
        )
    } else if t < 0.75 {
        let local_t = (t - 0.5) / 0.25;
        Vector3::new(
            cream.x + (white_band.x - cream.x) * local_t,
            cream.y + (white_band.y - cream.y) * local_t,
            cream.z + (white_band.z - cream.z) * local_t,
        )
    } else {
        let local_t = (t - 0.75) / 0.25;
        Vector3::new(
            white_band.x + (light_orange.x - white_band.x) * local_t,
            white_band.y + (light_orange.y - white_band.y) * local_t,
            white_band.z + (light_orange.z - white_band.z) * local_t,
        )
    };

    // Add Great Red Spot
    let with_storm = Vector3::new(
        base.x
            + (storm_red.x - base.x) * storm * 0.8
            + (storm_dark.x - base.x) * storm * storm * 0.4,
        base.y
            + (storm_red.y - base.y) * storm * 0.8
            + (storm_dark.y - base.y) * storm * storm * 0.4,
        base.z
            + (storm_red.z - base.z) * storm * 0.8
            + (storm_dark.z - base.z) * storm * storm * 0.4,
    );

    // Add smaller storms
    let with_small_storms = Vector3::new(
        with_storm.x + (cream.x - with_storm.x) * small_storm * 0.4,
        with_storm.y + (cream.y - with_storm.y) * small_storm * 0.4,
        with_storm.z + (cream.z - with_storm.z) * small_storm * 0.4,
    );

    // Add polar glow
    let with_polar = Vector3::new(
        with_small_storms.x + polar_blue.x * polar_glow * aurora_shimmer * 0.3,
        with_small_storms.y + polar_blue.y * polar_glow * aurora_shimmer * 0.3,
        with_small_storms.z + polar_blue.z * polar_glow * aurora_shimmer * 0.3,
    );

    // Calculate lighting
    let light_dir = uniforms.light_direction();
    let light_intensity = (normalized_normal.x * light_dir.x
        + normalized_normal.y * light_dir.y
        + normalized_normal.z * light_dir.z)
        .max(0.0);

    // Gas giants have thick atmospheres - softer lighting
    let ambient = 0.35;
    let final_intensity = ambient + light_intensity * 0.65;

    let r = (with_polar.x * final_intensity * 255.0).min(255.0).max(0.0) as u8;
    let g = (with_polar.y * final_intensity * 255.0).min(255.0).max(0.0) as u8;
    let b = (with_polar.z * final_intensity * 255.0).min(255.0).max(0.0) as u8;

    Color::new(r, g, b, 255)
}

// Ringed Planet Shader - Saturn-like ice planet with prominent rings
fn ringed_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.object_position(); // Use object space!
    let world_pos = fragment.world_position(); // Keep for view-dependent effects
    let normal = fragment.normal();

    // Calculate distance from Y axis and height - NOW IN OBJECT SPACE
    let distance_from_center = (pos.x * pos.x + pos.z * pos.z).sqrt();
    let height_from_equator = pos.y.abs();

    // RING RENDERING (pure fragment shader approach)
    // Define ring zone in object space
    let ring_inner_radius = 1.15;
    let ring_outer_radius = 1.9;
    let ring_max_thickness = 0.02; // Very thin!

    // Check if we're in the ring's radial range
    let in_ring_radius =
        distance_from_center >= ring_inner_radius && distance_from_center <= ring_outer_radius;

    // Check if we're close enough to the equatorial plane for rings
    let in_ring_plane = height_from_equator <= ring_max_thickness;

    if in_ring_radius && in_ring_plane {
        // We're in the ring zone - render ring material

        // Create ring bands with animation
        let ring_pattern =
            ((distance_from_center * 30.0 + uniforms.time() * 0.5).sin() + 1.0) * 0.5;
        let ring_bands = ((distance_from_center * 8.0).sin() + 1.0) * 0.5;

        // Combine patterns for more detail
        let ring_detail = ring_pattern * 0.6 + ring_bands * 0.4;

        // Ring colors - icy/rocky mix
        let ring_bright = Vector3::new(0.9, 0.85, 0.75);
        let ring_dark = Vector3::new(0.6, 0.55, 0.5);

        let ring_color = Vector3::new(
            ring_dark.x + (ring_bright.x - ring_dark.x) * ring_detail,
            ring_dark.y + (ring_bright.y - ring_dark.y) * ring_detail,
            ring_dark.z + (ring_bright.z - ring_dark.z) * ring_detail,
        );

        // Calculate lighting for rings (they're nearly flat, so simplified)
        let light_dir = uniforms.light_direction();
        let ring_light = (light_dir.y.abs() * 0.3 + 0.5).min(1.0);

        // Fade rings near edges (both inner and outer)
        let radial_pos =
            (distance_from_center - ring_inner_radius) / (ring_outer_radius - ring_inner_radius);
        let edge_fade = if radial_pos < 0.1 {
            radial_pos / 0.1
        } else if radial_pos > 0.9 {
            (1.0 - radial_pos) / 0.1
        } else {
            1.0
        };

        // Fade based on distance from equatorial plane
        let vertical_fade = 1.0 - (height_from_equator / ring_max_thickness);

        let total_fade = edge_fade * vertical_fade;

        let r = (ring_color.x * ring_light * 255.0 * total_fade)
            .min(255.0)
            .max(0.0) as u8;
        let g = (ring_color.y * ring_light * 255.0 * total_fade)
            .min(255.0)
            .max(0.0) as u8;
        let b = (ring_color.z * ring_light * 255.0 * total_fade)
            .min(255.0)
            .max(0.0) as u8;
        let a = (total_fade * 220.0).min(255.0).max(0.0) as u8;

        return Color::new(r, g, b, a);
    }

    // If not in ring zone, render planet surface

    // Normalize normal
    let normal_length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    let normalized_normal = Vector3::new(
        normal.x / normal_length,
        normal.y / normal_length,
        normal.z / normal_length,
    );

    // PLANET SURFACE LAYERS

    // LAYER 1: Base ice patterns (large crystalline structures)
    let ice_crystals = ((pos.x * 4.0 + uniforms.time() * 0.1).sin()
        * (pos.z * 4.0 - uniforms.time() * 0.08).cos())
    .abs();

    // LAYER 2: Frost patterns (medium detail)
    let frost = smooth_noise(
        pos.x * 15.0 + uniforms.time() * 0.15,
        pos.y * 15.0,
        pos.z * 15.0 - uniforms.time() * 0.12,
    );

    // LAYER 3: Fine snow/ice detail
    let snow_detail = ((pos.x * 40.0).sin() * (pos.y * 38.0).cos() * (pos.z * 42.0).sin()).abs();

    // LAYER 4: Atmospheric haze bands
    let haze_bands = ((pos.y * 8.0 + uniforms.time() * 0.2).sin() + 1.0) * 0.5;

    // LAYER 5: Polar aurora shimmer
    let aurora = ((pos.y.abs() - 0.5).max(0.0) * 2.0).min(1.0)
        * ((uniforms.time() * 3.0 + pos.x * 8.0).sin() * 0.5 + 0.5);

    // Combine ice layers
    let ice_pattern = ice_crystals * 0.4 + frost * 0.35 + snow_detail * 0.25;

    // Define ice color palette
    let deep_ice = Vector3::new(0.55, 0.7, 0.85);
    let light_ice = Vector3::new(0.85, 0.92, 0.98);
    let blue_ice = Vector3::new(0.6, 0.8, 0.95);
    let aurora_color = Vector3::new(0.4, 0.9, 0.7);

    // Map ice patterns to colors
    let base_color = if ice_pattern < 0.4 {
        let t = ice_pattern / 0.4;
        Vector3::new(
            deep_ice.x + (blue_ice.x - deep_ice.x) * t,
            deep_ice.y + (blue_ice.y - deep_ice.y) * t,
            deep_ice.z + (blue_ice.z - deep_ice.z) * t,
        )
    } else {
        let t = (ice_pattern - 0.4) / 0.6;
        Vector3::new(
            blue_ice.x + (light_ice.x - blue_ice.x) * t,
            blue_ice.y + (light_ice.y - blue_ice.y) * t,
            blue_ice.z + (light_ice.z - blue_ice.z) * t,
        )
    };

    // Add atmospheric haze
    let haze_color = Vector3::new(0.75, 0.85, 0.92);
    let with_haze = Vector3::new(
        base_color.x + (haze_color.x - base_color.x) * haze_bands * 0.2,
        base_color.y + (haze_color.y - base_color.y) * haze_bands * 0.2,
        base_color.z + (haze_color.z - base_color.z) * haze_bands * 0.2,
    );

    // Add aurora effect at poles
    let with_aurora = Vector3::new(
        with_haze.x + aurora_color.x * aurora * 0.3,
        with_haze.y + aurora_color.y * aurora * 0.3,
        with_haze.z + aurora_color.z * aurora * 0.3,
    );

    // Calculate lighting
    let light_dir = uniforms.light_direction();
    let light_intensity = (normalized_normal.x * light_dir.x
        + normalized_normal.y * light_dir.y
        + normalized_normal.z * light_dir.z)
        .max(0.0);

    // Add strong specular for icy surface - uses world_pos for view direction
    let view_dir = Vector3::new(
        uniforms.camera_position().x - world_pos.x,
        uniforms.camera_position().y - world_pos.y,
        uniforms.camera_position().z - world_pos.z,
    );
    let view_length =
        (view_dir.x * view_dir.x + view_dir.y * view_dir.y + view_dir.z * view_dir.z).sqrt();
    let normalized_view = Vector3::new(
        view_dir.x / view_length,
        view_dir.y / view_length,
        view_dir.z / view_length,
    );

    let reflect_dot = 2.0
        * (normalized_normal.x * light_dir.x
            + normalized_normal.y * light_dir.y
            + normalized_normal.z * light_dir.z);
    let reflect = Vector3::new(
        reflect_dot * normalized_normal.x - light_dir.x,
        reflect_dot * normalized_normal.y - light_dir.y,
        reflect_dot * normalized_normal.z - light_dir.z,
    );

    let spec_dot = (reflect.x * normalized_view.x
        + reflect.y * normalized_view.y
        + reflect.z * normalized_view.z)
        .max(0.0);
    let specular = spec_dot.powf(40.0) * 0.8; // Very shiny ice

    // Ring shadow on planet (darken equatorial regions) - NOW PROPERLY ALIGNED
    let ring_shadow_factor = if height_from_equator < 0.3 {
        0.4 // Dark shadow
    } else if height_from_equator < 0.5 {
        0.4 + (height_from_equator - 0.3) * 3.0 // Transition
    } else {
        1.0 // No shadow
    };

    // Final lighting combination
    let ambient = 0.2;
    let final_intensity = (ambient + light_intensity * 0.8 + specular) * ring_shadow_factor;

    let r = (with_aurora.x * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;
    let g = (with_aurora.y * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;
    let b = (with_aurora.z * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;

    Color::new(r, g, b, 255)
}

// Vertex shader for ringed planet - creates ring geometry from sphere vertices
fn ringed_vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let pos = vertex.position();
    let distance_from_axis = (pos.x * pos.x + pos.z * pos.z).sqrt();
    let height = pos.y.abs();

    // Ring parameters - vertices near equator get displaced
    let ring_start = 0.55;
    let ring_end = 0.60;
    let max_extension = 0.8;

    // Only affect vertices in the equatorial band
    if height < 0.15 && distance_from_axis > ring_start {
        // Calculate how much to flatten and extend
        let equator_factor = 1.0 - (height / 0.15); // 1.0 at equator, 0.0 at edge
        let radial_factor = ((distance_from_axis - ring_start) / (ring_end - ring_start))
            .min(1.0)
            .max(0.0);

        // Flatten toward equatorial plane
        let new_y = pos.y * (1.0 - equator_factor * 0.92);

        // Extend outward to form ring
        let extend_amount = equator_factor * radial_factor * max_extension;
        let direction_x = pos.x / distance_from_axis;
        let direction_z = pos.z / distance_from_axis;
        let new_radius = distance_from_axis + extend_amount;

        let new_position = Vector3::new(direction_x * new_radius, new_y, direction_z * new_radius);

        // Adjust normal to point up/down for ring surface
        let ring_strength = equator_factor * radial_factor;
        let base_normal = vertex.normal();
        let ring_normal = Vector3::new(0.0, pos.y.signum(), 0.0);

        let new_normal = Vector3::new(
            base_normal.x * (1.0 - ring_strength) + ring_normal.x * ring_strength,
            base_normal.y * (1.0 - ring_strength) + ring_normal.y * ring_strength,
            base_normal.z * (1.0 - ring_strength) + ring_normal.z * ring_strength,
        );

        return Vertex::new(new_position, new_normal, vertex.tex_coords());
    }

    vertex.clone()
}

// GJ 504 b - Magenta Gas Giant with deep purple swirls
fn magenta_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.object_position();
    let normal = fragment.normal();

    // Normalize normal
    let normal_length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    let normalized_normal = Vector3::new(
        normal.x / normal_length,
        normal.y / normal_length,
        normal.z / normal_length,
    );

    // LAYER 1: Deep magenta/purple atmospheric bands
    let bands = (pos.y * 8.0 + uniforms.time() * 0.3).sin();

    // LAYER 2: Swirling patterns with longitude variation
    let swirls = (pos.y * 12.0 + pos.x * 4.0 - uniforms.time() * 0.5).sin()
        * (pos.z * 3.5 + uniforms.time() * 0.4).cos();

    // LAYER 3: Fine turbulence for texture
    let turbulence = smooth_noise(
        pos.x * 20.0 + uniforms.time() * 0.6,
        pos.y * 18.0,
        pos.z * 20.0 - uniforms.time() * 0.5,
    );

    // LAYER 4: Glowing magenta storms
    let storm_x = pos.x - 0.3;
    let storm_y = pos.y + 0.2;
    let storm_dist = (storm_x * storm_x * 5.0 + storm_y * storm_y * 8.0).sqrt();
    let storm = ((1.0 - (storm_dist * 3.0).min(1.0)) * 0.7).max(0.0);

    // LAYER 5: Polar glow (intense magenta at poles)
    let polar_intensity = ((pos.y.abs() - 0.5).max(0.0) * 2.0).min(1.0);
    let polar_pulse = ((uniforms.time() * 2.5).sin() * 0.5 + 0.5);

    // Combine layers
    let pattern = bands * 0.35 + swirls * 0.35 + turbulence * 0.3;
    let t = (pattern + 1.0) * 0.5;

    // Magenta color palette
    let deep_magenta = Vector3::new(0.5, 0.1, 0.5);
    let bright_magenta = Vector3::new(0.9, 0.2, 0.9);
    let purple = Vector3::new(0.6, 0.15, 0.75);
    let pink = Vector3::new(0.95, 0.4, 0.85);
    let storm_glow = Vector3::new(1.0, 0.3, 1.0);

    let base = if t < 0.33 {
        let local_t = t / 0.33;
        Vector3::new(
            deep_magenta.x + (purple.x - deep_magenta.x) * local_t,
            deep_magenta.y + (purple.y - deep_magenta.y) * local_t,
            deep_magenta.z + (purple.z - deep_magenta.z) * local_t,
        )
    } else if t < 0.66 {
        let local_t = (t - 0.33) / 0.33;
        Vector3::new(
            purple.x + (bright_magenta.x - purple.x) * local_t,
            purple.y + (bright_magenta.y - purple.y) * local_t,
            purple.z + (bright_magenta.z - purple.z) * local_t,
        )
    } else {
        let local_t = (t - 0.66) / 0.34;
        Vector3::new(
            bright_magenta.x + (pink.x - bright_magenta.x) * local_t,
            bright_magenta.y + (pink.y - bright_magenta.y) * local_t,
            bright_magenta.z + (pink.z - bright_magenta.z) * local_t,
        )
    };

    // Add glowing storms
    let with_storm = Vector3::new(
        base.x + (storm_glow.x - base.x) * storm * 0.6,
        base.y + (storm_glow.y - base.y) * storm * 0.6,
        base.z + (storm_glow.z - base.z) * storm * 0.6,
    );

    // Add polar glow
    let with_polar = Vector3::new(
        with_storm.x + bright_magenta.x * polar_intensity * polar_pulse * 0.4,
        with_storm.y + bright_magenta.y * polar_intensity * polar_pulse * 0.4,
        with_storm.z + bright_magenta.z * polar_intensity * polar_pulse * 0.4,
    );

    // Lighting
    let light_dir = uniforms.light_direction();
    let light_intensity = (normalized_normal.x * light_dir.x
        + normalized_normal.y * light_dir.y
        + normalized_normal.z * light_dir.z)
        .max(0.0);

    let ambient = 0.4; // Self-illuminating appearance
    let final_intensity = ambient + light_intensity * 0.6;

    let r = (with_polar.x * final_intensity * 255.0).min(255.0).max(0.0) as u8;
    let g = (with_polar.y * final_intensity * 255.0).min(255.0).max(0.0) as u8;
    let b = (with_polar.z * final_intensity * 255.0).min(255.0).max(0.0) as u8;

    Color::new(r, g, b, 255)
}

// Kepler-22 b - Ocean World with deep blues and dynamic clouds
fn water_world_fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.object_position();
    let world_pos = fragment.world_position();
    let normal = fragment.normal();

    // Normalize normal
    let normal_length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    let normalized_normal = Vector3::new(
        normal.x / normal_length,
        normal.y / normal_length,
        normal.z / normal_length,
    );

    // LAYER 1: Ocean depth variations
    let ocean_depth = ((pos.x * 3.5 + uniforms.time() * 0.08).sin()
        * (pos.y * 2.8).cos()
        * (pos.z * 3.2 + uniforms.time() * 0.06).sin())
    .abs();

    // LAYER 2: Ocean currents and waves
    let currents = smooth_noise(
        pos.x * 10.0 + uniforms.time() * 0.3,
        pos.y * 9.0,
        pos.z * 10.0 - uniforms.time() * 0.25,
    );

    // LAYER 3: Scattered island chains (rare land masses)
    let islands = ((pos.x * 25.0).sin() * (pos.y * 20.0).cos() * (pos.z * 28.0).sin()).abs();
    let is_land = islands > 0.85 && ocean_depth > 0.6;

    // LAYER 4: Dynamic cloud systems
    let clouds = ((pos.x * 15.0 + uniforms.time() * 0.6).sin()
        * (pos.y * 12.0 - uniforms.time() * 0.5).cos()
        * (pos.z * 14.0 + uniforms.time() * 0.55).sin()
        + 1.0)
        * 0.5;

    // LAYER 5: Polar ice caps
    let polar_ice = (pos.y.abs() - 0.75).max(0.0) * 4.0;

    // Ocean color palette
    let deep_ocean = Vector3::new(0.02, 0.1, 0.25);
    let mid_ocean = Vector3::new(0.08, 0.25, 0.5);
    let shallow_ocean = Vector3::new(0.15, 0.45, 0.7);
    let tropical = Vector3::new(0.2, 0.6, 0.85);
    let land = Vector3::new(0.25, 0.35, 0.2);
    let ice = Vector3::new(0.85, 0.9, 0.95);

    let water_color = if is_land {
        land
    } else {
        let depth_value = ocean_depth + currents * 0.3;
        if depth_value < 0.3 {
            let t = depth_value / 0.3;
            Vector3::new(
                deep_ocean.x + (mid_ocean.x - deep_ocean.x) * t,
                deep_ocean.y + (mid_ocean.y - deep_ocean.y) * t,
                deep_ocean.z + (mid_ocean.z - deep_ocean.z) * t,
            )
        } else if depth_value < 0.6 {
            let t = (depth_value - 0.3) / 0.3;
            Vector3::new(
                mid_ocean.x + (shallow_ocean.x - mid_ocean.x) * t,
                mid_ocean.y + (shallow_ocean.y - mid_ocean.y) * t,
                mid_ocean.z + (shallow_ocean.z - mid_ocean.z) * t,
            )
        } else {
            let t = (depth_value - 0.6) / 0.4;
            Vector3::new(
                shallow_ocean.x + (tropical.x - shallow_ocean.x) * t,
                shallow_ocean.y + (tropical.y - shallow_ocean.y) * t,
                shallow_ocean.z + (tropical.z - shallow_ocean.z) * t,
            )
        }
    };

    // Add polar ice
    let with_ice = Vector3::new(
        water_color.x + (ice.x - water_color.x) * polar_ice.min(1.0),
        water_color.y + (ice.y - water_color.y) * polar_ice.min(1.0),
        water_color.z + (ice.z - water_color.z) * polar_ice.min(1.0),
    );

    // Add clouds
    let cloud_brightness = (clouds - 0.55).max(0.0) * 2.0;
    let cloud_color = Vector3::new(0.9, 0.92, 0.95);
    let with_clouds = Vector3::new(
        with_ice.x + cloud_color.x * cloud_brightness * 0.5,
        with_ice.y + cloud_color.y * cloud_brightness * 0.5,
        with_ice.z + cloud_color.z * cloud_brightness * 0.5,
    );

    // Lighting
    let light_dir = uniforms.light_direction();
    let light_intensity = (normalized_normal.x * light_dir.x
        + normalized_normal.y * light_dir.y
        + normalized_normal.z * light_dir.z)
        .max(0.0);

    // Strong water specular highlights
    let view_dir = Vector3::new(
        uniforms.camera_position().x - world_pos.x,
        uniforms.camera_position().y - world_pos.y,
        uniforms.camera_position().z - world_pos.z,
    );
    let view_length =
        (view_dir.x * view_dir.x + view_dir.y * view_dir.y + view_dir.z * view_dir.z).sqrt();
    let normalized_view = Vector3::new(
        view_dir.x / view_length,
        view_dir.y / view_length,
        view_dir.z / view_length,
    );

    let reflect_dot = 2.0
        * (normalized_normal.x * light_dir.x
            + normalized_normal.y * light_dir.y
            + normalized_normal.z * light_dir.z);
    let reflect = Vector3::new(
        reflect_dot * normalized_normal.x - light_dir.x,
        reflect_dot * normalized_normal.y - light_dir.y,
        reflect_dot * normalized_normal.z - light_dir.z,
    );

    let spec_dot = (reflect.x * normalized_view.x
        + reflect.y * normalized_view.y
        + reflect.z * normalized_view.z)
        .max(0.0);

    let specular = if is_land {
        0.0
    } else {
        spec_dot.powf(30.0) * 0.7
    };

    let ambient = 0.2;
    let final_intensity = ambient + light_intensity * 0.8 + specular;

    let r = (with_clouds.x * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;
    let g = (with_clouds.y * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;
    let b = (with_clouds.z * final_intensity * 255.0)
        .min(255.0)
        .max(0.0) as u8;

    Color::new(r, g, b, 255)
}
