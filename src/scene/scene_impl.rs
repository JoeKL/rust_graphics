use crate::renderer::DrawCommand;
use crate::types::camera::Camera;
use crate::types::color::ColorRGB;
use crate::types::light::PointLight;
use crate::types::math::{Mat4x4, Point3D, Vector3D};
use crate::types::geometry::Mesh;
use crate::types::primitives::Vertex;
use super::SceneNode;

pub struct Scene {
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
        
        ball_node.set_mesh(Mesh::create_cube(0, [0.0, 1.0, 0.8]));

        // ball_node.set_mesh(Mesh::create_ball(0, [0.0, 1.0, 0.8]));
        child_ball_node.set_mesh(Mesh::create_cube(1, [0.78, 0.42, 0.0]));
        grandchild_ball_node.set_mesh(Mesh::create_cube(2, [0.78, 0.0, 0.6]));
        
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

    pub fn collect(&mut self) -> (Vec<Vertex>, Vec<u32>, Vec<DrawCommand>) {
        let mut vertex_buffer: Vec<Vertex> = Vec::new();
        let mut triangle_index_buffer: Vec<u32> = Vec::new();
        let mut draw_command_buffer: Vec<DrawCommand> = Vec::new();
    
        // Helper function to traverse tree recursively
        fn collect_node(
            node: &mut SceneNode,
            vertex_buffer: &mut Vec<Vertex>,
            triangle_index_buffer: &mut Vec<u32>,
            draw_command_buffer: &mut Vec<DrawCommand>,
            parent_transform: Mat4x4,
        ) {
            let world_transform = parent_transform.mul_mat(node.get_world_transform());  // Or however you combine transforms
    
            if let Some(mesh) = &mut node.mesh {
                draw_command_buffer.push(DrawCommand {
                    first_vertex: vertex_buffer.len(),          // Start index in vertex buffer (current length before adding new vertices)
                    vertex_count: mesh.vertices.len(),          // How many vertices this mesh contains
                    first_triangle_index: triangle_index_buffer.len(),  // Start index in index buffer (current length before adding new indices) 
                    triangle_index_count: mesh.triangle_indices.len(),  // How many indices this mesh contains
                    material_id: mesh.material_indices[0] as usize,     // Use first material ID found in mesh (temporary solution)
                    transform: world_transform,                         // Store node's world transform for vertex transformation
                });
                vertex_buffer.extend(&mesh.vertices);
                triangle_index_buffer.extend(&mesh.triangle_indices);
            }
    
            // Recursively process children
            for child in &mut node.children {
                collect_node(child, vertex_buffer, triangle_index_buffer, draw_command_buffer, world_transform);
            }
        }
    
        // Start collection from root
        collect_node(&mut self.root_node, &mut vertex_buffer, &mut triangle_index_buffer, &mut draw_command_buffer, Mat4x4::identity());
    
        (vertex_buffer, triangle_index_buffer, draw_command_buffer)
    }
}
