use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::matrix::{
    create_projection_matrix, create_view_matrix, create_viewport_matrix, multiply_matrix_vector4,
};
use crate::orbital_math::generate_orbital_path;
use crate::render::Camera;
use crate::solar_system::{Planet, SolarSystem};
use raylib::prelude::*;

/// Centralized projection function - converts 3D world space to 2D screen space
fn project_to_screen(
    point: Vector3,
    camera: &Camera,
    framebuffer: &Framebuffer,
) -> Option<Vector2> {
    // Create transformation matrices
    let view_matrix = create_view_matrix(camera.position(), camera.target(), camera.up());
    let projection_matrix =
        create_projection_matrix(camera.fov(), camera.aspect(), camera.near(), camera.far());
    let viewport_matrix = create_viewport_matrix(
        0.0,
        0.0,
        framebuffer.width() as f32,
        framebuffer.height() as f32,
    );

    // Transform point through pipeline
    let point4 = Vector4::new(point.x, point.y, point.z, 1.0);
    let view_space = multiply_matrix_vector4(&view_matrix, &point4);

    // Discard points behind camera
    if view_space.z >= -camera.near() {
        return None;
    }

    let clip_space = multiply_matrix_vector4(&projection_matrix, &view_space);

    // Check for valid perspective divide
    if clip_space.w.abs() < 0.0001 {
        return None;
    }

    // Perspective divide to NDC
    let ndc = Vector3::new(
        clip_space.x / clip_space.w,
        clip_space.y / clip_space.w,
        clip_space.z / clip_space.w,
    );

    // Cull points outside view frustum
    if ndc.x < -1.5 || ndc.x > 1.5 || ndc.y < -1.5 || ndc.y > 1.5 {
        return None;
    }

    // Transform to screen space
    let screen_space =
        multiply_matrix_vector4(&viewport_matrix, &Vector4::new(ndc.x, ndc.y, ndc.z, 1.0));

    Some(Vector2::new(screen_space.x, screen_space.y))
}

/// Draw a line between two 3D points (projected to 2D)
fn draw_line_3d(
    framebuffer: &mut Framebuffer,
    p1: Vector3,
    p2: Vector3,
    camera: &Camera,
    color: Color,
) {
    if let (Some(screen1), Some(screen2)) = (
        project_to_screen(p1, camera, framebuffer),
        project_to_screen(p2, camera, framebuffer),
    ) {
        // Check if points are within screen bounds (with margin for clipping)
        let margin = 100.0;
        let width = framebuffer.width() as f32;
        let height = framebuffer.height() as f32;

        if screen1.x > -margin
            && screen1.x < width + margin
            && screen1.y > -margin
            && screen1.y < height + margin
            && screen2.x > -margin
            && screen2.x < width + margin
            && screen2.y > -margin
            && screen2.y < height + margin
        {
            framebuffer.set_foreground_color(color);
            line(framebuffer, screen1, screen2);
        }
    }
}

/// Render the full orbital path (ellipse) for a planet
/// NOW uses the centralized orbital_math functions!
pub fn render_orbital_path(
    framebuffer: &mut Framebuffer,
    planet: &Planet,
    center: Vector3,
    camera: &Camera,
) {
    // Use the centralized path generation - same math as planet movement!
    let path_points = generate_orbital_path(
        center,
        planet.orbital_params.semi_major_axis,
        planet.orbital_params.eccentricity,
        planet.orbital_params.inclination,
        100, // number of points for smooth ellipse
    );

    // Draw the complete orbit as connected line segments
    for i in 0..path_points.len() {
        let p1 = path_points[i];
        let p2 = path_points[(i + 1) % path_points.len()];

        // Faint color for orbital path (25% brightness)
        let base_color = planet.get_trail_color();
        let path_color = Color::new(base_color.r / 4, base_color.g / 4, base_color.b / 4, 80);

        draw_line_3d(framebuffer, p1, p2, camera, path_color);
    }
}

/// Render the trailing effect behind a planet
pub fn render_trail(framebuffer: &mut Framebuffer, planet: &Planet, camera: &Camera) {
    let trail_len = planet.trail_positions.len();
    if trail_len < 2 {
        return;
    }

    let base_color = planet.get_trail_color();

    // Draw trail segments with fade effect
    for i in 0..trail_len - 1 {
        let p1 = planet.trail_positions[i];
        let p2 = planet.trail_positions[i + 1];

        // Fade from transparent (old) to opaque (new)
        let age_factor = i as f32 / trail_len as f32;
        let alpha = (age_factor * 220.0) as u8;

        let trail_color = Color::new(base_color.r, base_color.g, base_color.b, alpha);

        draw_line_3d(framebuffer, p1, p2, camera, trail_color);
    }
}

/// Render all orbital paths and trails for the solar system
pub fn render_solar_system_orbits(
    framebuffer: &mut Framebuffer,
    solar_system: &SolarSystem,
    camera: &Camera,
) {
    let center_pos = solar_system.center.position();

    for planet in &solar_system.planets {
        if solar_system.show_orbital_paths {
            render_orbital_path(framebuffer, planet, center_pos, camera);
        }

        if solar_system.show_trails {
            render_trail(framebuffer, planet, camera);
        }
    }
}
