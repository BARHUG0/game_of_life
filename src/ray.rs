//ray.rs
use raylib::prelude::*;

/// Represents the result of a raycast operation
#[derive(Debug, Clone)]
pub struct Ray {
    distance: f32,
    hit_point: Vector2,
    wall_type: char,
    side_hit: Side, // Which side of the wall was hit (for texture mapping)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Horizontal, // Hit horizontal wall (top/bottom)
    Vertical,   // Hit vertical wall (left/right)
}

impl Ray {
    pub fn new(distance: f32, hit_point: Vector2, wall_type: char, side_hit: Side) -> Self {
        Ray {
            distance,
            hit_point,
            wall_type,
            side_hit,
        }
    }

    // Getters
    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn hit_point(&self) -> Vector2 {
        self.hit_point
    }

    pub fn wall_type(&self) -> char {
        self.wall_type
    }

    pub fn side_hit(&self) -> Side {
        self.side_hit
    }

    // Setters
    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }

    pub fn set_hit_point(&mut self, hit_point: Vector2) {
        self.hit_point = hit_point;
    }

    pub fn set_wall_type(&mut self, wall_type: char) {
        self.wall_type = wall_type;
    }

    pub fn set_side_hit(&mut self, side_hit: Side) {
        self.side_hit = side_hit;
    }
}
