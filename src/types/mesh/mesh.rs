use crate::types::math::{Point3D, Mat4x4};
use crate::types::mesh::Face;
use crate::types::primitives::Triangle;


use crate::utils::*;

pub struct Mesh {
    pub vertices: Vec<Point3D>,
    pub faces: Vec<Face>,
}
impl Mesh {    
    pub fn new(vertices: Vec<Point3D>, faces: Vec<Face>) -> Mesh {
    Mesh {
        vertices,
        faces,
    }
}
    pub fn new_ball() -> Mesh {
        let vertices = create_vertices();
        let faces = create_faces();
        Mesh {
            vertices,
            faces,
        }
    }

    pub fn transform_mesh(&mut self, transform: Mat4x4) {
        for vertex in &mut self.vertices {
            *vertex = transform.mul_point(*vertex);
        }
    }

    // Helper to get triangles for rendering
    pub fn get_triangles(&self) -> Vec<Triangle> {
        self.faces.iter()
            .map(|face| face.to_triangle(&self.vertices))
            .collect()
    }
}
