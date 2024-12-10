#![warn(dead_code)]

use crate::camera::Camera;
use crate::color::ColorRGB;
use crate::light_source::LightSource;
use crate::obj_loader::*;
use crate::primitives::*;

pub struct Mesh {
    pub vertices: Vec<Point>,
    pub triangles: Vec<Triangle>,
}
impl Mesh {
    pub fn new() -> Mesh {
        let vertices = create_vertices();
        let triangles = create_triangles(&vertices);
        Mesh {
            vertices,
            triangles,
        }
    }

    pub fn transform_mesh(&mut self, transform: Mat4x4){

        for i in 0..self.triangles.len() {
            self.triangles[i].a = transform.mul_point(self.triangles[i].a);
            self.triangles[i].b = transform.mul_point(self.triangles[i].b);
            self.triangles[i].c = transform.mul_point(self.triangles[i].c);
        }
    }
}

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
        let pos = Point::new(0.0, 0.0, 25.0);
        let target = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let camera: Camera = Camera::new(pos, target, up);

        //light sources
        let light = LightSource::new(Point::new(15.0, 15.0, 15.0), ColorRGB::WHITE);

        let mut lights: Vec<LightSource> = Vec::new();
        lights.push(light);

        let mesh = Mesh::new();

        let mut mesh_list: Vec<Mesh> = Vec::new();

        mesh_list.push(mesh);

        Scene {
            root_node,
            camera,
            lights,
            mesh_list,
        }
    }
}
