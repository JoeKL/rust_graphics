use crate::types::math::{Mat4x4, Point3D, Vector3D};

pub struct Camera {
    // positional parameters
    pub position: Point3D,
    pub direction: Vector3D,
    pub up: Vector3D,
    pub right: Vector3D,

    // projection parameters
    pub fov_in_degrees: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,

    // Cache matrices to avoid recomputing when nothing changes
    look_at_matrix: Mat4x4,
    projection_matrix: Mat4x4,
    frustum_matrix: Mat4x4,

    // flags
    needs_update: bool,
}

impl Camera {
    pub fn new(position: Point3D, target: Point3D, up: Vector3D) -> Camera {
        let direction = target.sub_p(position).normalize();
        let right = direction.cross(up).normalize();
        let up = right.cross(direction).normalize();

        let mut camera = Self {
            position,
            direction,
            up,
            right,
            fov_in_degrees: 60.0,
            aspect_ratio: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
            look_at_matrix: Mat4x4::identity(),
            projection_matrix: Mat4x4::identity(),
            frustum_matrix: Mat4x4::identity(),
            needs_update: true,
        };

        camera.update_matrices();
        camera
    }

    pub fn set_position(&mut self, position: Point3D) {
        self.position = position;
        self.needs_update = true;
    }

    pub fn get_position(&self) -> Point3D {
        self.position
    }

    pub fn look_at(&mut self, target: Point3D) {
        self.direction = target.sub_p(self.position).normalize();
        let world_up = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 1,
        }; // Or Vector3D
        self.right = self.direction.cross(world_up).normalize();
        self.up = self.right.cross(self.direction).normalize();
        self.needs_update = true;
    }

    #[allow(dead_code)]
    pub fn get_pitch_radiants(&self) -> f32 {
        self.direction.y.asin()
    }

    #[allow(dead_code)]
    pub fn get_yaw_radiants(&self) -> f32 {
        self.direction.x.atan2(self.direction.z)
    }

    pub fn set_projection_params(
        &mut self,
        fov_in_degrees: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) {
        self.fov_in_degrees = fov_in_degrees;
        self.aspect_ratio = aspect_ratio;
        self.near = near;
        self.far = far;
        self.needs_update = true;
    }

    fn update_matrices(&mut self) {
        if !self.needs_update {
            return;
        }

        // Generate view matrix
        self.look_at_matrix = Mat4x4 {
            mat: [
                [
                    self.right.x,
                    self.right.y,
                    self.right.z,
                    -self.right.dot(self.position.to_vector()),
                ],
                [
                    self.up.x,
                    self.up.y,
                    self.up.z,
                    -self.up.dot(self.position.to_vector()),
                ],
                [
                    -self.direction.x,
                    -self.direction.y,
                    -self.direction.z,
                    self.direction.dot(self.position.to_vector()),
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let fov_in_radians = self.fov_in_degrees.to_radians();
        let f = 1.0 / (fov_in_radians / 2.0).tan();
        self.projection_matrix = Mat4x4 {
            mat: [
                [f / self.aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [
                    0.0,
                    0.0,
                    (self.far + self.near) / (self.near - self.far),
                    (2.0 * self.far * self.near) / (self.near - self.far),
                ],
                [0.0, 0.0, -1.0, 0.0],
            ],
        };

        // Combine view and projection matrices
        self.frustum_matrix = self.projection_matrix.mul_mat(self.look_at_matrix);
        self.needs_update = false;
    }

    pub fn set_fov_in_degrees(&mut self, fov_in_degrees: f32) {
        self.fov_in_degrees = fov_in_degrees;
        self.needs_update = true;
    }

    pub fn get_fov_in_degrees(&self) -> f32 {
        self.fov_in_degrees
    }

    #[allow(dead_code)]
    pub fn get_look_at_matrix(&mut self) -> Mat4x4 {
        self.update_matrices();
        self.look_at_matrix
    }

    #[allow(dead_code)]
    pub fn get_projection_matrix(&mut self) -> Mat4x4 {
        self.update_matrices();
        self.projection_matrix
    }

    pub fn get_frustum_matrix(&mut self) -> Mat4x4 {
        self.update_matrices();
        self.frustum_matrix
    }
}
