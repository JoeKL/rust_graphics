
use crate::types::math::{Point2D,Point3D};


#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
    pub w: u32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        let w = 0;
        Self { x, y, w }
    }

    pub fn dot(&self, v: Vector2D) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn add(&self, v: Vector2D) -> Vector2D {
        Vector2D::new(self.x + v.x, self.y + v.y)
    }

    pub fn mul(&self, s: f32) -> Vector2D {
        Vector2D::new(self.x * s, self.y * s)
    }

    pub fn mul_vec(&self, v: Vector2D) -> Vector2D {
        Vector2D::new(self.x * v.x, self.y * v.y)
    }

    pub fn norm(&self) -> f32 {
        (self.dot(*self)).sqrt()
    }

    pub fn normalize(&self) -> Vector2D {
        let n = self.norm();
        Vector2D::new(self.x / n, self.y / n)
    }

    pub fn sub_v(&self, v: Vector2D) -> Vector2D {
        Vector2D::new(self.x - v.x, self.y - v.y)
    }

    pub fn sub_p(&self, p: Point2D) -> Vector2D {
        Vector2D::new(self.x - p.x, self.y - p.y)
    }

}


#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: u32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let w = 0;
        Self { x, y, z, w }
    }

    pub fn dot(&self, v: Vector3D) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn add(&self, v: Vector3D) -> Vector3D {
        Vector3D::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    pub fn mul(&self, s: f32) -> Vector3D {
        Vector3D::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn mul_vec(&self, v: Vector3D) -> Vector3D {
        Vector3D::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

    pub fn cross(&self, v: Vector3D) -> Vector3D {
        let x = self.y * v.z - self.z * v.y;
        let y = self.z * v.x - self.x * v.z;
        let z = self.x * v.y - self.y * v.x;
        Vector3D::new(x, y, z)
    }

    pub fn norm(&self) -> f32 {
        (self.dot(*self)).sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let n = self.norm();
        Vector3D::new(self.x / n, self.y / n, self.z / n)
    }

    pub fn sub_v(&self, v: Vector3D) -> Vector3D {
        Vector3D::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    pub fn sub_p(&self, p: Point3D) -> Vector3D {
        Vector3D::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }

    pub fn clamp(&self, min: f32, max: f32) -> Vector3D {
        Vector3D {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
            w: self.w
        }
    }

    pub(crate) fn negate(&self) -> Vector3D {
        Vector3D {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
            w: self.w
        }
    }
}
