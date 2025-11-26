use crate::types::math::{Mat4x4, Point3D, Vector3D};

#[repr(C)] // Important: ensures consistent memory layout
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    // uv: [f32; 2], todo
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            normal,
            color,
        }
    }

    pub fn position_to_point(self) -> Point3D {
        Point3D::new(self.position[0], self.position[1], self.position[2])
    }

    pub fn normal_to_vector(self) -> Vector3D {
        Vector3D::new(self.normal[0], self.normal[1], self.normal[2])
    }

    pub fn normal_to_point(self) -> Point3D {
        Point3D::new(self.normal[0], self.normal[1], self.normal[2])
    }

    pub fn has_normal(self) -> bool {
        self.normal[0].is_normal() || self.normal[1].is_normal() || self.normal[2].is_normal()
    }

    pub fn transform(&mut self, transform_mat: Mat4x4) {
        let transformed_position = transform_mat.mul_point(Point3D::new(
            self.position[0],
            self.position[1],
            self.position[2],
        ));
        let transformed_normal = transform_mat
            .mul_vec(Vector3D::new(
                self.normal[0],
                self.normal[1],
                self.normal[2],
            ))
            .normalize();

        self.position = [
            transformed_position.x,
            transformed_position.y,
            transformed_position.z,
        ];
        self.normal = [
            transformed_normal.x,
            transformed_normal.y,
            transformed_normal.z,
        ];
    }
}
