use crate::fragment::Fragment;
use crate::framebuffer::Framebuffer;
use crate::matrix::{
    create_model_matrix, create_model_matrix_from_rotation_matrix, create_projection_matrix,
    create_view_matrix, create_viewport_matrix, multiply_matrix_vector4,
};
use crate::shader::ShaderType;
use crate::triangle::triangle;
use crate::uniforms::Uniforms;
use crate::vertex::Vertex;
use raylib::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum RenderMode {
    Wireframe,
    Solid,
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Euler(Vector3), // Traditional pitch/yaw/roll angles
    Matrix(Matrix), // Pre-built rotation matrix
}

pub struct Camera {
    position: Vector3,
    target: Vector3,
    up: Vector3,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(position: Vector3, target: Vector3, up: Vector3, fov: f32, aspect: f32) -> Self {
        Camera {
            position,
            target,
            up,
            fov,
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }

    pub fn target(&self) -> Vector3 {
        self.target
    }

    pub fn up(&self) -> Vector3 {
        self.up
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }
}

struct TransformedVertex {
    screen_position: Vector3,
    world_position: Vector3,
    object_position: Vector3,
    normal: Vector3,
    tex_coords: Vector2,
}

fn apply_model_transform(vertex: &Vertex, model_matrix: &Matrix) -> (Vector4, Vector3) {
    let pos = vertex.position();
    let vertex4 = Vector4::new(pos.x, pos.y, pos.z, 1.0);
    let transformed_pos = multiply_matrix_vector4(model_matrix, &vertex4);

    // Transform normal (using the model matrix, ignoring translation)
    let normal = vertex.normal();
    let normal4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);
    let transformed_normal = multiply_matrix_vector4(model_matrix, &normal4);

    (
        transformed_pos,
        Vector3::new(
            transformed_normal.x,
            transformed_normal.y,
            transformed_normal.z,
        ),
    )
}

fn apply_view_transform(vertex: &Vector4, view_matrix: &Matrix) -> Vector4 {
    multiply_matrix_vector4(view_matrix, vertex)
}

fn apply_projection_transform(vertex: &Vector4, projection_matrix: &Matrix) -> Vector4 {
    multiply_matrix_vector4(projection_matrix, vertex)
}

fn apply_viewport_transform(vertex: &Vector4, viewport_matrix: &Matrix) -> Vector4 {
    multiply_matrix_vector4(viewport_matrix, vertex)
}

fn perspective_divide(vertex: &Vector4) -> Vector3 {
    Vector3::new(
        vertex.x / vertex.w,
        vertex.y / vertex.w,
        vertex.z / vertex.w,
    )
}

pub fn render_model(
    framebuffer: &mut Framebuffer,
    vertices: &[Vertex],
    translation: Vector3,
    scale: f32,
    rotation: Rotation, // Changed from Vector3
    camera: &Camera,
    mode: RenderMode,
    shader: ShaderType,
    uniforms: &Uniforms<'_>,
) {
    // Create model matrix using the new create_model_matrix_with_rotation
    let model_matrix = match rotation {
        Rotation::Euler(angles) => create_model_matrix(translation, scale, angles),
        Rotation::Matrix(rot_matrix) => {
            create_model_matrix_from_rotation_matrix(translation, scale, rot_matrix)
        }
    };

    let view_matrix = create_view_matrix(camera.position(), camera.target(), camera.up());
    let projection_matrix =
        create_projection_matrix(camera.fov(), camera.aspect(), camera.near(), camera.far());
    let viewport_matrix = create_viewport_matrix(
        0.0,
        0.0,
        framebuffer.width() as f32,
        framebuffer.height() as f32,
    );

    let mut transformed_vertices: Vec<TransformedVertex> = Vec::with_capacity(vertices.len());

    for vertex in vertices {
        // Apply vertex shader FIRST (in model/object space)
        let modified_vertex = shader.vertex_shader(vertex, uniforms);

        // Store original object-space position
        let object_pos = modified_vertex.position();

        let (world_space, world_normal) = apply_model_transform(&modified_vertex, &model_matrix);
        let view_space = apply_view_transform(&world_space, &view_matrix);
        let clip_space = apply_projection_transform(&view_space, &projection_matrix);
        let ndc_space = perspective_divide(&clip_space);
        let screen_space = apply_viewport_transform(
            &Vector4::new(ndc_space.x, ndc_space.y, ndc_space.z, 1.0),
            &viewport_matrix,
        );

        transformed_vertices.push(TransformedVertex {
            screen_position: Vector3::new(screen_space.x, screen_space.y, screen_space.z),
            world_position: Vector3::new(world_space.x, world_space.y, world_space.z),
            object_position: object_pos,
            normal: world_normal,
            tex_coords: modified_vertex.tex_coords(),
        });
    }

    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            let v1 = &transformed_vertices[i];
            let v2 = &transformed_vertices[i + 1];
            let v3 = &transformed_vertices[i + 2];

            match mode {
                RenderMode::Wireframe => {
                    triangle(
                        framebuffer,
                        v1.screen_position,
                        v2.screen_position,
                        v3.screen_position,
                    );
                }
                RenderMode::Solid => {
                    filled_triangle(framebuffer, v1, v2, v3, shader, uniforms);
                }
            }
        }
    }
}

// Add to render.rs - updated filled_triangle function

fn filled_triangle(
    framebuffer: &mut Framebuffer,
    v1: &TransformedVertex,
    v2: &TransformedVertex,
    v3: &TransformedVertex,
    shader: ShaderType,
    uniforms: &Uniforms<'_>,
) {
    let fb_width = framebuffer.width();
    let fb_height = framebuffer.height();

    // Clamp to screen bounds FIRST
    let min_x = v1
        .screen_position
        .x
        .min(v2.screen_position.x)
        .min(v3.screen_position.x)
        .floor()
        .max(0.0) as i32;

    let min_y = v1
        .screen_position
        .y
        .min(v2.screen_position.y)
        .min(v3.screen_position.y)
        .floor()
        .max(0.0) as i32;

    let max_x = v1
        .screen_position
        .x
        .max(v2.screen_position.x)
        .max(v3.screen_position.x)
        .ceil()
        .min(fb_width as f32) as i32;

    let max_y = v1
        .screen_position
        .y
        .max(v2.screen_position.y)
        .max(v3.screen_position.y)
        .ceil()
        .min(fb_height as f32) as i32;

    // CRITICAL: Reject triangles that are too large (performance killer!)
    let area = (max_x - min_x) * (max_y - min_y);
    const MAX_TRIANGLE_AREA: i32 = 500_000; // ~700x700 pixels max

    if area > MAX_TRIANGLE_AREA {
        return; // Skip rendering massive triangles
    }

    // Early reject if completely outside screen
    if max_x < 0 || max_y < 0 || min_x >= fb_width || min_y >= fb_height {
        return;
    }

    // Pre-calculate barycentric denominator (optimization)
    let denom = (v2.screen_position.y - v3.screen_position.y)
        * (v1.screen_position.x - v3.screen_position.x)
        + (v3.screen_position.x - v2.screen_position.x)
            * (v1.screen_position.y - v3.screen_position.y);

    if denom.abs() < 1e-10 {
        return; // Degenerate triangle
    }

    let inv_denom = 1.0 / denom;

    for y in min_y..max_y {
        for x in min_x..max_x {
            // Optimized barycentric calculation
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            let w = ((v2.screen_position.y - v3.screen_position.y) * (px - v3.screen_position.x)
                + (v3.screen_position.x - v2.screen_position.x) * (py - v3.screen_position.y))
                * inv_denom;

            // Early rejection
            if w < 0.0 || w > 1.0 {
                continue;
            }

            let v = ((v3.screen_position.y - v1.screen_position.y) * (px - v3.screen_position.x)
                + (v1.screen_position.x - v3.screen_position.x) * (py - v3.screen_position.y))
                * inv_denom;

            if v < 0.0 || v > 1.0 {
                continue;
            }

            let u = 1.0 - w - v;
            if u < 0.0 {
                continue;
            }

            // Depth test
            let depth =
                w * v1.screen_position.z + v * v2.screen_position.z + u * v3.screen_position.z;
            if depth >= framebuffer.get_depth(x, y) {
                continue;
            }

            framebuffer.set_depth(x, y, depth);

            // Interpolate attributes
            let world_pos = Vector3::new(
                w * v1.world_position.x + v * v2.world_position.x + u * v3.world_position.x,
                w * v1.world_position.y + v * v2.world_position.y + u * v3.world_position.y,
                w * v1.world_position.z + v * v2.world_position.z + u * v3.world_position.z,
            );

            let object_pos = Vector3::new(
                w * v1.object_position.x + v * v2.object_position.x + u * v3.object_position.x,
                w * v1.object_position.y + v * v2.object_position.y + u * v3.object_position.y,
                w * v1.object_position.z + v * v2.object_position.z + u * v3.object_position.z,
            );

            let normal = Vector3::new(
                w * v1.normal.x + v * v2.normal.x + u * v3.normal.x,
                w * v1.normal.y + v * v2.normal.y + u * v3.normal.y,
                w * v1.normal.z + v * v2.normal.z + u * v3.normal.z,
            );

            let tex_coords = Vector2::new(
                w * v1.tex_coords.x + v * v2.tex_coords.x + u * v3.tex_coords.x,
                w * v1.tex_coords.y + v * v2.tex_coords.y + u * v3.tex_coords.y,
            );

            let fragment = Fragment::new(
                Vector2::new(px, py),
                world_pos,
                object_pos,
                normal,
                depth,
                tex_coords,
            );

            let color = if let Some(color) = shader.fragment_shader(&fragment, uniforms) {
                color
            } else {
                // Default lighting (same as before)
                let light_dir = uniforms.light_direction();
                let n_len =
                    (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
                let norm = Vector3::new(normal.x / n_len, normal.y / n_len, normal.z / n_len);

                let intensity =
                    (norm.x * light_dir.x + norm.y * light_dir.y + norm.z * light_dir.z).max(0.0);
                let final_int = 0.3 + intensity * 0.7;

                Color::new(
                    (0.7 * final_int * 255.0) as u8,
                    (0.7 * final_int * 255.0) as u8,
                    (0.7 * final_int * 255.0) as u8,
                    255,
                )
            };

            framebuffer.set_foreground_color(color);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn barycentric(p_x: f32, p_y: f32, a: Vector3, b: Vector3, c: Vector3) -> (f32, f32, f32) {
    let area = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);

    if area.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    let w = ((b.y - c.y) * (p_x - c.x) + (c.x - b.x) * (p_y - c.y)) / area;
    let v = ((c.y - a.y) * (p_x - c.x) + (a.x - c.x) * (p_y - c.y)) / area;
    let u = 1.0 - w - v;

    (w, v, u)
}
