use crate::types::math::Point3D;

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
        signed_area as f32 / 2.0
    }
}