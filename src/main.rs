#![allow(warnings)]

mod conway;
mod framebuffer;
mod light;
mod line;
mod matrix;
mod obj;
mod triangle;

use matrix::{
    create_model_matrix, create_projection_matrix, create_view_matrix, create_viewport_matrix,
    multiply_matrix_vector4,
};
use raylib::prelude::*;

use framebuffer::Framebuffer;
use light::Light;
use line::line;
use obj::Obj;
use std::f32::consts::PI;
use triangle::triangle;

const MATRIX_WIDTH: usize = WINDOW_WIDTH as usize;
const MATRIX_HEIGHT: usize = WINDOW_HEIGHT as usize;

const WINDOW_WIDTH: i32 = 1900;
const WINDOW_HEIGHT: i32 = 1000;

const FREMEBUFFER_WIDTH: i32 = WINDOW_WIDTH;
const FRAMEBUFFER_HEIGHT: i32 = WINDOW_HEIGHT;
const MATRIX_CELL_SCALLING_FACTOR: usize = 1;

fn main() {
    game_loop();
}

fn transform_matrix(
    vertex: Vector3,
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
) -> Vector3 {
    let model: Matrix = create_model_matrix(translation, scale, rotation);

    let view: Matrix = create_view_matrix(
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.1),
    );

    let vertex4 = Vector4::new(vertex.x, vertex.y, vertex.z, 1.0);

    let transformed_vertex4 = multiply_matrix_vector4(&model, &vertex4);

    let transformed_vertex3 = Vector3::new(
        transformed_vertex4.x / transformed_vertex4.w,
        transformed_vertex4.y / transformed_vertex4.w,
        transformed_vertex4.z / transformed_vertex4.w,
    );

    let projection: Matrix = create_projection_matrix(PI / 3.0, 1900.0 / 1200.0, 0.1, 100.0);

    let viewport: Matrix = create_viewport_matrix(0.0, 0.0, 1900.0, 1200.0);

    let world_transform = multiply_matrix_vector4(&model, &vertex4);

    let view_transform = multiply_matrix_vector4(&view, &world_transform);

    let projection_transform = multiply_matrix_vector4(&projection, &view_transform);

    let transformed_vertex = multiply_matrix_vector4(&viewport, &projection_transform);
    transformed_vertex3
}

fn transform(vertex: Vector3, translation: Vector2, scale: f32, rotation: f32) -> Vector3 {
    let mut new_vertex = vertex;

    let cos_theta = (rotation * PI / 180.0).cos();
    let sin_theta = (rotation * PI / 180.0).sin();

    let rotated_x = new_vertex.x * cos_theta - new_vertex.y * sin_theta;
    let rotated_y = new_vertex.x * sin_theta + new_vertex.y * cos_theta;

    new_vertex.x = rotated_x;
    new_vertex.y = rotated_y;

    new_vertex.x *= scale;
    new_vertex.y *= scale;

    new_vertex.x += translation.x;
    new_vertex.y += translation.y;

    new_vertex
}

fn render(
    framebuffer: &mut Framebuffer,
    translation: Vector2,
    scale: f32,
    rotation: f32,
    vertex_array: &[Vector3],
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = transform(vertex.clone(), translation, scale, rotation);
        transformed_vertices.push(transformed);
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

    /*
    let start = Vector2::new(0.0, 0.0);
    let end = Vector2::new(300.0, 300.0);

    line(framebuffer, start, end);

    let v1 = Vector3::new(500.0, 500.0, 0.0);
    let v2 = Vector3::new(600.0, 500.0, 0.0);
    let v3 = Vector3::new(550.0, 600.0, 0.0);

    //let center = Vector3::new((v1.x + v2.x + v3.x) / 3.0, (v1.y + v2.y + v3.y) / 3.0, 0.0);

    let tv1 = transform(v1, translation, scale, rotation);
    let tv2 = transform(v2, translation, scale, rotation);
    let tv3 = transform(v3, translation, scale, rotation);

    triangle(framebuffer, tv1, tv2, tv3);
    */
}

fn compute_model_center(vertices: &[Vector3]) -> Vector3 {
    let (mut min_x, mut min_y, mut min_z) = (f32::MAX, f32::MAX, f32::MAX);
    let (mut max_x, mut max_y, mut max_z) = (f32::MIN, f32::MIN, f32::MIN);

    for v in vertices {
        min_x = min_x.min(v.x);
        min_y = min_y.min(v.y);
        min_z = min_z.min(v.z);

        max_x = max_x.max(v.x);
        max_y = max_y.max(v.y);
        max_z = max_z.max(v.z);
    }

    Vector3::new(
        (min_x + max_x) / 2.0,
        (min_y + max_y) / 2.0,
        (min_z + max_z) / 2.0,
    )
}

fn game_loop() {
    let (mut handle, raylib_thread) = raylib::init()
        .undecorated()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("raylib")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(FREMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT, Color::WHITE);
    //    let mut translation = Vector2::new(0.0, 0.0);
    let mut rotation = 1.0;
    let mut scale = 100.0;

    let mut translation = Vector2::new(
        FREMEBUFFER_WIDTH as f32 / 2.0,
        FRAMEBUFFER_HEIGHT as f32 / 2.0,
    );

    let obj = Obj::load("models/nave.obj").expect("Failed to load Obj");
    let vertex_array = obj.get_vertex_array();
    let model_center = compute_model_center(&vertex_array);
    println!(
        "Vertices: {}, Indices: {}",
        obj.vertices.len(),
        obj.indices.len()
    );

    framebuffer.set_background_color(Color::new(4, 12, 36, 255));

    while !&handle.window_should_close() {
        framebuffer.clear();
        framebuffer.set_foreground_color(Color::new(100, 200, 255, 255));

        if handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            translation.x += 1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_LEFT) {
            translation.x -= 1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_UP) {
            translation.y -= 1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_DOWN) {
            translation.y += 1.0;
        }
        if handle.is_key_down(KeyboardKey::KEY_D) {
            scale -= 0.05;
        }
        if handle.is_key_down(KeyboardKey::KEY_U) {
            scale += 0.05;
        }

        /*        render(
                    &mut framebuffer,
                    translation,
                    scale,
                    rotation,
                    &vertex_array,
                );
        */

        improved_render(
            &mut framebuffer,
            translation,
            scale,
            rotation,
            &vertex_array,
            model_center,
        );

        let texture = handle
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .expect("The texture loaded from the color buffer should be valid");

        let mut draw_handle = handle.begin_drawing(&raylib_thread);
        {
            draw_handle.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}

fn improved_transform(
    vertex: Vector3,
    model_center: Vector3,
    translation: Vector2,
    scale: f32,
    rotation: f32,
) -> Vector3 {
    // --- Move vertex so model center is at origin ---
    let mut v = vertex - model_center;

    // --- Basic 3D rotation around Z-axis (for now) ---
    let cos_theta = (rotation * PI / 180.0).cos();
    let sin_theta = (rotation * PI / 180.0).sin();

    let x_rot = v.x * cos_theta - v.y * sin_theta;
    let y_rot = v.x * sin_theta + v.y * cos_theta;
    let z_rot = v.z;

    v = Vector3::new(x_rot, y_rot, z_rot);

    // --- Simple perspective projection ---
    let z = v.z + 3.0; // push object away from camera
    let px = v.x / z;
    let py = v.y / z;

    // --- Scale and translate to screen center ---
    let x = px * scale + translation.x;
    let y = py * scale + translation.y;

    Vector3::new(x, y, z)
}

fn improved_render(
    framebuffer: &mut Framebuffer,
    translation: Vector2,
    scale: f32,
    rotation: f32,
    vertex_array: &[Vector3],
    model_center: Vector3,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = improved_transform(*vertex, model_center, translation, scale, rotation);
        transformed_vertices.push(transformed);
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
