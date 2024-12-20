#![allow(dead_code)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::types::math::Mat4x4;

use super::Object;

pub struct SceneNode {
    local_transform: Mat4x4,          // Node's transform relative to parent
    world_transform: Mat4x4,          // Cached world transform
    object: Option<Object>,           // Optional because not all nodes need objects
    parent: Weak<RefCell<SceneNode>>, // weak reference to parent so that it will be revoked when
    // parent is popped
    children: Vec<Rc<RefCell<SceneNode>>>, //Each node can have many owners (Rc) but only one can modify it at a time (RefCell)
}

impl SceneNode {
    //create a new Node with no rotation, no scaling, at origin, without mesh, and no transformation stack
    pub fn new() -> Self {
        let local_transform = Mat4x4::identity();
        let world_transform = Mat4x4::identity();
        let object: Option<Object> = None;
        let parent = Weak::new();
        let children: Vec<Rc<RefCell<SceneNode>>> = Vec::new();

        Self {
            local_transform,
            world_transform,
            object,
            parent,
            children,
        }
    }

    pub fn set_object(&mut self, object: Object) {
        self.object = Some(object);
    }

    pub fn remove_object(&mut self) {
        self.object.take();
    }

    // #TODO add set_position, set_scale, set_rotation, translate, scale and rotate

    pub fn set_transform(node: &Rc<RefCell<SceneNode>>, transform: Mat4x4) {
        node.borrow_mut().local_transform = transform;

        SceneNode::update_world_transform(node);
    }

    pub fn transform_node(node: &Rc<RefCell<SceneNode>>, delta_transform: Mat4x4) {
        node.borrow_mut().local_transform =
            node.borrow_mut().local_transform.mul_mat(delta_transform);

        // TODO need to updated bounding boxes of object if option == true
        SceneNode::update_world_transform(node);
    }

    // function udates the world_transform and propergates it to its children
    fn update_world_transform(node: &Rc<RefCell<SceneNode>>) {
        let new_world_transform = {
            // get reference of node to access parent
            let node_ref = node.borrow();

            //try to set parent_world by accessing parents node and getting its worldtransform
            let parent_world = node_ref
                .parent
                .upgrade() // Try to get strong reference to parent
                .map(|p| p.borrow().world_transform) // Get transform if parent exists
                .unwrap_or_else(Mat4x4::identity); // Fall back to identity if parent is gone

            //calculate nodes new world_transform by multiplying the parents world with the nodes
            //local transform
            parent_world.mul_mat(node_ref.local_transform)
        };

        // Update our world transform and all children
        let mut node_mut = node.borrow_mut();
        node_mut.world_transform = new_world_transform;

        // Update all children recursively
        for child in &node_mut.children {
            Self::update_world_transform(child);
        }
    }

    pub fn add_child(parent: &Rc<RefCell<Self>>, child: Rc<RefCell<SceneNode>>) {
        // Set up parent-child relationship
        child.borrow_mut().parent = Rc::downgrade(parent);

        // Update transforms
        Self::update_world_transform(&child);

        // Add to children list
        parent.borrow_mut().children.push(child);
    }

    pub fn get_world_transform(&self) -> Mat4x4 {
        self.world_transform
    }

    pub fn get_local_transform(&self) -> Mat4x4 {
        self.local_transform
    }
}
