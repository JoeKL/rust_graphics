use crate::types::{
    math::{Point3D, Vector3D},
    primitives::Vertex,
};

#[derive(Debug, Clone, Copy)]
pub struct RenderTriangle {
    pub vertices: [Vertex; 3],
    pub normal: [[f32; 3]; 3],
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

    pub fn normal_to_vector(self) -> [Vector3D; 3] {
        [
            Vector3D::new(self.normal[0][0], self.normal[0][1], self.normal[0][2]),
            Vector3D::new(self.normal[1][1], self.normal[1][1], self.normal[1][2]),
            Vector3D::new(self.normal[2][2], self.normal[2][1], self.normal[2][2]),
        ]
    }

    pub fn normal_to_face_vector(self) -> Vector3D {
        Vector3D::new(
            (self.normal[0][0] + self.normal[1][0] + self.normal[2][0]) / 3.0,
            (self.normal[0][1] + self.normal[1][1] + self.normal[2][1]) / 3.0,
            (self.normal[0][2] + self.normal[1][2] + self.normal[2][2]) / 3.0
        )
    }

    pub fn is_front_facing(render_triangle: &RenderTriangle, camera_direction: &Vector3D) -> bool {
        // triangle_normal · (0,0,1) = normal.z

        // If normal.z > 0: cull (facing away)
        // If normal.z < 0: keep (facing camera)

        //epsilon 0.5 = 45°
        //epsilon 0.25 = 22.5°

        let epsilon = 0.1;

        if render_triangle.normal_to_face_vector().dot(*camera_direction) - epsilon > 0.0 {
            return false;
        }
        true
    }
}
