use crate::fragment::Fragment;
use crate::framebuffer::Framebuffer;
use crate::matrix::{
    create_model_matrix, create_projection_matrix, create_view_matrix, create_viewport_matrix,
    multiply_matrix_vector4,
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
    rotation: Vector3,
    camera: &Camera,
    mode: RenderMode,
    shader: ShaderType,
    uniforms: &Uniforms,
) {
    let model_matrix = create_model_matrix(translation, scale, rotation);
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

fn filled_triangle(
    framebuffer: &mut Framebuffer,
    v1: &TransformedVertex,
    v2: &TransformedVertex,
    v3: &TransformedVertex,
    shader: ShaderType,
    uniforms: &Uniforms,
) {
    let min_x = v1
        .screen_position
        .x
        .min(v2.screen_position.x)
        .min(v3.screen_position.x)
        .floor() as i32;
    let min_y = v1
        .screen_position
        .y
        .min(v2.screen_position.y)
        .min(v3.screen_position.y)
        .floor() as i32;
    let max_x = v1
        .screen_position
        .x
        .max(v2.screen_position.x)
        .max(v3.screen_position.x)
        .ceil() as i32;
    let max_y = v1
        .screen_position
        .y
        .max(v2.screen_position.y)
        .max(v3.screen_position.y)
        .ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (w, v, u) = barycentric(
                x as f32,
                y as f32,
                v1.screen_position,
                v2.screen_position,
                v3.screen_position,
            );

            if w >= 0.0 && v >= 0.0 && u >= 0.0 {
                if x >= 0 && y >= 0 && x < framebuffer.width() && y < framebuffer.height() {
                    let depth = w * v1.screen_position.z
                        + v * v2.screen_position.z
                        + u * v3.screen_position.z;

                    // Depth test - skip if fragment is behind existing pixel
                    if depth >= framebuffer.get_depth(x, y) {
                        continue;
                    }

                    // Update depth buffer
                    framebuffer.set_depth(x, y, depth);

                    // Interpolate world position
                    let world_pos = Vector3::new(
                        w * v1.world_position.x + v * v2.world_position.x + u * v3.world_position.x,
                        w * v1.world_position.y + v * v2.world_position.y + u * v3.world_position.y,
                        w * v1.world_position.z + v * v2.world_position.z + u * v3.world_position.z,
                    );

                    // Interpolate object position
                    let object_pos = Vector3::new(
                        w * v1.object_position.x
                            + v * v2.object_position.x
                            + u * v3.object_position.x,
                        w * v1.object_position.y
                            + v * v2.object_position.y
                            + u * v3.object_position.y,
                        w * v1.object_position.z
                            + v * v2.object_position.z
                            + u * v3.object_position.z,
                    );

                    let normal = Vector3::new(
                        w * v1.normal.x + v * v2.normal.x + u * v3.normal.x,
                        w * v1.normal.y + v * v2.normal.y + u * v3.normal.y,
                        w * v1.normal.z + v * v2.normal.z + u * v3.normal.z,
                    );

                    let fragment = Fragment::new(
                        Vector2::new(x as f32, y as f32),
                        world_pos,
                        object_pos,
                        normal,
                        depth,
                    );

                    let color = if let Some(color) = shader.fragment_shader(&fragment, uniforms) {
                        // Use shader color
                        color
                    } else {
                        // Default solid color with simple diffuse lighting
                        let light_dir = uniforms.light_direction();
                        let normal_length =
                            (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z)
                                .sqrt();
                        let normalized_normal = Vector3::new(
                            normal.x / normal_length,
                            normal.y / normal_length,
                            normal.z / normal_length,
                        );

                        let light_intensity = (normalized_normal.x * light_dir.x
                            + normalized_normal.y * light_dir.y
                            + normalized_normal.z * light_dir.z)
                            .max(0.0);

                        let ambient = 0.3;
                        let final_intensity = ambient + light_intensity * 0.7;

                        let r = (0.7 * final_intensity * 255.0) as u8;
                        let g = (0.7 * final_intensity * 255.0) as u8;
                        let b = (0.7 * final_intensity * 255.0) as u8;

                        Color::new(r, g, b, 255)
                    };

                    framebuffer.set_foreground_color(color);
                    framebuffer.set_pixel(x, y);
                }
            }
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
