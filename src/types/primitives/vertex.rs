use crate::types::{
    color::ColorRGB,
    math::{Point3D, Vector3D},
};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Point3D,
    pub normal: Vector3D,
    pub color: ColorRGB,
}

impl Vertex{
    pub fn new(position: Point3D, normal: Vector3D, color: ColorRGB) -> Self{
        Self { position, normal, color }
    }

}