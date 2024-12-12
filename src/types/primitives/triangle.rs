use crate::types::math::{Point3D, Vector3D};

use super::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Self {
        Self { a, b, c }
    }

    pub fn calc_triangle_area(self) -> f32 {
        let signed_area = (self.b.position.x - self.a.position.x) * (self.c.position.y - self.a.position.y)
            - (self.b.position.y - self.a.position.y) * (self.c.position.x - self.a.position.x);
        signed_area as f32 / 2.0
    }

    pub fn calc_normal(&self) -> Vector3D {
        let edge_1 = self.b.position.sub_p(self.a.position);
        let edge_2 = self.c.position.sub_p(self.a.position);
        edge_1.cross(edge_2).normalize()
    }

    pub fn calc_center(&self) -> Point3D {
        Point3D::new(
            (self.a.position.x + self.b.position.x + self.c.position.x) / 3.0,
            (self.a.position.y + self.b.position.y + self.c.position.y) / 3.0,
            (self.a.position.z + self.b.position.z + self.c.position.z) / 3.0,
        )
    }
}
