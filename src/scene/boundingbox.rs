use crate::types::{
    geometry::Mesh,
    math::{Mat4x4, Point3D},
};

#[derive(Clone, Copy)]
pub struct BoundingBox {
    min: [f32; 3],
    max: [f32; 3],
}

impl BoundingBox {
    pub fn from_mesh(mesh: &Mesh) -> BoundingBox {
        let min: [f32; 3];
        let max: [f32; 3];
        (min, max) = BoundingBox::calculate_bounds(mesh);
        Self { min, max }
    }

    pub fn calculate_bounds(mesh: &Mesh) -> ([f32; 3], [f32; 3]) {
        //create a new bounds touple
        let mut bounds = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);

        //for each vertex go through xyz and note the uppder/lower bound
        //when bigger/smaller as max/min
        for vertex in &mesh.vertices {
            for i in 0..3 {
                bounds.0[i] = bounds.0[i].min(vertex.position[i]); // update min
                bounds.1[i] = bounds.1[i].max(vertex.position[i]); // update max
            }
        }

        bounds
    }

    pub fn transform_bounding_box(&mut self, transform: Mat4x4) {
        let mut min_point = Point3D::from_array(self.min);
        let mut max_point = Point3D::from_array(self.max);

        min_point = transform.mul_point(min_point);
        max_point = transform.mul_point(max_point);

        self.min = [min_point.x, min_point.y, min_point.z];
        self.max = [max_point.x, max_point.y, max_point.z];
    }
}
