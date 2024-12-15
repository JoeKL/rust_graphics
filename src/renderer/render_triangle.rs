use crate::types::{math::{Point3D, Vector3D}, primitives::Vertex};

#[derive(Debug, Clone, Copy)]
pub struct RenderTriangle {
    pub vertices: [Vertex; 3],
    pub normal: [f32; 3],
    pub material_id: u32,
}

impl RenderTriangle {
    pub fn calculate_center(self) -> Point3D {
        Point3D::new(
            (self.vertices[0].position[0]
                + self.vertices[1].position[0]
                + self.vertices[2].position[0])
                / 3.0,
            (self.vertices[0].position[1]
                + self.vertices[1].position[1]
                + self.vertices[2].position[1])
                / 3.0,
            (self.vertices[0].position[2]
                + self.vertices[1].position[2]
                + self.vertices[2].position[2])
                / 3.0,
        )
    }

    pub fn normal_to_vector(self) -> Vector3D{
        Vector3D::new(self.normal[0], self.normal[1], self.normal[2])
    }
}
