use super::{Camera, Mesh, PointLight, SceneNode, Vertex};
use crate::math::{Point3D, Vector3D};
use crate::renderer::DrawCommand;
use crate::renderer::color::ColorRGB;

pub struct Scene {
    pub root_node: SceneNode,
}

impl Scene {
    pub fn new() -> Scene {
        let mut root_node = SceneNode::new();

        // camera
        let pos = Point3D::new(0.0, 0.0, 10.0);
        let target = Point3D::new(0.0, 0.0, 0.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);

        let mut camera: Camera = Camera::new(pos, target, up);

        camera.set_position(Point3D::new(0.0, 2.0, -10.0));
        camera.look_at(Point3D::new(0.0, 0.0, 0.0));

        let mut camera_node = SceneNode::new();
        camera_node.set_camera(camera);
        root_node.add_child(camera_node);

        // light sources
        let light = PointLight::new(Point3D::new(0.0, 3.0, -3.0), ColorRGB::WHITE, 1.0);
        let mut light_node = SceneNode::new();
        light_node.set_light(light);
        root_node.add_child(light_node);

        // model
        let mut model_node = SceneNode::new();

        match Mesh::load_obj("models/f-16.obj", 2, [1.0, 1.0, 1.0]) {
            Ok(mesh) => model_node.set_mesh(mesh),
            Err(e) => {
                eprintln!("Failed to load 'models/f-16.obj': {}", e);
            }
        }

        root_node.add_child(model_node);

        Scene { root_node }
    }

    pub fn find_camera(&self) -> Option<&Camera> {
        let mut node_queue = vec![&self.root_node];
        while let Some(node) = node_queue.pop() {
            if node.camera.is_some() {
                return node.camera.as_ref();
            }
            for child in &node.children {
                node_queue.push(child);
            }
        }
        None
    }

    pub fn find_camera_mut(&mut self) -> Option<&mut Camera> {
        let mut node_queue = vec![&mut self.root_node];
        while let Some(node) = node_queue.pop() {
            if node.camera.is_some() {
                return node.camera.as_mut();
            }
            for child in &mut node.children {
                node_queue.push(child);
            }
        }
        None
    }

    pub fn get_active_camera(&self) -> Camera {
        let mut node_queue = vec![&self.root_node];
        while let Some(node) = node_queue.pop() {
            if let Some(camera) = &node.camera {
                let world_transform = node.get_world_transform();
                return camera.to_world(&world_transform);
            }
            for child in &node.children {
                node_queue.push(child);
            }
        }
        // Fallback default camera if none is found
        Camera::new(
            Point3D::new(0.0, 2.0, -10.0),
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        )
    }

    pub fn collect_lights(&self) -> Vec<PointLight> {
        let mut lights = Vec::new();
        let mut node_queue = vec![&self.root_node];
        while let Some(node) = node_queue.pop() {
            if let Some(light) = &node.light {
                let world_transform = node.get_world_transform();
                lights.push(light.to_world(&world_transform));
            }
            for child in &node.children {
                node_queue.push(child);
            }
        }
        lights
    }

    pub fn update_lights<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut PointLight),
    {
        let mut node_queue = vec![&mut self.root_node];
        while let Some(node) = node_queue.pop() {
            if let Some(light) = &mut node.light {
                f(light);
            }
            for child in &mut node.children {
                node_queue.push(child);
            }
        }
    }

    pub fn collect(&self) -> (Vec<Vertex>, Vec<u32>, Vec<DrawCommand>) {
        let mut vertex_buffer: Vec<Vertex> = Vec::new();
        let mut triangle_index_buffer: Vec<u32> = Vec::new();
        let mut draw_command_buffer: Vec<DrawCommand> = Vec::new();

        let mut node_queue: Vec<&SceneNode> = vec![&self.root_node];

        // Keep processing until queue is empty
        while let Some(node) = node_queue.pop() {
            let world_transform = node.get_world_transform();

            // if node has a mesh add it to "to-be-drawn" objects
            if let Some(mesh) = &node.mesh {
                let vertex_offset = vertex_buffer.len(); // Store current vertex buffer length

                draw_command_buffer.push(DrawCommand {
                    first_vertex_offset: vertex_buffer.len(), // Start index in vertex buffer (current length before adding new vertices)
                    vertex_count: mesh.vertices.len(), // How many vertices this mesh contains
                    first_triangle_index_offset: triangle_index_buffer.len(), // Start index in index buffer (current length before adding new indices)
                    triangle_index_count: mesh.triangle_indices.len(), // How many indices this mesh contains
                    material_id: mesh.material_indices[0] as usize, // Use first material ID found in mesh (temporary solution)
                    transform: world_transform, // Store node's world transform (transformaton to place in world space) for vertex transformation
                });
                vertex_buffer.extend(&mesh.vertices);
                // Offset indices by vertex_offset before adding them
                triangle_index_buffer.extend(
                    mesh.triangle_indices
                        .iter()
                        .map(|&i| i + vertex_offset as u32),
                );
            }

            // Add references of all children of the current node to the node queue
            for child in &node.children {
                node_queue.push(child);
            }
        }
        (vertex_buffer, triangle_index_buffer, draw_command_buffer)
    }
}
