use crate::framebuffer::Framebuffer;
use crate::matrix::{
    create_model_matrix, create_projection_matrix, create_view_matrix, create_viewport_matrix,
    multiply_matrix_vector4,
};
use crate::triangle::triangle;
use crate::vertex::Vertex;
use raylib::prelude::*;
use std::f32::consts::PI;

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

fn apply_model_transform(vertex: &Vertex, model_matrix: &Matrix) -> Vector4 {
    let pos = vertex.position();
    let vertex4 = Vector4::new(pos.x, pos.y, pos.z, 1.0);
    multiply_matrix_vector4(model_matrix, &vertex4)
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

    let mut transformed_vertices = Vec::with_capacity(vertices.len());

    for vertex in vertices {
        let world_space = apply_model_transform(vertex, &model_matrix);
        let view_space = apply_view_transform(&world_space, &view_matrix);
        let clip_space = apply_projection_transform(&view_space, &projection_matrix);
        let ndc_space = perspective_divide(&clip_space);
        let screen_space = apply_viewport_transform(
            &Vector4::new(ndc_space.x, ndc_space.y, ndc_space.z, 1.0),
            &viewport_matrix,
        );

        transformed_vertices.push(Vector3::new(screen_space.x, screen_space.y, screen_space.z));
    }

    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangle(
                framebuffer,
                transformed_vertices[i],
                transformed_vertices[i + 1],
                transformed_vertices[i + 2],
            );
        }
    }
}
