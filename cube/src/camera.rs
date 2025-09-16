use raylib::prelude::*;

/// A 3D camera that maintains its position and orientation in world space
pub struct Camera {
    pub eye: Vector3,     // Camera position in world coordinates
    pub center: Vector3,  // Point the camera is looking at
    pub up: Vector3,      // Up direction (initially world up, gets orthonormalized)
    pub forward: Vector3, // Direction camera is facing (computed from eye->center)
    pub right: Vector3,   // Right direction (perpendicular to forward and up)
}

impl Camera {
    /// Creates a new camera and computes its initial orientation
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        // Initialize camera with given parameters and zero vectors for computed values
        let mut camera = Camera {
            eye,
            center,
            up,
            forward: Vector3::zero(), // Will be computed
            right: Vector3::zero(),   // Will be computed
        };
        // Compute the orthonormal basis vectors (forward, right, up)
        camera.update_basis_vectors();
        camera
    }

    /// Recomputes the camera's orthonormal basis vectors from eye, center, and up
    pub fn update_basis_vectors(&mut self) {
        // Step 1: Calculate forward direction (from eye toward center)
        // This gives us the primary viewing direction
        self.forward = (self.center - self.eye).normalized();
        
        // Step 2: Calculate right direction using cross product
        // forward × up gives us a vector perpendicular to both (pointing right)
        // This assumes up is roughly correct but may not be perfectly orthogonal
        self.right = self.forward.cross(self.up).normalized();
        
        // Step 3: Recalculate up to ensure perfect orthogonality
        // right × forward gives us a vector perpendicular to both
        // This creates a true orthonormal basis (all vectors perpendicular, unit length)
        self.up = self.right.cross(self.forward);
        
        // Note: We now have an orthonormal coordinate system:
        // - forward: direction camera looks
        // - right: rightward direction from camera's perspective  
        // - up: upward direction from camera's perspective
    }

    /// Rotates the camera around the center point (orbital camera movement)
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        // Step 1: Get camera position relative to the center point
        // This treats center as origin for rotation calculations
        let relative_pos = self.eye - self.center;
        
        // Step 2: Convert to spherical coordinates for easier rotation
        // Calculate current distance from center (radius in spherical coordinates)
        let radius = relative_pos.length();
        
        // Calculate current angles
        // Current yaw: angle around Y axis (horizontal rotation)
        let current_yaw = relative_pos.z.atan2(relative_pos.x);
        // Current pitch: angle from horizontal plane (vertical rotation)
        let current_pitch = (relative_pos.y / radius).asin();
        
        // Step 3: Apply rotation deltas
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5); // Clamp to avoid gimbal lock
        
        // Step 4: Convert back to Cartesian coordinates using trigonometry
        // Spherical to Cartesian: x = r*cos(pitch)*cos(yaw), y = r*sin(pitch), z = r*cos(pitch)*sin(yaw)
        let cos_pitch = new_pitch.cos();
        let new_relative_pos = Vector3::new(
            radius * cos_pitch * new_yaw.cos(),  // X component
            radius * new_pitch.sin(),            // Y component  
            radius * cos_pitch * new_yaw.sin(),  // Z component
        );
        
        // Step 5: Convert back to world coordinates by adding center back
        self.eye = self.center + new_relative_pos;
        
        // Step 6: Recompute basis vectors for new camera orientation
        self.update_basis_vectors();
    }

    /// Transforms a vector from camera space to world space using basis vectors
    pub fn basis_change(&self, v: &Vector3) -> Vector3 {
        // This performs a change of basis transformation
        // Input: vector in camera coordinate system
        // Output: same vector expressed in world coordinate system
        
        // In camera space coordinates:
        // - X axis points right
        // - Y axis points up  
        // - Z axis points backward (away from what camera sees)
        //   Note: This follows right-handed convention where camera looks down -Z
        
        // The math: to convert from camera space to world space,
        // we need to express the camera space vector as a linear combination
        // of world space basis vectors
        
        // World X component = v.x * right.x + v.y * up.x - v.z * forward.x
        // (negative v.z because camera Z points backward, but forward points forward)
        Vector3::new(
            v.x * self.right.x + v.y * self.up.x - v.z * self.forward.x,
            v.x * self.right.y + v.y * self.up.y - v.z * self.forward.y,
            v.x * self.right.z + v.y * self.up.z - v.z * self.forward.z,
        )
        
        // Example: if v = (1,0,0) in camera space (pointing right),
        // result will be self.right in world space
        // if v = (0,0,1) in camera space (pointing backward),
        // result will be -self.forward in world space
    }
}