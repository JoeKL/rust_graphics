use crate::types::geometry::Mesh;
use crate::types::math::Mat4x4;

use super::BoundingBox;

pub struct Object {
    mesh: Mesh,
    local_bounds: BoundingBox, // Bounds in model space
    world_bounds: BoundingBox, // Bounds in world space
    mesh_transform: Mat4x4,    // transformation relative to local | "directly on mesh"
}

impl Object {
    pub fn new(
        mesh: Mesh,
        mesh_transform: Mat4x4,
        node_local_transform: Mat4x4,
        node_world_transform: Mat4x4,
    ) -> Self {
        // calculate local_bounds
        let bounds = BoundingBox::from_mesh(&mesh);

        let mut local_bounds = bounds.clone();
        local_bounds.transform_bounding_box(node_local_transform.mul_mat(mesh_transform));

        // calculate world_bounds
        let mut world_bounds = bounds.clone();
        world_bounds.transform_bounding_box(node_world_transform.mul_mat(mesh_transform));

        Self {
            mesh,
            local_bounds,
            world_bounds,
            mesh_transform,
        }
    }

    pub fn set_mesh_transform(
        &mut self,
        transform: Mat4x4,
        node_local: Mat4x4,
        node_world: Mat4x4,
    ) {
        self.mesh_transform = transform;
        // Update both bounds
        self.local_bounds
            .transform_bounding_box(node_local.mul_mat(self.mesh_transform));
        self.world_bounds
            .transform_bounding_box(node_world.mul_mat(self.mesh_transform));
    }
}
