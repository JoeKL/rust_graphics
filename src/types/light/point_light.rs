use crate::types::math::{Point3D, Vector3D};
use crate::types::color::ColorRGB;

#[derive(Debug, Clone, Copy)]
pub struct PointLight{
    position: Point3D,
    color: ColorRGB,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Point3D, color: ColorRGB, intensity: f32)-> PointLight{
        PointLight{position, color, intensity}
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

    pub fn set_intensity(&mut self, intensity: f32){
        self.intensity = intensity
    }

    pub fn get_intensity(&self) -> f32{
        self.intensity
    }

    pub fn get_direction(&self, surface_point: &Point3D) -> Vector3D {
        self.position.sub_p(*surface_point).normalize()
    }

    pub fn get_color_as_vector(&self) -> Vector3D{
        Vector3D::new(
            self.get_color().get_r() as f32 / 255.0, // Convert 0-255 to 0-1 range
            self.get_color().get_g() as f32 / 255.0,
            self.get_color().get_b() as f32 / 255.0,
        )
    }
}