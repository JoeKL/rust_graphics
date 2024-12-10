use crate::primitives::*;
use crate::obj_loader::*;

pub struct Mesh {
    pub vertices: Vec<Point>,
    pub faces: Vec<Face>,
}
impl Mesh {    
    pub fn new(vertices: Vec<Point>, faces: Vec<Face>) -> Mesh {
    Mesh {
        vertices,
        faces,
    }
}
    pub fn new_ball() -> Mesh {
        let vertices = create_vertices();
        let faces = create_faces(&vertices);
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
