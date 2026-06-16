use crate::math::{Mat4x4, Point3D, Vector3D};
use crate::renderer::color::ColorRGB;

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Point3D,
    pub color: ColorRGB,
    pub intensity: f32,
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
            self.color.get_r() as f32 / 255.0, // Convert 0-255 to 0-1 range
            self.color.get_g() as f32 / 255.0,
            self.color.get_b() as f32 / 255.0,
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

#[derive(Debug, Clone, Copy)]
pub struct Spotlight {
    pub position: Point3D,
    pub color: ColorRGB,
    pub intensity: f32,
    pub direction: Vector3D,
    pub cutoff: f32, // cutoff angle in degrees
}

impl Spotlight {
    pub fn new(position: Point3D, color: ColorRGB, intensity: f32, direction: Vector3D, cutoff: f32) -> Spotlight {
        Spotlight {
            position,
            color,
            intensity,
            direction: direction.normalize(),
            cutoff,
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
            self.color.get_r() as f32 / 255.0,
            self.color.get_g() as f32 / 255.0,
            self.color.get_b() as f32 / 255.0,
        )
    }

    pub fn transform_light(&mut self, projection_mat: Mat4x4) {
        self.set_position(projection_mat * self.get_position());
        self.direction = projection_mat.mul_vec(self.direction).normalize();
    }

    pub fn new_transformed_light(light: &Spotlight, look_at_mat: Mat4x4) -> Spotlight {
        let new_light_pos = look_at_mat * light.get_position();
        let new_light_dir = look_at_mat.mul_vec(light.direction).normalize();
        Spotlight::new(new_light_pos, light.get_color(), light.get_intensity(), new_light_dir, light.cutoff)
    }

    pub fn to_world(&self, world_transform: &Mat4x4) -> Spotlight {
        let world_pos = world_transform.mul_point(self.position);
        let world_dir = world_transform.mul_vec(self.direction).normalize();
        Spotlight {
            position: world_pos,
            color: self.color,
            intensity: self.intensity,
            direction: world_dir,
            cutoff: self.cutoff,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Light {
    Point(PointLight),
    Spot(Spotlight),
}

impl Light {
    pub fn get_position(&self) -> Point3D {
        match self {
            Light::Point(l) => l.get_position(),
            Light::Spot(l) => l.get_position(),
        }
    }

    pub fn set_position(&mut self, position: Point3D) {
        match self {
            Light::Point(l) => l.set_position(position),
            Light::Spot(l) => l.set_position(position),
        }
    }

    pub fn get_color(&self) -> ColorRGB {
        match self {
            Light::Point(l) => l.get_color(),
            Light::Spot(l) => l.get_color(),
        }
    }

    pub fn set_color(&mut self, color: ColorRGB) {
        match self {
            Light::Point(l) => l.set_color(color),
            Light::Spot(l) => l.set_color(color),
        }
    }

    pub fn get_intensity(&self) -> f32 {
        match self {
            Light::Point(l) => l.get_intensity(),
            Light::Spot(l) => l.get_intensity(),
        }
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        match self {
            Light::Point(l) => l.set_intensity(intensity),
            Light::Spot(l) => l.set_intensity(intensity),
        }
    }

    pub fn get_direction(&self, surface_point: &Point3D) -> Vector3D {
        match self {
            Light::Point(l) => l.get_direction(surface_point),
            Light::Spot(l) => l.get_direction(surface_point),
        }
    }

    pub fn get_color_as_vector(&self) -> Vector3D {
        match self {
            Light::Point(l) => l.get_color_as_vector(),
            Light::Spot(l) => l.get_color_as_vector(),
        }
    }

    pub fn transform_light(&mut self, projection_mat: Mat4x4) {
        match self {
            Light::Point(l) => l.transform_light(projection_mat),
            Light::Spot(l) => l.transform_light(projection_mat),
        }
    }

    pub fn new_transformed_light(light: &Light, look_at_mat: Mat4x4) -> Light {
        match light {
            Light::Point(l) => Light::Point(PointLight::new_transformed_light(l, look_at_mat)),
            Light::Spot(l) => Light::Spot(Spotlight::new_transformed_light(l, look_at_mat)),
        }
    }

    pub fn to_world(&self, world_transform: &Mat4x4) -> Light {
        match self {
            Light::Point(l) => Light::Point(l.to_world(world_transform)),
            Light::Spot(l) => Light::Spot(l.to_world(world_transform)),
        }
    }

    pub fn rotate_direction(&mut self, rot_mat: Mat4x4) {
        if let Light::Spot(spot) = self {
            spot.direction = rot_mat.mul_vec(spot.direction).normalize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_light() {
        let pl = PointLight::new(Point3D::new(1.0, 2.0, 3.0), ColorRGB::WHITE, 1.5);
        assert_eq!(pl.get_position(), Point3D::new(1.0, 2.0, 3.0));
        assert_eq!(pl.get_intensity(), 1.5);
    }

    #[test]
    fn test_spotlight() {
        let spot = Spotlight::new(
            Point3D::new(0.0, 5.0, 0.0),
            ColorRGB::WHITE,
            2.0,
            Vector3D::new(0.0, -1.0, 0.0), // pointing straight down
            30.0, // 30 degrees cutoff
        );

        assert_eq!(spot.get_position(), Point3D::new(0.0, 5.0, 0.0));
        assert_eq!(spot.get_intensity(), 2.0);

        // A point straight down (0.0, 0.0, 0.0)
        let pt_inside = Point3D::new(0.0, 0.0, 0.0);
        let dir_to_inside = (pt_inside - spot.get_position()).normalize();
        let cos_inside = dir_to_inside.dot(spot.direction.normalize());
        assert!(cos_inside >= spot.cutoff.to_radians().cos(), "Should be inside the cone");

        // A point way off to the side (5.0, 5.0, 0.0)
        let pt_outside = Point3D::new(5.0, 5.0, 0.0);
        let dir_to_outside = (pt_outside - spot.get_position()).normalize();
        let cos_outside = dir_to_outside.dot(spot.direction.normalize());
        assert!(cos_outside < spot.cutoff.to_radians().cos(), "Should be outside the cone");
    }
}

