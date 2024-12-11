#![allow(dead_code)]

use crate::camera::Camera;
use crate::color::ColorRGB;
use crate::light_source::LightSource;
use crate::mesh::Mesh;
use crate::primitives::*;

pub struct Scene {
    pub root_node: Point3D,
    pub camera: Camera,
    pub lights: Vec<LightSource>,
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
        let light = LightSource::new(Point3D::new(0.0, 0.0, 10.0), ColorRGB::WHITE);
        // let light2 = LightSource::new(Point3D::new(-10.0, 10.0, 0.0), ColorRGB::WHITE);

        let mut lights: Vec<LightSource> = Vec::new();
        lights.push(light);
        // lights.push(light2);

        let mut mesh_list: Vec<Mesh> = Vec::new();

        let mesh_ball  = Mesh::new_ball();
        mesh_list.push(mesh_ball);

        Scene {
            root_node,
            camera,
            lights,
            mesh_list,
        }
    }
}
