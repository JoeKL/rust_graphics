use crate::types::{
    light::PointLight,
    math::{Point3D, Vector3D},
    shader::Material,
};

pub trait ShadingModel {
    fn calc_color(
        &self,
        surface_point: &Point3D,
        surface_normal: &Vector3D,
        surface_color: &[f32; 3],
        view_vector: &Vector3D,
        material: &Material,
        lights: &[PointLight],
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
        lights: &[PointLight],
    ) -> [f32; 3] {
        let material_color =Vector3D::new(surface_color[0], surface_color[1], surface_color[2]);
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
                    let light_contribution = cd_diffuse
                        .add(cs_specular)
                        .mul_vec(light_color)
                        .mul(light.get_intensity() / light_count);
                    
                    final_color = final_color.add(light_contribution);
        }        
        // Clamp and convert back to RGB
        final_color = final_color.clamp(0.0, 1.0);
        [final_color.x, final_color.y, final_color.z]
    }
}
