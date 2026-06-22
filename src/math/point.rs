use super::{Vector2D, Vector3D};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
    pub w: u32,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        let w = 1;
        Self { x, y, w }
    }

    pub fn sub_v(&self, v: Vector2D) -> Point2D {
        *self - v
    }

    pub fn sub_p(&self, p: Point2D) -> Vector2D {
        *self - p
    }

    pub fn add_v(&self, v: Vector2D) -> Point2D {
        *self + v
    }

    pub fn add_p(&self, p: Point2D) -> Point2D {
        Point2D::new(self.x + p.x, self.y + p.y)
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

impl Add<Vector2D> for Point2D {
    type Output = Point2D;

    fn add(self, other: Vector2D) -> Self::Output {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vector2D> for Point2D {
    type Output = Point2D;

    fn sub(self, other: Vector2D) -> Self::Output {
        Point2D::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Vector2D;

    fn sub(self, other: Point2D) -> Self::Output {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let w = 1.0;
        Self { x, y, z, w }
    }

    pub fn from_array(array: [f64; 3]) -> Point3D {
        Point3D::new(array[0], array[1], array[2])
    }

    pub fn dehomogen(&mut self) {
        if self.w == 0.0 {
            println!("trying to divide by zero");
            return;
        }
        self.x /= self.w;
        self.y /= self.w;
        self.z /= self.w;
        self.w = 1.0;
    }

    pub fn sub_v(&self, v: Vector3D) -> Point3D {
        *self - v
    }

    pub fn add_v(&self, v: Vector3D) -> Point3D {
        *self + v
    }

    pub fn sub_p(&self, p: Point3D) -> Vector3D {
        *self - p
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

impl Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, other: Vector3D) -> Self::Output {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vector3D> for Point3D {
    type Output = Point3D;

    fn sub(self, other: Vector3D) -> Self::Output {
        Point3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Vector3D;

    fn sub(self, other: Point3D) -> Self::Output {
        Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

impl ScreenPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
