use crate::renderer::RenderTriangle;
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

        camera.set_position(Point3D::new(10.0, 10.0, -20.0));
        camera.look_at(Point3D::new(0.0, 0.0, 0.0));

        //light sources
        let light = PointLight::new(Point3D::new(0.0, 5.0, -5.0), ColorRGB::WHITE, 1.0);

        let lights: Vec<PointLight> = vec![light];

        let mut ball_node = SceneNode::new();
        let mut child_ball_node = SceneNode::new();
        let mut grandchild_ball_node = SceneNode::new();
        
        ball_node.set_mesh(Mesh::create_ball());
        child_ball_node.set_mesh(Mesh::create_ball());
        grandchild_ball_node.set_mesh(Mesh::create_ball());
        
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

    pub fn collect_triangles(&self) -> Vec<RenderTriangle> {
        let mut transformed_triangles: Vec<RenderTriangle> = Vec::new();
        let mut node_queue = vec![&self.root_node];

        
        // Keep processing until queue is empty
        while let Some(node) = node_queue.pop() {

            // If this node has a mesh, add a reference to collection
            if let Some(mesh) = &node.mesh {            
                let mut mesh_snapshot = mesh.clone();           // clone since we dont want to override the meshes. we just want a snapshot of them
                mesh_snapshot.transform(node.get_world_transform()); // translate snapshot into world coordinates 
                transformed_triangles.extend(mesh_snapshot.get_render_triangles()); // push into buffer
            }
            
            // Add references to all children to queue
            for child in &node.children {
                node_queue.push(child);
            }
        }
        
        transformed_triangles
    }
}
