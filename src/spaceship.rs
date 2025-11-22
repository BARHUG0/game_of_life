use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Spaceship {
    pub position: Vector3,
    pub velocity: Vector3,
    pub scale: f32,

    // Rotation as pitch/yaw only (no roll for FPS-style)
    pub pitch: f32, // Look up/down
    pub yaw: f32,   // Turn left/right

    // Cached direction vectors
    pub forward: Vector3,
    right: Vector3,
    up: Vector3,

    // Visual rotation (for rendering) - smoothly follows actual direction
    pub visual_forward: Vector3,

    pub visual_right: Vector3,
    pub visual_up: Vector3,

    // Banking effect
    bank_angle: f32,
    target_bank_angle: f32,

    // Physics
    acceleration: f32,
    strafe_speed: f32,
    max_speed: f32,
    drag: f32,
    turn_speed: f32,

    // Interpolation speeds
    rotation_smoothing: f32,
    bank_smoothing: f32,
}

impl Spaceship {
    pub fn new(position: Vector3, scale: f32) -> Self {
        let mut ship = Self {
            position,
            velocity: Vector3::zero(),
            scale,
            pitch: 0.0,
            yaw: 0.0,
            forward: Vector3::new(0.0, 0.0, -1.0), // Temporary
            right: Vector3::new(1.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            visual_forward: Vector3::new(0.0, 0.0, -1.0), // Temporary
            visual_right: Vector3::new(1.0, 0.0, 0.0),
            visual_up: Vector3::new(0.0, 1.0, 0.0),
            bank_angle: 0.0,
            target_bank_angle: 0.0,
            acceleration: 50.0,
            strafe_speed: 40.0,
            max_speed: 100.0,
            drag: 0.98,
            turn_speed: 3.5,
            rotation_smoothing: 5.0,
            bank_smoothing: 15.0,
        };

        // Initialize directions properly
        ship.update_directions();

        // Make visual match actual (no interpolation needed at start)
        ship.visual_forward = ship.forward;
        ship.visual_right = ship.right;
        ship.visual_up = ship.up;

        ship
    }

    fn update_directions(&mut self) {
        let (sin_y, cos_y) = self.yaw.sin_cos();
        let (sin_p, cos_p) = self.pitch.sin_cos();

        // For Z-forward coordinate system (Z decreases = forward)
        self.forward = Vector3::new(
            sin_y * cos_p,  // X = side-to-side based on yaw
            -sin_p,         // Y = up/down based on pitch
            -cos_y * cos_p, // Z = forward (negative Z is forward!)
        );

        // Right vector (perpendicular on XZ plane)
        self.right = Vector3::new(cos_y, 0.0, sin_y);

        // Up vector (cross product)
        self.up = self.cross(self.right, self.forward);

        // Normalize all
        self.forward = self.normalize(self.forward);
        self.right = self.normalize(self.right);
        self.up = self.normalize(self.up);
    }

    fn normalize(&self, v: Vector3) -> Vector3 {
        let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        if len > 0.0001 {
            Vector3::new(v.x / len, v.y / len, v.z / len)
        } else {
            v
        }
    }

    fn cross(&self, a: Vector3, b: Vector3) -> Vector3 {
        Vector3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    fn lerp_vector3(&self, a: Vector3, b: Vector3, t: f32) -> Vector3 {
        Vector3::new(
            a.x + (b.x - a.x) * t,
            a.y + (b.y - a.y) * t,
            a.z + (b.z - a.z) * t,
        )
    }

    pub fn update(
        &mut self,
        dt: f32,
        forward_input: f32, // W/S: -1 to 1
        strafe_input: f32,  // A/D: -1 to 1
        pitch_input: f32,   // Up/Down arrows
        yaw_input: f32,     // Left/Right arrows
    ) {
        // Update rotation
        self.pitch += pitch_input * self.turn_speed * dt;
        self.yaw += yaw_input * self.turn_speed * dt;

        // Clamp pitch to prevent over-rotation
        self.pitch = self.pitch.clamp(-PI / 2.2, PI / 2.2);

        // Update direction vectors
        //
        self.update_directions();

        // Apply forward/backward thrust
        if forward_input.abs() > 0.001 {
            println!(
                "Applying thrust! forward vec: {:?}, accel: {}",
                self.forward, self.acceleration
            );
            self.velocity.x += self.forward.x * self.acceleration * forward_input * dt;
            self.velocity.y += self.forward.y * self.acceleration * forward_input * dt;
            self.velocity.z += self.forward.z * self.acceleration * forward_input * dt;
        }

        println!("Velocity after update: {:?}", self.velocity);

        // Apply strafe (left/right) thrust
        if strafe_input.abs() > 0.001 {
            self.velocity.x += self.right.x * self.strafe_speed * strafe_input * dt;
            self.velocity.y += self.right.y * self.strafe_speed * strafe_input * dt;
            self.velocity.z += self.right.z * self.strafe_speed * strafe_input * dt;
        }

        // Calculate target bank angle based on strafe input
        self.target_bank_angle = -strafe_input * PI / 6.0; // Max 30 degrees bank

        // Smooth bank angle interpolation
        let bank_diff = self.target_bank_angle - self.bank_angle;
        self.bank_angle += bank_diff * self.bank_smoothing * dt;

        // Apply drag
        self.velocity.x *= self.drag;
        self.velocity.y *= self.drag;
        self.velocity.z *= self.drag;

        // Clamp speed
        let speed = (self.velocity.x * self.velocity.x
            + self.velocity.y * self.velocity.y
            + self.velocity.z * self.velocity.z)
            .sqrt();

        if speed > self.max_speed {
            let factor = self.max_speed / speed;
            self.velocity.x *= factor;
            self.velocity.y *= factor;
            self.velocity.z *= factor;
        }

        // Update position
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.position.z += self.velocity.z * dt;

        println!("Position after velocity applied: {:?}", self.position);

        // Smoothly interpolate visual rotation toward actual direction
        let interp_factor = (self.rotation_smoothing * dt).min(1.0);

        self.visual_forward =
            self.normalize(self.lerp_vector3(self.visual_forward, self.forward, interp_factor));

        self.visual_right =
            self.normalize(self.lerp_vector3(self.visual_right, self.right, interp_factor));

        self.visual_up = self.normalize(self.lerp_vector3(self.visual_up, self.up, interp_factor));

        // Recalculate visual_up to ensure orthogonality with banking
        self.visual_up = self.normalize(self.cross(self.visual_forward, self.visual_right));
        self.visual_right = self.normalize(self.cross(self.visual_up, self.visual_forward));
    }

    pub fn get_camera_position(&self, distance: f32, height: f32) -> Vector3 {
        // Camera looks from behind and slightly above
        // Use visual_forward to match rendered ship orientation
        Vector3::new(
            self.position.x - self.visual_forward.x * distance + self.visual_up.x * height,
            self.position.y - self.visual_forward.y * distance + self.visual_up.y * height,
            self.position.z - self.visual_forward.z * distance + self.visual_up.z * height,
        )
    }

    pub fn get_camera_target(&self, look_ahead: f32) -> Vector3 {
        // Target is directly ahead of the ship's current position
        // This ensures camera always looks where ship is pointing
        Vector3::new(
            self.position.x + self.visual_forward.x * look_ahead,
            self.position.y + self.visual_forward.y * look_ahead,
            self.position.z + self.visual_forward.z * look_ahead,
        )
    }

    pub fn get_up_vector(&self) -> Vector3 {
        // Use the ship's visual up vector (with banking applied) for camera up
        // This makes the camera roll with the ship
        self.visual_up
    }
    pub fn get_rotation_matrix(&self) -> Matrix {
        let (sin_bank, cos_bank) = self.bank_angle.sin_cos();

        let banked_right = Vector3::new(
            self.visual_right.x * cos_bank + self.visual_up.x * sin_bank,
            self.visual_right.y * cos_bank + self.visual_up.y * sin_bank,
            self.visual_right.z * cos_bank + self.visual_up.z * sin_bank,
        );

        let banked_up = Vector3::new(
            self.visual_up.x * cos_bank - self.visual_right.x * sin_bank,
            self.visual_up.y * cos_bank - self.visual_right.y * sin_bank,
            self.visual_up.z * cos_bank - self.visual_right.z * sin_bank,
        );

        Matrix {
            m0: banked_right.z,
            m4: banked_up.z,
            m8: -self.visual_forward.z,
            m12: 0.0,
            m1: banked_right.y,
            m5: banked_up.y,
            m9: -self.visual_forward.y,
            m13: 0.0,
            m2: banked_right.x,
            m6: banked_up.x,
            m10: -self.visual_forward.x,
            m14: 0.0,
            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    // Keep this for backward compatibility but mark as deprecated
    #[deprecated(note = "Use get_rotation_matrix() instead")]
    pub fn rotation(&self) -> Vector3 {
        Vector3::new(self.pitch, self.yaw, 0.0)
    }
}

impl Spaceship {
    // Add this method

    pub fn get_euler_rotation(&self) -> Vector3 {
        // Return pitch, yaw, and bank as Euler angles
        // Adjust these to match your model's default orientation
        Vector3::new(
            self.pitch,
            self.yaw + PI, // Add PI (180°) to flip the model around if it's facing backward
            self.bank_angle,
        )
    }
}
