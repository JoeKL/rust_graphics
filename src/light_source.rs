#![allow(dead_code)]

use crate::primitives::*;
use crate::color::ColorRGB;

#[derive(Debug, Clone, Copy)]
pub struct LightSource{
    position: Point3D,
    color: ColorRGB
}

impl LightSource {
    pub fn new(position: Point3D, color: ColorRGB)-> LightSource{
        LightSource{position, color}
    }

    pub fn set_position(&mut self, position: Point3D){
        self.position = position
    }

    pub fn get_position(&self)-> Point3D{
        self.position
    }

    pub fn set_color(&mut self, color: ColorRGB){
        self.color = color
    }

    pub fn get_color(&self) -> ColorRGB{
        self.color
    }

    pub fn get_color_as_vector(&self) -> Vector3D{
        Vector3D::new(
            self.get_color().get_r() as f32 / 255.0, // Convert 0-255 to 0-1 range
            self.get_color().get_g() as f32 / 255.0,
            self.get_color().get_b() as f32 / 255.0,
        )
    }
}