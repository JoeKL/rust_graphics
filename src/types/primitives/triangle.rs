use crate::types::math::{Point3D, Vector3D};

use super::vertex::Vertex;

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

    pub fn calc_triangle_area(self) -> f32 {
        let signed_area = (self.b.x - self.a.x) * (self.c.y - self.a.y)
            - (self.b.y - self.a.y) * (self.c.x - self.a.x);
        signed_area / 2.0
    }

    pub fn calc_normal(&self) -> Vector3D {
        let edge_1 = self.b.sub_p(self.a);
        let edge_2 = self.c.sub_p(self.a);
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
