use crate::types::math::{Point3D, Mat4x4};
use crate::types::geometry::Face;
use crate::types::primitives::{Triangle, Vertex};


use crate::utils::*;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}
impl Mesh {    
    pub fn new(vertices: Vec<Vertex>, faces: Vec<Face>) -> Mesh {
    Mesh {
        vertices,
        faces,
    }
}
    pub fn new_ball() -> Mesh {
        let vertices: Vec<Vertex> = create_vertices();
        let faces = create_faces();
        Mesh {
            vertices,
            faces,
        }
    }

    pub fn transform_mesh(&mut self, transform: Mat4x4) {
        for vertex in &mut self.vertices {
            vertex.position = transform.mul_point(vertex.position);
        }
    }

    pub fn get_triangles(&self) -> Vec<Triangle> {
        self.faces.iter()
            .map(|face| face.to_triangle(&self.vertices))
            .collect()
    }
}
