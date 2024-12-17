use crate::renderer::DrawCommand;
use crate::types::camera::Camera;
use crate::types::color::ColorRGB;
use crate::types::light::PointLight;
use crate::types::math::{Point3D, Vector3D};
use crate::types::geometry::Mesh;
use crate::types::primitives::Vertex;
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

        camera.set_position(Point3D::new(0.0, 0.0, -20.0));
        camera.look_at(Point3D::new(0.0, 0.0, 0.0));

        //light sources
        let light = PointLight::new(Point3D::new(0.0, 5.0, -5.0), ColorRGB::WHITE, 1.0);

        let lights: Vec<PointLight> = vec![light];

        let mut ball_node = SceneNode::new();
        let mut child_ball_node = SceneNode::new();
        let mut grandchild_ball_node = SceneNode::new();
        
        ball_node.set_mesh(Mesh::create_ball(0));
        child_ball_node.set_mesh(Mesh::create_ball(1));
        grandchild_ball_node.set_mesh(Mesh::create_ball(2));
        
        // ball_node.set_position(Vector3D::new(2.5, 0.0, 0.0));
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

    pub fn transform_and_collect_vertices(&self) -> Vec<(usize, Vec<Vertex>)> {
        let mut vertex_tupels: Vec<(usize, Vec<Vertex>)> = Vec::new();
        let mut node_queue = vec![&self.root_node];

        
        // Keep processing until queue is empty
        while let Some(node) = node_queue.pop() {

            // If this node has a mesh, add a reference to collection
            if let Some(mesh) = &node.mesh {            
                let mut mesh_snapshot = mesh.clone();           // clone since we dont want to override the meshes. we just want a snapshot of them
                mesh_snapshot.transform(node.get_world_transform()); // translate snapshot into world coordinates 
                let mesh_id = mesh_snapshot.id;

                vertex_tupels.push((mesh_id, mesh_snapshot.vertices)); // push into buffer
            }
            
            // Add references to all children to queue
            for child in &node.children {
                node_queue.push(child);
            }
        }
        
        vertex_tupels
    }


    pub fn collect_mesh_refs(&self) -> Vec<&Mesh> {
        let mut mesh_refs: Vec<&Mesh> = Vec::new();
        let mut node_queue = vec![&self.root_node];

        // Keep processing until queue is empty
        while let Some(node) = node_queue.pop() {

            // If this node has a mesh, add a reference to collection
            if let Some(mesh) = &node.mesh {            
                
                mesh_refs.push(mesh); // push into buffer
            }
            
            // Add references to all children to queue
            for child in &node.children {
                node_queue.push(child);
            }
        }
        
        mesh_refs
    }
    
    pub fn collect_geometry(&mut self) -> (Vec<Vertex>, Vec<u32>, Vec<DrawCommand>) {
        let mut vertex_buffer: Vec<Vertex> = Vec::new();
        let mut triangle_index_buffer: Vec<u32> = Vec::new();
        let mut draw_command_buffer: Vec<DrawCommand> = Vec::new();

        let mut node_queue: Vec<&mut SceneNode> = vec![&mut self.root_node];

        // Keep processing until queue is empty
        while let Some(node) = node_queue.pop() {
            let world_transform = node.get_world_transform();

            // If this node has a mesh, add a reference to collection
            if let Some(mesh) = &mut node.mesh {        

                mesh.calculate_vertex_normals();    

                // Create draw command
                draw_command_buffer.push(DrawCommand {
                    first_vertex: vertex_buffer.len(), // first element will be inserted at current length +1
                    vertex_count: mesh.vertices.len(),
                    first_triangle_index: triangle_index_buffer.len(),
                    triangle_index_count: mesh.triangle_indices.len(),
                    material_id: 0,
                    transform: world_transform,  // Store transform for later use
                });
                
                vertex_buffer.extend(&mesh.vertices);
                triangle_index_buffer.extend(&mesh.triangle_indices);
            }
            
            // Add references to all children to queue
            for child in &mut node.children {
                node_queue.push(child);
            }
        }
        (vertex_buffer, triangle_index_buffer, draw_command_buffer)
    }
}
