use raylib::prelude::*;

pub struct Camera {
    pub eye: Vector3,
    pub center: Vector3,
    pub up: Vector3,
    pub forward: Vector3,
    pub right: Vector3,

    // Spherical coordinates
    pub distance: f32,
    yaw: f32,   // Horizontal rotation around world Y axis (radians)
    pitch: f32, // Vertical angle from horizontal plane (radians)
}

impl Camera {
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        let relative_pos = eye - center;
        let distance = relative_pos.length();

        // Calculate initial spherical coordinates from position
        let yaw = relative_pos.z.atan2(relative_pos.x);
        let horizontal_dist =
            (relative_pos.x * relative_pos.x + relative_pos.z * relative_pos.z).sqrt();
        let pitch = relative_pos.y.atan2(horizontal_dist);

        let mut camera = Camera {
            eye,
            center,
            up,
            forward: Vector3::zero(),
            right: Vector3::zero(),
            distance,
            yaw,
            pitch,
        };

        camera.update_from_spherical();
        camera
    }

    fn update_from_spherical(&mut self) {
        // Clamp pitch to avoid gimbal lock at poles
        // Leave a small margin to prevent looking exactly straight up/down
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01; // ~89 degrees
        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);

        // Convert spherical coordinates to Cartesian position
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();

        // Calculate relative position from center
        let relative_pos = Vector3::new(
            self.distance * cos_pitch * cos_yaw,
            self.distance * sin_pitch,
            self.distance * cos_pitch * sin_yaw,
        );

        // Update eye position
        self.eye = self.center + relative_pos;

        // Update basis vectors
        self.forward = (self.center - self.eye).normalized();
        self.right = self.forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
        self.up = self.right.cross(self.forward).normalized();
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // Update spherical angles
        self.yaw -= delta_yaw; // Negative because left should increase yaw
        self.pitch += delta_pitch; // Positive pitch looks up

        // Reconstruct position from new angles
        self.update_from_spherical();
    }

    pub fn translate(&mut self, forward: f32, right: f32, up: f32) {
        let movement =
            self.forward * forward + self.right * right + Vector3::new(0.0, 1.0, 0.0) * up;

        self.eye += movement;
        self.center += movement;

        // Recalculate distance but keep angles the same
        let relative_pos = self.eye - self.center;
        self.distance = relative_pos.length();

        // No need to update spherical angles, they stay relative to center
        self.update_from_spherical();
    }

    pub fn basis_change(&self, p: &Vector3) -> Vector3 {
        Vector3::new(
            p.x * self.right.x + p.y * self.up.x - p.z * self.forward.x,
            p.x * self.right.y + p.y * self.up.y - p.z * self.forward.y,
            p.x * self.right.z + p.y * self.up.z - p.z * self.forward.z,
        )
    }

    // Removed update_basis() - no longer needed as a public method
    // Everything is handled through update_from_spherical()
}
