use crate::types::{geometry::Mesh, math::Mat4x4};

pub struct SceneNode {
    pub local_transform: Mat4x4, //local transformation relative to parent coordinate system
    pub mesh: Option<Mesh>, // Not all nodes need meshes (empty groups/pivots)
    pub children: Vec<SceneNode>,
    dirty_world_transform: bool,    // Track if world matrix needs update
    pub cached_world_transform: Mat4x4, //transformation back to World Coordinates
}

impl SceneNode {
    pub fn new() -> Self {
        let local_transform = Mat4x4::new_identity();
        let mesh: Option<Mesh> = None;
        let children: Vec<SceneNode> = Vec::new();
        let dirty_world_transform = false;
        let cached_world_transform = Mat4x4::new_identity();
        Self {
            local_transform,
            mesh,
            children,
            dirty_world_transform,
            cached_world_transform,
        }
    }

    pub fn set_local_transform(&mut self, new_transform: Mat4x4){
        self.local_transform = new_transform;
        self.dirty_world_transform = true;
        self.update_transforms(None);
    }

    pub fn apply_transform(&mut self, delta_transform: Mat4x4){
        self.local_transform = self.local_transform.mul_mat(delta_transform);
        self.dirty_world_transform = true;
        self.update_transforms(None);
    }

    pub fn set_mesh(&mut self, mesh: Mesh){
        self.mesh = Some(mesh);
    }

    pub fn add_child(&mut self, mut scene_node: SceneNode){
        // First prepare the child with parent's transform
        scene_node.update_transforms(Some(self.cached_world_transform));
        // Then add it to children
        self.children.push(scene_node);
    }

    pub fn update_transforms(&mut self, parent_transform: Option<Mat4x4>) {
        // when there is a parent_transform then pT * T
        let new_world_transform = if let Some(parent) = parent_transform {
            parent.mul_mat(self.local_transform)
        } else {
            self.local_transform
        };

        // Only update if something changed
        if self.dirty_world_transform || new_world_transform != self.cached_world_transform {
            self.cached_world_transform = new_world_transform;

            // Propagate to children
            for child in &mut self.children {
                child.update_transforms(Some(self.cached_world_transform));
            }
        }
        self.dirty_world_transform = false;
    }


}
