#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Point2D{
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}



impl Point2D {
    pub fn new (x:f32, y:f32) -> Self{
        Self{x, y}
    }

}


impl Point3D {
    pub fn new (x:f32, y:f32, z:f32) -> Self{
        Self{x, y, z}
    }

    
}
