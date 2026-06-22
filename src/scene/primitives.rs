use crate::math::{Mat4x4, Point3D, Vector3D};

#[repr(C)] // Important: ensures consistent memory layout
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f64; 3],
    pub uv: [f64; 2],
    pub normal: [f64; 3],
    pub color: [f64; 3],
}

impl Vertex {
    pub fn new(position: [f64; 3], uv: [f64; 2], normal: [f64; 3], color: [f64; 3]) -> Self {
        Self {
            position,
            uv,
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
        // is_normal returns true if the number is neither zero, infinite, subnormal, or NaN.
        self.normal[0].is_normal() && self.normal[1].is_normal() && self.normal[2].is_normal()
    }

    pub fn transform(&mut self, transform_mat: Mat4x4) {
        let transformed_position =
            transform_mat * Point3D::new(self.position[0], self.position[1], self.position[2]);
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

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,
}

impl Triangle {
    pub fn new(a: Point3D, b: Point3D, c: Point3D) -> Self {
        Self { a, b, c }
    }

    pub fn calc_triangle_area(self) -> f64 {
        let signed_area = (self.b.x - self.a.x) * (self.c.y - self.a.y)
            - (self.b.y - self.a.y) * (self.c.x - self.a.x);
        signed_area / 2.0
    }

    pub fn calc_normal(&self) -> Vector3D {
        let edge_1 = self.b - self.a;
        let edge_2 = self.c - self.a;
        edge_1.cross(edge_2).normalize()
    }

    pub fn calc_center(&self) -> Point3D {
        Point3D::new(
            (self.a.x + self.b.x + self.c.x) / 3.0,
            (self.a.y + self.b.y + self.c.y) / 3.0,
            (self.a.z + self.b.z + self.c.z) / 3.0,
        )
    }
}
