use crate::math::{Mat4x4, Point3D, Vector3D};

pub struct Camera {
    // positional parameters
    pub position: Point3D,
    pub direction: Vector3D,
    pub up: Vector3D,
    pub right: Vector3D,

    // projection parameters
    pub fov_in_degrees: f64,
    pub aspect_ratio: f64,
    pub near: f64,
    pub far: f64,

    // Cache matrices to avoid recomputing when nothing changes
    look_at_matrix: Mat4x4,
    projection_matrix: Mat4x4,
    frustum_matrix: Mat4x4,
}

impl Camera {
    pub fn new(position: Point3D, target: Point3D, up: Vector3D) -> Camera {
        let direction = (target - position).normalize();
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
        };

        camera.update_matrices();
        camera
    }

    pub fn set_position(&mut self, position: Point3D) {
        self.position = position;
        self.update_matrices();
    }

    pub fn get_position(&self) -> Point3D {
        self.position
    }

    pub fn look_at(&mut self, target: Point3D) {
        self.direction = (target - self.position).normalize();
        let world_up = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 1,
        };
        self.right = self.direction.cross(world_up).normalize();
        self.up = self.right.cross(self.direction).normalize();
        self.update_matrices();
    }

    #[allow(dead_code)]
    pub fn get_pitch_radiants(&self) -> f64 {
        self.direction.y.asin()
    }

    #[allow(dead_code)]
    pub fn get_yaw_radiants(&self) -> f64 {
        self.direction.x.atan2(self.direction.z)
    }

    pub fn set_projection_params(
        &mut self,
        fov_in_degrees: f64,
        aspect_ratio: f64,
        near: f64,
        far: f64,
    ) {
        self.fov_in_degrees = fov_in_degrees;
        self.aspect_ratio = aspect_ratio;
        self.near = near;
        self.far = far;
        self.update_matrices();
    }

    fn update_matrices(&mut self) {
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
    }

    pub fn set_fov_in_degrees(&mut self, fov_in_degrees: f64) {
        if (0.0..180.0).contains(&fov_in_degrees) {
            self.fov_in_degrees = fov_in_degrees;
            self.update_matrices();
        }
    }

    pub fn get_fov_in_degrees(&self) -> f64 {
        self.fov_in_degrees
    }

    #[allow(dead_code)]
    pub fn get_look_at_matrix(&self) -> Mat4x4 {
        self.look_at_matrix
    }

    #[allow(dead_code)]
    pub fn get_projection_matrix(&self) -> Mat4x4 {
        self.projection_matrix
    }

    pub fn get_frustum_matrix(&self) -> Mat4x4 {
        self.frustum_matrix
    }

    pub fn to_world(&self, world_transform: &Mat4x4) -> Camera {
        let world_pos = world_transform.mul_point(self.position);
        let world_dir = world_transform.mul_vec(self.direction).normalize();
        let world_up = world_transform.mul_vec(self.up).normalize();
        let world_right = world_transform.mul_vec(self.right).normalize();

        let mut world_camera = Camera {
            position: world_pos,
            direction: world_dir,
            up: world_up,
            right: world_right,
            fov_in_degrees: self.fov_in_degrees,
            aspect_ratio: self.aspect_ratio,
            near: self.near,
            far: self.far,
            look_at_matrix: Mat4x4::identity(),
            projection_matrix: Mat4x4::identity(),
            frustum_matrix: Mat4x4::identity(),
        };

        world_camera.update_matrices();
        world_camera
    }
}
