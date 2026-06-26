use super::{Point2D, Point3D};
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
    pub w: u32,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        let w = 0;
        Self { x, y, w }
    }

    pub fn dot(&self, v: Vector2D) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn add(&self, v: Vector2D) -> Vector2D {
        *self + v
    }

    pub fn mul(&self, s: f64) -> Vector2D {
        *self * s
    }

    pub fn mul_vec(&self, v: Vector2D) -> Vector2D {
        Vector2D::new(self.x * v.x, self.y * v.y)
    }

    pub fn norm(&self) -> f64 {
        (self.dot(*self)).sqrt()
    }

    pub fn normalize(&self) -> Vector2D {
        let n = self.norm();
        Vector2D::new(self.x / n, self.y / n)
    }

    pub fn sub_v(&self, v: Vector2D) -> Vector2D {
        *self - v
    }

    pub fn sub_p(&self, p: Point2D) -> Vector2D {
        Vector2D::new(self.x - p.x, self.y - p.y)
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Self::Output {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Self::Output {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector2D::new(self.x * scalar, self.y * scalar)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: u32,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let w = 0;
        Self { x, y, z, w }
    }

    pub fn from_array(array: [f64; 3]) -> Vector3D {
        Vector3D::new(array[0], array[1], array[2])
    }

    pub fn dot(&self, v: Vector3D) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn add(&self, v: Vector3D) -> Vector3D {
        *self + v
    }

    pub fn mul(&self, s: f64) -> Vector3D {
        *self * s
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

    pub fn norm(&self) -> f64 {
        (self.dot(*self)).sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let n = self.norm();
        Vector3D::new(self.x / n, self.y / n, self.z / n)
    }

    pub fn sub_v(&self, v: Vector3D) -> Vector3D {
        *self - v
    }

    pub fn sub_p(&self, p: Point3D) -> Vector3D {
        Vector3D::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vector3D {
        Vector3D {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
            w: self.w,
        }
    }

    pub fn negate(&self) -> Vector3D {
        -*self
    }

    pub fn length(&self) -> f64 {
        self.norm()
    }
}

impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Self::Output {
        Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Self::Output {
        Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector3D::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}
