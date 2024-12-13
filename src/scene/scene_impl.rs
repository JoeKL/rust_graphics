use crate::types::camera::Camera;
use crate::types::color::ColorRGB;
use crate::types::light::PointLight;
use crate::types::math::{Point3D, Vector3D};
use crate::types::geometry::Mesh;

pub struct Scene {
    #[allow(dead_code)]
    pub root_node: Point3D,
    pub camera: Camera,
    pub lights: Vec<PointLight>,
    pub mesh_list: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Scene {
        let root_node = Point3D::new(0.0, 0.0, 0.0);

        // camera
        let pos = Point3D::new(0.0, 0.0, 10.0);
        let target = Point3D::new(0.0, 0.0, 0.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);

        let mut camera: Camera = Camera::new(pos, target, up);

        camera.set_position(Point3D::new(0.0, 0.0, -10.0));
        camera.look_at(Point3D::new(0.0, 0.0, 0.0));

        //light sources
        let light = PointLight::new(Point3D::new(0.0, 5.0, -5.0), ColorRGB::WHITE, 1.0);
        let light2 = PointLight::new(Point3D::new(-10.0, 10.0, 0.0), ColorRGB::WHITE, 0.5);

        let lights: Vec<PointLight> = vec![light, light2];

        let mesh_list: Vec<Mesh> = vec![Mesh::new_ball()];

        Scene {
            root_node,
            camera,
            lights,
            mesh_list,
        }
    }
}
