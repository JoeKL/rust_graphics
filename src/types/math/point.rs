
use crate::types::math::{Vector2D,Vector3D};

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
    pub w: u32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        let w = 1;
        Self { x, y, w }
    }

    pub fn sub_v(&self, v: Vector2D) -> Vector2D {
        Vector2D::new(self.x - v.x, self.y - v.y)
    }

    pub fn sub_p(&self, p: Point2D) -> Vector2D {
        Vector2D::new(self.x - p.x, self.y - p.y)
    }

    pub fn add_v(&self, v: Vector2D) -> Vector2D {
        Vector2D::new(self.x + v.x, self.y + v.y)
    }

    pub fn add_p(&self, p: Point2D) -> Vector2D {
        Vector2D::new(self.x + p.x, self.y + p.y)
    }


    pub fn to_vector(self) -> Vector2D {
        let w = 0;
        Vector2D {
            x: self.x,
            y: self.y,
            w,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: u32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let w = 1;
        Self { x, y, z, w }
    }

    pub fn dehomogen(&mut self) {
        if self.z == 0.0 {
            println!("trying to divide by zero")
        }
        self.x /= -self.z;
        self.y /= -self.z;
        self.z = -1.0;
        self.w = 1;
    }

    pub fn sub_v(&self, v: Vector3D) -> Vector3D {
        Vector3D::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    pub fn sub_p(&self, p: Point3D) -> Vector3D {
        Vector3D::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }

    pub fn to_vector(self) -> Vector3D {
        let w = 0;
        Vector3D {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }
}
