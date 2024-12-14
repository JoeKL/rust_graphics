#![allow(dead_code)]

use crate::types::{
    geometry::Mesh,
    math::{Mat4x4, Vector3D},
};

#[derive(Clone)]
pub struct SceneNode {
    position: Vector3D, // current position
    rotation: Mat4x4,   // current rotation
    scale: Vector3D,    // current scale

    has_dirty_locals: bool,

    pub mesh: Option<Mesh>, // Not all nodes need meshes (empty groups/pivots)
    pub children: Vec<SceneNode>, // Vector of child nodes
    pub transform_stack: Vec<Mat4x4>, // transformation stack stacks the necessary transformations from root to child for each node
}

impl SceneNode {
    //create a new Node with no rotation, no scaling, at origin, without mesh, and no transformation stack
    pub fn new() -> Self {
        let position = Vector3D::new(0.0, 0.0, 0.0);
        let rotation = Mat4x4::new_identity();
        let scale = Vector3D::new(1.0, 1.0, 1.0);

        let has_dirty_locals = false;

        let mesh: Option<Mesh> = None;
        let children: Vec<SceneNode> = Vec::new();
        let transform_stack: Vec<Mat4x4> = Vec::new();

        Self {
            position,
            rotation,
            scale,
            has_dirty_locals,
            mesh,
            children,
            transform_stack,
        }
    }

    //overwrites the current local position
    pub fn set_position(&mut self, pos: Vector3D) {
        self.position = pos;
        self.has_dirty_locals = true;
        self.update_local_transform();
    }

    //overwrites the current local rotation
    pub fn set_rotation(&mut self, rot: Mat4x4) {
        self.rotation = rot;
        self.has_dirty_locals = true;
        self.update_local_transform();
    }

    //overwrites the current local scale
    pub fn set_scale(&mut self, scale: Vector3D) {
        self.scale = scale;
        self.has_dirty_locals = true;
        self.update_local_transform();
    }

    //adds a delta to local position
    pub fn translate(&mut self, delta: Vector3D) {
        self.position = self.position.add(delta);
        self.has_dirty_locals = true;
        self.update_local_transform();
    }

    //adds a delta to local rotation
    pub fn rotate(&mut self, delta_rot: Mat4x4) {
        self.rotation = delta_rot.mul_mat(self.rotation);
        self.has_dirty_locals = true;
        self.update_local_transform();
    }

    //adds a delta to local scale
    pub fn scale(&mut self, delta_scale: Vector3D) {
        self.scale = Vector3D::new(
            self.scale.x * delta_scale.x,
            self.scale.y * delta_scale.y,
            self.scale.z * delta_scale.z,
        );
        self.has_dirty_locals = true;
        self.update_local_transform();
    }

    // appends mesh to node
    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = Some(mesh);
    }

    // appends child to current node
    pub fn add_child(&mut self, mut scene_node: SceneNode) {
        // copy its parents transformation stack
        scene_node.transform_stack = self.transform_stack.clone();

        // calculate local transformation based on given information
        let local_transform = scene_node.calculate_local_transform();

        // add local transformation to stack as last element, so it can be easily retrived and modified
        scene_node.transform_stack.push(local_transform);

        // Then add it to children
        self.children.push(scene_node);
    }

    //calculates local transformation based on current self.rotation, self.scale and self.translation
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

    // updates local transformation
    fn update_local_transform(&mut self) {
        //recalculates local transformation
        let updated_local_transform = self.calculate_local_transform();

        //swaps current local transformation with updated one
        self.transform_stack.pop();
        self.transform_stack.push(updated_local_transform);

        //reset because locals now updated
        self.has_dirty_locals = false;

        // propagate changes of parents locals to children
        self.update_children_stacks();
    }

    // update children's stacks recursively
    fn update_children_stacks(&mut self) {
        for child in &mut self.children {

            let child_local = match child.has_dirty_locals {
                false => {
                    // If locals are dirty, try to reuse existing transform or recalculate
                    child.transform_stack
                        .pop()
                        .unwrap_or_else(|| child.calculate_local_transform())
                }
                true => {
                    // If locals aren't dirty, just recalculate
                    child.calculate_local_transform()
                }
            };

            // Give child a copy of transformation_stack except its last is empty
            child.transform_stack = self.transform_stack[..self.transform_stack.len() - 1].to_vec();

            // Add updated local transform of childs parent
            child
                .transform_stack
                .push(*self.transform_stack.last().unwrap());

            // add child's local transform
            child.transform_stack.push(child_local);

            // Recurse
            child.update_children_stacks();
        }
    }

    // get final world transform for rendering
    pub fn get_world_transform(&self) -> Mat4x4 {
        // Multiply entire stack
        self.transform_stack
            .iter()
            .fold(Mat4x4::new_identity(), |acc, transform| {
                acc.mul_mat(*transform)
            })
    }
}
