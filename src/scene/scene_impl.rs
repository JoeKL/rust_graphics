use super::SceneNode;
use crate::renderer::DrawCommand;
use crate::types::camera::Camera;
use crate::types::color::ColorRGB;
use crate::types::geometry::Mesh;
use crate::types::light::PointLight;
use crate::types::{
    math::{Point3D, Vector3D},
    primitives::Vertex,
};

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
        // let mut child_ball_node = SceneNode::new();
        // let mut grandchild_ball_node = SceneNode::new();

        // ball_node.set_mesh(Mesh::create_ball(0, [0.0, 1.0, 0.8]));
        // child_ball_node.set_mesh(Mesh::create_ball(1, [0.78, 0.42, 0.0]));
        // grandchild_ball_node.set_mesh(Mesh::create_ball(2, [0.78, 0.0, 0.6]));

        ball_node.set_mesh(Mesh::load_obj("models/f-16.obj".into(), 0, [0.0, 1.0, 0.8]));
        // child_ball_node.set_mesh(Mesh::load_obj("teapot.obj".into(), 1, [0.78, 0.42, 0.0]));
        // grandchild_ball_node.set_mesh(Mesh::load_obj("teapot.obj".into(), 2, [0.78, 0.0, 0.6]));

        // ball_node.set_scale(Vector3D {
        //     x: (0.015),
        //     y: (0.015),
        //     z: (0.015),
        //     w: (1),
        // });

        // child_ball_node.set_position(Vector3D::new(2.5 * 100.0, 0.0, 0.0));
        // grandchild_ball_node.set_position(Vector3D::new(2.5 * 100.0, 0.0, 0.0));

        // child_ball_node.add_child(grandchild_ball_node);
        // ball_node.add_child(child_ball_node);
        root_node.add_child(ball_node);

        Scene {
            root_node,
            camera,
            lights,
        }
    }

    // depth first traversal of scene tree and return vertices, triangle_indices and draw commands
    pub fn collect(&mut self) -> (Vec<Vertex>, Vec<u32>, Vec<DrawCommand>) {
        let mut vertex_buffer: Vec<Vertex> = Vec::new();
        let mut triangle_index_buffer: Vec<u32> = Vec::new();
        let mut draw_command_buffer: Vec<DrawCommand> = Vec::new();

        let mut node_queue: Vec<&mut SceneNode> = vec![&mut self.root_node];

        // Keep processing until queue is empty
        while let Some(node) = node_queue.pop() {
            let world_transform = node.get_world_transform();

            if let Some(mesh) = &mut node.mesh {
                let vertex_offset = vertex_buffer.len(); // Store current vertex buffer length

                draw_command_buffer.push(DrawCommand {
                    first_vertex_offset: vertex_buffer.len(), // Start index in vertex buffer (current length before adding new vertices)
                    vertex_count: mesh.vertices.len(), // How many vertices this mesh contains
                    first_triangle_index_offset: triangle_index_buffer.len(), // Start index in index buffer (current length before adding new indices)
                    triangle_index_count: mesh.triangle_indices.len(), // How many indices this mesh contains
                    material_id: mesh.material_indices[0] as usize, // Use first material ID found in mesh (temporary solution)
                    transform: world_transform, // Store node's world transform for vertex transformation
                });
                vertex_buffer.extend(&mesh.vertices);
                // Offset indices by vertex_offset before adding them
                triangle_index_buffer.extend(
                    mesh.triangle_indices
                        .iter()
                        .map(|&i| i + vertex_offset as u32),
                );
            }

            // Add references to all children to queue
            for child in &mut node.children {
                node_queue.push(child);
            }
        }
        (vertex_buffer, triangle_index_buffer, draw_command_buffer)
    }
}
