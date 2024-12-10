#![allow(dead_code)]

use crate::camera::Camera;
use crate::color::ColorRGB;
use crate::light_source::LightSource;
use crate::mesh::Mesh;
use crate::primitives::*;

pub struct Scene {
    pub root_node: Point,
    pub camera: Camera,
    pub lights: Vec<LightSource>,
    pub mesh_list: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Scene {
        let root_node = Point::new(0.0, 0.0, 0.0);

        // camera
        let pos = Point::new(0.0, 0.0, 10.0);
        let target = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let mut camera: Camera = Camera::new(pos, target, up);

        camera.set_position(Point::new(0.0, 0.0, -10.0));
        camera.look_at(Point::new(0.0, 0.0, 0.0));

        //light sources
        let light = LightSource::new(Point::new(3.0, 3.0, 7.0), ColorRGB::WHITE);

        let mut lights: Vec<LightSource> = Vec::new();
        lights.push(light);

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
