use super::{DrawCommand, Frustum, Rasterizer, RenderTriangle};
use crate::{
    scene::Scene,
    types::{
        color::ColorRGB,
        display::ScreenPoint,
        math::{Mat4x4, Point3D, Vector3D},
        primitives::Vertex,
        shader::{FlatShader, Material},
    },
};

pub struct Renderer {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
    draw_commands: Vec<DrawCommand>,
    materials: Vec<Material>,
    // frame_buffer: Vec<Vec<Color>>,
    depth_buffer: Vec<Vec<f32>>,

    pub rasterizer: Rasterizer,
    pub shader: FlatShader,
}

impl Renderer {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let vertex_buffer: Vec<Vertex> = Vec::new();
        let index_buffer: Vec<u32> = Vec::new();
        let draw_commands: Vec<DrawCommand> = Vec::new();
        let materials: Vec<Material> = Vec::new();
        // let frame_buffer: Vec<Vec<Color>>= Vec::new();
        let depth_buffer: Vec<Vec<f32>> = Vec::new();
        Self {
            vertex_buffer,
            index_buffer,
            draw_commands,
            materials,
            // frame_buffer,
            depth_buffer,

            rasterizer: Rasterizer::new(window_width, window_height),
            shader: FlatShader,
        }
    }

    pub fn render_scene(&mut self, scene: &mut Scene) {
        self.rasterizer
            .framebuffer
            .fill(ColorRGB::from_u32(0x101010));

        // Get camera matrices once
        let frustum_matrix = &scene.camera.get_frustum_matrix();
        let viewport = self.rasterizer.viewport.get_matrix();

        // Create frustum from camera matrix
        let view_frustum = Frustum::from_matrix(frustum_matrix);

        // collection stage: here we need to collect
        // - vetices in the self.vertex_buffer
        // - triangle indices in the self.index_buffer
        // - draw calls in the self.draw_commands vector
        // at some point materials from the resource manager
        // build the depth buffer (this could also be done earlier)

        (self.vertex_buffer, self.index_buffer, self.draw_commands) = scene.collect_geometry();

        // let vertex_buffer: Vec<(usize, Vec<Vertex>)> = scene.transform_and_collect_vertices();
        // let mesh_ref_buffer: Vec<&Mesh> = scene.collect_mesh_refs();

        //constructing triangles to cull
        let mut triangle_buffer: Vec<RenderTriangle> = Vec::new();

        for draw_command in &self.draw_commands {
            triangle_buffer.extend(Self::construct_triangles(
                &mut self.vertex_buffer,
                &self.index_buffer,
                draw_command,
            ));
        }

        //frustum culling
        triangle_buffer.retain(|t| view_frustum.triangle_in_bounds_conservative(t));

        //backface culling
        triangle_buffer.retain(|t| RenderTriangle::is_front_facing(t, &scene.camera.direction));

        // Sort triangles
        Renderer::z_face_sort(&mut triangle_buffer, &scene.camera.get_position());

        // Render them
        self.render_triangles(&triangle_buffer, frustum_matrix, &viewport, scene);
    }
    pub fn construct_triangles(
        vertices: &mut [Vertex],
        indices: &[u32],
        draw_command: &DrawCommand,
    ) -> Vec<RenderTriangle> {

        for vertex_idx in 0..draw_command.vertex_count {
            vertices[draw_command.first_vertex + vertex_idx].transform(draw_command.transform);
        }

        let mut triangles = Vec::new();

        // Process indices in groups of 3, using the draw command's range
        for i in (0..draw_command.triangle_index_count).step_by(3) {
            // Get vertex indices from index buffer, offset by first_triangle_index
            let i0 = indices[draw_command.first_triangle_index + i] as usize;
            let i1 = indices[draw_command.first_triangle_index + i + 1] as usize;
            let i2 = indices[draw_command.first_triangle_index + i + 2] as usize;

            // Get vertices from vertex buffer, offset by first_vertex
            let v0_idx = draw_command.first_vertex + i0;
            let v1_idx = draw_command.first_vertex + i1;
            let v2_idx = draw_command.first_vertex + i2;
            
            // Get references to transformed vertices
            let v0 = &vertices[v0_idx];
            let v1 = &vertices[v1_idx];
            let v2 = &vertices[v2_idx];

            // let normal_x = v0.normal[0] + v1.normal[0] + v2.normal[0];
            // let normal_y = v0.normal[1] + v1.normal[1] + v2.normal[1];
            // let normal_z = v0.normal[2] + v1.normal[2] + v2.normal[2];
            
            // Calculate edges for normal
            let edge1 = Vector3D::new(
                v1.position[0] - v0.position[0],
                v1.position[1] - v0.position[1],
                v1.position[2] - v0.position[2],
            );
            let edge2 = Vector3D::new(
                v2.position[0] - v0.position[0],
                v2.position[1] - v0.position[1],
                v2.position[2] - v0.position[2],
            );

            // // Calculate normal
            let triangle_normal = edge1.cross(edge2).normalize();
            // let triangle_normal = Vector3D::new(normal_x, normal_y, normal_z).normalize();

            // Create triangle
            let triangle = RenderTriangle {
                vertices: [*v0, *v1, *v2], // or clone() if needed
                normal: [triangle_normal.x, triangle_normal.y, triangle_normal.z],
                material_id: draw_command.material_id as u32,
            };

            triangles.push(triangle);
        }

        triangles
    }
    pub fn z_face_sort(triangles: &mut [RenderTriangle], camera_position: &Point3D) {
        // Sort based on distance to eye
        triangles.sort_by(|a, b| {
            // Calculate centers
            let center_a = a.calculate_center();
            let center_b = b.calculate_center();

            // Calculate squared distances to cam.position
            let dist_a = (center_a.x - camera_position.x).powi(2)
                + (center_a.y - camera_position.y).powi(2)
                + (center_a.z - camera_position.z).powi(2);
            let dist_b = (center_b.x - camera_position.x).powi(2)
                + (center_b.y - camera_position.y).powi(2)
                + (center_b.z - camera_position.z).powi(2);

            // Sort furthest first
            dist_b.partial_cmp(&dist_a).unwrap()
        });
    }

    pub fn render_triangles(
        &mut self,
        triangles: &Vec<RenderTriangle>,
        frustum_matrix: &Mat4x4,
        viewport_matrix: &Mat4x4,
        scene: &Scene,
    ) {
        for triangle in triangles {
            // in World coordinates

            let mut point_0: Point3D = triangle.vertices[0].to_point();
            let mut point_1: Point3D = triangle.vertices[1].to_point();
            let mut point_2: Point3D = triangle.vertices[2].to_point();

            // Frustum transformation
            point_0 = frustum_matrix.mul_point(point_0);
            point_1 = frustum_matrix.mul_point(point_1);
            point_2 = frustum_matrix.mul_point(point_2);

            // in NDC

            // perspective divide
            point_0.dehomogen();
            point_1.dehomogen();
            point_2.dehomogen();

            // Viewport transformation
            point_0 = viewport_matrix.mul_point(point_0);
            point_1 = viewport_matrix.mul_point(point_1);
            point_2 = viewport_matrix.mul_point(point_2);

            //in Screen coordinates
            let screen_point_0 = ScreenPoint {
                y: point_0.y as i32,
                x: point_0.x as i32,
            };
            let screen_point_1 = ScreenPoint {
                x: point_1.x as i32,
                y: point_1.y as i32,
            };
            let screen_point_2 = ScreenPoint {
                x: point_2.x as i32,
                y: point_2.y as i32,
            };

            let material: Vec<Material> = vec![
                Material::MATERIAL_0,
                Material::MATERIAL_1,
                Material::MATERIAL_2,
            ];

            self.rasterizer.draw_triangle(
                screen_point_0,
                screen_point_1,
                screen_point_2,
                Rasterizer::shade_triangle(
                    triangle,
                    &scene.camera.get_position(),
                    &material[triangle.material_id as usize],
                    &scene.lights,
                    &self.shader,
                ),
            );
        }
    }

    pub fn render_axis(&mut self, scene: &mut Scene) {
        let frustum_matrix = scene.camera.get_frustum_matrix();
        let viewport_matrix = self.rasterizer.viewport.get_matrix();

        let origin = Point3D::new(0.0, 0.0, 0.0);
        let x_end = Point3D::new(5.0, 0.0, 0.0); // X axis in red
        let y_end = Point3D::new(0.0, 5.0, 0.0); // Y axis in green
        let z_end = Point3D::new(0.0, 0.0, 5.0); // Z axis in blue

        let axes = [
            (origin, x_end, ColorRGB::RED),   // X axis - red
            (origin, y_end, ColorRGB::GREEN), // Y axis - green
            (origin, z_end, ColorRGB::BLUE),  // Z axis - blue
        ];

        for (start, end, color) in axes {
            let mut start_point = start;
            let mut end_point = end;

            start_point = frustum_matrix.mul_point(start_point);
            end_point = frustum_matrix.mul_point(end_point);

            start_point.dehomogen();
            end_point.dehomogen();

            start_point = viewport_matrix.mul_point(start_point);
            end_point = viewport_matrix.mul_point(end_point);

            let screen_start = ScreenPoint {
                x: start_point.x as i32,
                y: start_point.y as i32,
            };
            let screen_end = ScreenPoint {
                x: end_point.x as i32,
                y: end_point.y as i32,
            };

            self.rasterizer.draw_line(screen_start, screen_end, color);
        }
    }

    pub fn render_light_vectors(&mut self, scene: &mut Scene) {
        let frustum_matrix = scene.camera.get_frustum_matrix();
        let viewport_matrix = self.rasterizer.viewport.get_matrix();

        let origin = Point3D::new(0.0, 0.0, 0.0);

        for lights in &scene.lights {
            let mut start_point = lights.get_position();
            let mut end_point = origin;

            start_point = frustum_matrix.mul_point(start_point);
            end_point = frustum_matrix.mul_point(end_point);

            start_point.dehomogen();
            end_point.dehomogen();

            start_point = viewport_matrix.mul_point(start_point);
            end_point = viewport_matrix.mul_point(end_point);

            let screen_start = ScreenPoint {
                x: start_point.x as i32,
                y: start_point.y as i32,
            };
            let screen_end = ScreenPoint {
                x: end_point.x as i32,
                y: end_point.y as i32,
            };

            self.rasterizer
                .draw_line(screen_start, screen_end, ColorRGB::YELLOW);
        }
    }
}
