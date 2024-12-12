use crate::types::{color::ColorRGB, math::Vector3D};

pub struct Material {
    pub color: ColorRGB,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(
        color: ColorRGB,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Material {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}
