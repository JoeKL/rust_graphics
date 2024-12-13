use crate::types::camera::Camera;
use crate::types::color::ColorRGB;
use crate::types::light::PointLight;
use crate::types::math::{Point3D, Vector3D};
use crate::types::geometry::Mesh;
use super::SceneNode;

pub struct Scene {
    #[allow(dead_code)]
    pub root_node: SceneNode,
    pub camera: Camera,
    pub lights: Vec<PointLight>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut root_node = SceneNode::new();

        // camera
        let pos = Point3D::new(0.0, 0.0, 10.0);
        let target = Point3D::new(0.0, 0.0, 0.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);

        let mut camera: Camera = Camera::new(pos, target, up);

        camera.set_position(Point3D::new(0.0, 0.0, -10.0));
        camera.look_at(Point3D::new(0.0, 0.0, 0.0));

        //light sources
        let light = PointLight::new(Point3D::new(0.0, 5.0, -5.0), ColorRGB::WHITE, 1.0);
        let light2 = PointLight::new(Point3D::new(-10.0, 10.0, 0.0), ColorRGB::WHITE, 0.5);

        let lights: Vec<PointLight> = vec![light, light2];

        let mut ball_node = SceneNode::new();
        let mut child_ball_node = SceneNode::new();
        let mut grandchild_ball_node = SceneNode::new();
        
        ball_node.set_mesh(Mesh::new_ball());
        child_ball_node.set_mesh(Mesh::new_ball());
        grandchild_ball_node.set_mesh(Mesh::new_ball());
        
        child_ball_node.set_position(Vector3D::new(2.5, 0.0, 0.0));
        grandchild_ball_node.set_position(Vector3D::new(2.5, 0.0, 0.0));
        
        child_ball_node.add_child(grandchild_ball_node);
        ball_node.add_child(child_ball_node);
        root_node.add_child(ball_node);

        Scene {
            root_node,
            camera,
            lights,
        }
    }

    pub fn collect_mesh_refs(&self) -> Vec<Mesh> {
        let mut transformed_meshes: Vec<Mesh> = Vec::new();
        let mut node_queue: Vec<&SceneNode> = Vec::new();
        
        // Start with root node
        node_queue.push(&self.root_node);
        
        // Keep processing until queue is empty
        while let Some(node) = node_queue.pop() {

            // If this node has a mesh, add a reference to our collection
            if let Some(mesh) = &node.mesh {            
                let mut mesh_copy = mesh.clone();
                
                mesh_copy.transform(node.get_world_transform());
                transformed_meshes.push(mesh_copy);
            }
            
            // Add references to all children to our queue
            for child in &node.children {
                node_queue.push(child);
            }
        }
        
        transformed_meshes
    }
}
