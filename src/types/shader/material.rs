use super::MATERIAL_ID_COUNTER;
use std::sync::atomic::Ordering;

#[derive(Clone, Copy)]
pub struct Material {
    pub id: usize,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Material {
        let id = MATERIAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub const MATERIAL_ARRAY: [Material; 3] = [
        Material {
            id: 0,
            ambient: 0.1,
            diffuse: 0.5,
            specular: 1.0,
            shininess: 50.0,
        },
        Material {
            id: 1,
            ambient: 0.2,
            diffuse: 0.7,
            specular: 0.4,
            shininess: 20.0,
        },
        Material {
            id: 2,
            ambient: 0.15,
            diffuse: 0.7,
            specular: 0.1,
            shininess: 5.0,
        },
    ];
}
