use crate::scene::Light;
use crate::math::{Point3D, Vector3D};
use std::sync::atomic::{AtomicUsize, Ordering};

static MATERIAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
pub struct Material {
    pub id: usize,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Material {
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

pub trait ShadingModel {
    fn calc_color(
        &self,
        surface_point: &Point3D,
        surface_normal: &Vector3D,
        surface_color: &[f32; 3],
        view_vector: &Vector3D,
        material: &Material,
        lights: &[Light],
    ) -> [f32; 3];
}

// Flat shading implementation
pub struct FlatShader;

impl ShadingModel for FlatShader {
    fn calc_color(
        &self,
        surface_point: &Point3D,
        surface_normal: &Vector3D,
        surface_color: &[f32; 3],
        view_vector: &Vector3D,
        material: &Material,
        lights: &[Light],
    ) -> [f32; 3] {
        let material_color = Vector3D::new(surface_color[0], surface_color[1], surface_color[2]);
        let mut final_color = Vector3D::new(0.0, 0.0, 0.0);
        let light_count = lights.len() as f32;

        // calculate Ambient component Ca
        let ca_ambient = material_color.mul(material.ambient);

        // add Ca
        final_color = final_color.add(ca_ambient);

        for light in lights {
            let light_dir = light.get_direction(surface_point);
            let halfway = view_vector.add(light_dir).normalize();

            // Diffuse component
            let cd_diffuse = material_color
                .mul(material.diffuse)
                .mul(f32::max(light_dir.dot(*surface_normal), 0.0));

            // Specular component
            let cs_specular = Vector3D::new(1.0, 1.0, 1.0)
                .mul(material.specular)
                .mul(f32::max(halfway.dot(*surface_normal), 0.0).powf(material.shininess));

            let light_color = light.get_color().to_vector();

            // Spotlight factor calculation
            let spotlight_factor = match light {
                Light::Point(_) => 1.0,
                Light::Spot(spot) => {
                    // Vector pointing from light position to surface point (both in view space)
                    let light_to_surface = (*surface_point - spot.get_position()).normalize();
                    // Direction vector of the spotlight is spot.direction
                    let cos_theta = light_to_surface.dot(spot.direction.normalize());
                    let cos_cutoff = spot.cutoff.to_radians().cos();
                    if cos_theta >= cos_cutoff {
                        // Apply a smooth falloff towards the edges of the spotlight cone
                        let delta = 1.0 - cos_cutoff;
                        if delta > 0.0 {
                            let factor = (cos_theta - cos_cutoff) / delta;
                            factor.powf(2.0)
                        } else {
                            1.0
                        }
                    } else {
                        0.0
                    }
                }
            };

            let light_contribution = cd_diffuse
                .add(cs_specular)
                .mul_vec(light_color)
                .mul(light.get_intensity() * spotlight_factor / light_count);

            final_color = final_color.add(light_contribution);
        }
        // Clamp and convert back to RGB
        final_color = final_color.clamp(0.0, 1.0);
        [final_color.x, final_color.y, final_color.z]
    }
}
