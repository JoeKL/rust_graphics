
use crate::types::math::{Point3D, Vector3D};


#[derive(Debug, Clone, Copy)]
pub struct Line3D {
    pub start: Point3D,
    pub end: Point3D,
}

impl Line3D {
    pub fn new(start: Point3D, end: Point3D) -> Self {
        Self { start, end }
    }
    pub fn length(&self) -> f32 {
        self.start.to_vector().sub_p(self.end).norm()
    }

    pub fn direction(&self) -> Vector3D {
        self.end.sub_p(self.start).normalize()
    }

    pub fn midpoint(&self) -> Point3D {
        Point3D::new(
            (self.start.x + self.end.x) * 0.5,
            (self.start.y + self.end.y) * 0.5,
            (self.start.z + self.end.z) * 0.5,
        )
    }
}
