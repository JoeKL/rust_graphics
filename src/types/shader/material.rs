use super::MATERIAL_ID_COUNTER;
use crate::types::color::ColorRGB;
use std::sync::atomic::Ordering;

pub struct Material {
    pub id: usize,
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
        let id = MATERIAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub const MATERIAL_0: Material = Material {
        id: 0, // Fixed ID
        color: ColorRGB {
            as_u32: 0x00FFCC, //mint
            r: 0,
            g: 255,
            b: 204,
        },
        ambient: 0.1,
        diffuse: 0.5,
        specular: 0.5,
        shininess: 50.0,
    };
    pub const MATERIAL_1: Material = Material {
        id: 1,
        color: ColorRGB {
            as_u32: 0xFF6B00, // orange
            r: 200,
            g: 107,
            b: 0,
        },
        ambient: 0.2,
        diffuse: 0.7,
        specular: 0.4,
        shininess: 20.0,
    };

    pub const MATERIAL_2: Material = Material {
        id: 2,
        color: ColorRGB { 
            as_u32: 0xFF0099,
            r: 200,
            g: 0,
            b: 153
        },
        ambient: 0.15,
        diffuse: 0.6,
        specular: 0.1,
        shininess: 5.0,
    };
}
