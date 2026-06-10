use crate::renderer::color::ColorRGB;
use crate::math::{Mat4x4, Point3D, Vector3D};

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    position: Point3D,
    color: ColorRGB,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Point3D, color: ColorRGB, intensity: f32) -> PointLight {
        PointLight {
            position,
            color,
            intensity,
        }
    }

    pub fn set_position(&mut self, position: Point3D) {
        self.position = position
    }

    pub fn get_position(&self) -> Point3D {
        self.position
    }

    pub fn set_color(&mut self, color: ColorRGB) {
        self.color = color
    }

    pub fn get_color(&self) -> ColorRGB {
        self.color
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity
    }

    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    pub fn get_direction(&self, surface_point: &Point3D) -> Vector3D {
        (self.position - *surface_point).normalize()
    }

    pub fn get_color_as_vector(&self) -> Vector3D {
        Vector3D::new(
            self.get_color().get_r() as f32 / 255.0, // Convert 0-255 to 0-1 range
            self.get_color().get_g() as f32 / 255.0,
            self.get_color().get_b() as f32 / 255.0,
        )
    }

    pub fn transform_light(&mut self, projection_mat: Mat4x4) {
        self.set_position(projection_mat * self.get_position())
    }

    pub fn new_transformed_light(light: &PointLight, loot_at_mat: Mat4x4) -> PointLight {
        let new_light_pos = loot_at_mat * light.get_position();
        PointLight::new(new_light_pos, light.get_color(), light.get_intensity())
    }

    pub fn to_world(&self, world_transform: &Mat4x4) -> PointLight {
        let world_pos = world_transform.mul_point(self.position);
        PointLight {
            position: world_pos,
            color: self.color,
            intensity: self.intensity,
        }
    }
}
