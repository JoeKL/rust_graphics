use crate::types::{geometry::Mesh, math::{Mat4x4, Vector3D}};

#[derive(Clone)]
pub struct SceneNode {
    position: Vector3D,
    rotation: Mat4x4,
    scale: Vector3D,

    pub local_transform: Mat4x4, //local transformation relative to parent coordinate system
    pub mesh: Option<Mesh>, // Not all nodes need meshes (empty groups/pivots)
    pub children: Vec<SceneNode>,
    dirty_world_transform: bool,    // Track if world matrix needs update
    pub world_transform: Mat4x4, //transformation back to World Coordinates
}


impl SceneNode {
    pub fn new() -> Self {
        let position = Vector3D::new(0.0, 0.0, 0.0);
        let rotation = Mat4x4::new_identity(); //quaternions ??
        let scale = Vector3D::new(1.0, 1.0, 1.0);

        let local_transform = Mat4x4::new_identity();
        let mesh: Option<Mesh> = None;
        let children: Vec<SceneNode> = Vec::new();
        let dirty_world_transform = false;
        let world_transform = Mat4x4::new_identity();


        Self {
            position,
            rotation,
            scale,
            local_transform,
            mesh,
            children,
            dirty_world_transform,
            world_transform,
        }
    }

    pub fn set_position(&mut self, pos: Vector3D) {
        self.position = pos;
        self.update_local_transform();
    }

    pub fn set_rotation(&mut self, rot: Mat4x4){
        self.rotation = rot;
        self.update_local_transform();
    }

    pub fn set_scale(&mut self, scale: Vector3D) {
        self.scale = scale;
        self.update_local_transform();
    }

    pub fn translate(&mut self, delta: Vector3D) {
        self.position = self.position.add(delta);
        self.update_local_transform();
    }

    pub fn rotate(&mut self, delta_rot: Mat4x4) {
        self.rotation = delta_rot.mul_mat(self.rotation);
        // self.rotation = self.rotation.mul_mat(delta_rot);
        self.update_local_transform();
    }

    pub fn scale(&mut self, delta_scale: Vector3D) {
        self.scale = Vector3D::new(
            self.scale.x * delta_scale.x,
            self.scale.y * delta_scale.y,
            self.scale.z * delta_scale.z
        );
        self.update_local_transform();
    }

    pub fn set_mesh(&mut self, mesh: Mesh){
        self.mesh = Some(mesh);
    }

    pub fn add_child(&mut self, mut scene_node: SceneNode){
        // First prepare the child with parent's transform
        scene_node.update_transforms(Some(self.world_transform));
        // Then add it to children
        self.children.push(scene_node);
    }

    fn update_local_transform(&mut self) {
        // First translate to origin
        let to_origin = Mat4x4::from_translation(Vector3D::new(0.0, 0.0, 0.0));
        // Apply scale and rotation
        let transform = Mat4x4::from_scale(self.scale).mul_mat(self.rotation);
        // Translate back
        let from_origin = Mat4x4::from_translation(self.position); // the problem is here. since the position of child 3 is locally 0,0,2.5 but its world coordiante is 0,0,5.0 its reset to the former one thats the jumping on scale

        // Order: translate back * (scale * rotate) * translate to origin
        self.local_transform = from_origin.mul_mat(transform).mul_mat(to_origin);
        
        self.dirty_world_transform = true;

        // Before update_transforms
        self.update_transforms(None);
    }

    pub fn update_transforms(&mut self, parent_transform: Option<Mat4x4>) {
        // when there is a parent_transform then pT * T
        let new_world_transform = if let Some(parent) = parent_transform {
            parent.mul_mat(self.local_transform)
        } else {
            self.local_transform
        };

        // Only update if something changed
        if self.dirty_world_transform || new_world_transform != self.world_transform {
            self.world_transform = new_world_transform;

            // Propagate to children
            for child in &mut self.children {
                child.update_transforms(Some(self.world_transform));
            }
        }
        self.dirty_world_transform = false;
    }


}
