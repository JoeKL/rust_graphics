use crate::types::{
    geometry::Mesh,
    math::{Mat4x4, Vector3D},
};

#[derive(Clone)]
pub struct SceneNode {
    position: Vector3D,
    rotation: Mat4x4,
    scale: Vector3D,

    pub mesh: Option<Mesh>, // Not all nodes need meshes (empty groups/pivots)
    pub children: Vec<SceneNode>,
    dirty_world_transform: bool, // Track if world matrix needs update
    pub transform_stack: Vec<Mat4x4>,
}

impl SceneNode {
    pub fn new() -> Self {
        let position = Vector3D::new(0.0, 0.0, 0.0);
        let rotation = Mat4x4::new_identity();
        let scale = Vector3D::new(1.0, 1.0, 1.0);

        let mesh: Option<Mesh> = None;
        let children: Vec<SceneNode> = Vec::new();
        let dirty_world_transform = false;
        let transform_stack: Vec<Mat4x4> = Vec::new();

        Self {
            position,
            rotation,
            scale,
            mesh,
            children,
            dirty_world_transform,
            transform_stack,
        }
    }

    pub fn set_position(&mut self, pos: Vector3D) {
        self.position = pos;
        self.update_local_transform();
    }

    pub fn set_rotation(&mut self, rot: Mat4x4) {
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
            self.scale.z * delta_scale.z,
        );
        self.update_local_transform();
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = Some(mesh);
    }

    pub fn add_child(&mut self, mut scene_node: SceneNode) {
        scene_node.transform_stack = self.transform_stack.clone(); // copy its parents transformation stack
        let local_transform = scene_node.calculate_local_transform();
        scene_node.transform_stack.push(local_transform);

        // Then add it to children
        self.children.push(scene_node);
    }

    fn calculate_local_transform(&mut self) -> Mat4x4 {
        // First translate to origin
        let to_origin = Mat4x4::from_translation(Vector3D::new(0.0, 0.0, 0.0));
        // Apply scale and rotation
        let transform = Mat4x4::from_scale(self.scale).mul_mat(self.rotation);
        // Translate back
        let from_origin = Mat4x4::from_translation(self.position); // the problem is here. since the position of child 3 is locally 0,0,2.5 but its world coordiante is 0,0,5.0 its reset to the former one thats the jumping on scale

        // Order: translate back * (scale * rotate) * translate to origin
        from_origin.mul_mat(transform).mul_mat(to_origin)
    }

    fn update_local_transform(&mut self) {
        let updated_local_transform = self.calculate_local_transform();

        self.transform_stack.pop();
        self.transform_stack.push(updated_local_transform);

        self.dirty_world_transform = true;
        // Add this line to propagate changes
        self.update_children_stacks();
    }

    // Add this function to update children's stacks recursively
    fn update_children_stacks(&mut self) {
        for child in &mut self.children {
            // Give child a copy of our stack except its last entry
            child.transform_stack = self.transform_stack[..self.transform_stack.len() - 1].to_vec();
            // Add our updated local transform
            child
                .transform_stack
                .push(self.transform_stack.last().unwrap().clone());
            // Calculate and add child's local transform
            let child_local = child.calculate_local_transform();
            child.transform_stack.push(child_local);
            // Recurse
            child.update_children_stacks();
        }
    }

    // Add this to get final world transform for rendering
    pub fn get_world_transform(&self) -> Mat4x4 {
        // Multiply entire stack
        self.transform_stack
            .iter()
            .fold(Mat4x4::new_identity(), |acc, transform| {
                acc.mul_mat(*transform)
            })
    }
}
