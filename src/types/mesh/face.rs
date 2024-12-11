use crate::types::math::Point3D;
use crate::types::primitives::Triangle;

#[derive(Debug, Clone, Copy)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}


impl Face {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }

    // Helper method to get the actual triangle from vertices
    pub fn to_triangle(&self, vertices: &[Point3D]) -> Triangle {
        Triangle::new(
            vertices[self.a],
            vertices[self.b],
            vertices[self.c],
        )
    }

    pub fn calc_triangle_area(&self, vertices: &[Point3D]) -> f32 {
        let point_a = vertices[self.a];
        let point_b = vertices[self.b];
        let point_c = vertices[self.c];
        
        let signed_area = (point_b.x - point_a.x) * (point_c.y - point_a.y)
            - (point_b.y - point_a.y) * (point_c.x - point_a.x);
        signed_area as f32 / 2.0
    }
}

