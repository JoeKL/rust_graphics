use super::{DrawCommand, Fragment, Frustum, Rasterizer};
use crate::{
    scene::Scene,
    types::{
        color::ColorRGB,
        display::ScreenPoint,
        light::PointLight,
        math::{Mat4x4, Point3D},
        primitives::Vertex,
        shader::{FlatShader, Material, ShadingModel},
    },
};

pub struct Renderer {
    // Input buffers
    vertex_buffer: Vec<Vertex>,
    triangle_index_buffer: Vec<u32>,
    draw_commands: Vec<DrawCommand>,

    // Transformed data
    transformed_vertices: Vec<Vertex>, // After vertex processing

    // Rasterization/Fragment data
    fragment_buffer: Vec<Fragment>, // Output of rasterization
    z_buffer: Vec<f32>,             // Z-buffer for depth testing

    // // Output
    // framebuffer: Vec<ColorRGB>,  // Final color buffer

    // Pipeline state
    material_cache: Vec<Material>,

    // Matrices (could also be per-frame data)
    look_at_matrix: Mat4x4,
    projection_matrix: Mat4x4,
    viewport_matrix: Mat4x4,

    frustum_matrix: Mat4x4,
    view_frustum: Frustum,

    pub rasterizer: Rasterizer,
    pub shader: FlatShader,
}

impl Renderer {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let vertex_buffer: Vec<Vertex> = Vec::new();
        let triangle_index_buffer: Vec<u32> = Vec::new();
        let draw_commands: Vec<DrawCommand> = Vec::new();

        let transformed_vertices: Vec<Vertex> = Vec::new();

        let fragment_buffer: Vec<Fragment> = Vec::new();
        let z_buffer: Vec<f32> = Vec::new();
        let material_cache: Vec<Material> = Vec::new();

        let look_at_matrix: Mat4x4 = Mat4x4::identity();
        let projection_matrix: Mat4x4 = Mat4x4::identity();
        let viewport_matrix: Mat4x4 = Mat4x4::identity();

        let frustum_matrix: Mat4x4 = Mat4x4::identity();
        let view_frustum: Frustum = Frustum::new();

        // let frame_buffer: Vec<Vec<Color>>= Vec::new();
        Self {
            vertex_buffer,
            triangle_index_buffer,
            draw_commands,

            transformed_vertices,

            fragment_buffer,
            z_buffer,
            material_cache,

            look_at_matrix,
            projection_matrix,
            viewport_matrix,

            frustum_matrix,
            view_frustum,

            rasterizer: Rasterizer::new(window_width, window_height),
            shader: FlatShader,
        }
    }

    /// Command Stream - Collect and prepare draw calls
    fn process_commands(&mut self, scene: &mut Scene) {
        // - Set up vertex/index buffer ranges
        // - Track material changes
        // - Handle state changes

        // collection stage: here we need to collect
        // - vetices in the self.vertex_buffer
        // - triangle indices in the self.index_buffer
        // - draw calls in the self.draw_commands vector
        // at some point materials from the resource manager
        // build the depth buffer (this could also be done earlier)

        (
            self.vertex_buffer,
            self.triangle_index_buffer,
            self.draw_commands,
        ) = scene.collect();

        //clone vertices so we can still access original vertices
        self.transformed_vertices = self.vertex_buffer.clone();
    }

    /// Vertex Processing Stage
    fn process_vertices(&mut self, scene: &Scene) {
        let mut transformed_lights: Vec<PointLight> = Vec::new();
        for light in &scene.lights {
            transformed_lights.push(PointLight::new_transformed_light(
                light,
                self.look_at_matrix,
            ))
        }

        for draw_command in &self.draw_commands {
            for vertex_idx in 0..draw_command.vertex_count {
                // 1. Model to World transform (Model space -> World space)
                self.transformed_vertices[draw_command.first_vertex_offset + vertex_idx]
                    .transform(draw_command.transform);

                // 2. World to look_at transform (world space -> view/camera space)
                self.transformed_vertices[draw_command.first_vertex_offset + vertex_idx]
                    .transform(self.look_at_matrix);

                // 3. Lighting calculations (in view space)
                let vertex =
                    &mut self.transformed_vertices[draw_command.first_vertex_offset + vertex_idx];

                vertex.color = self.shader.calc_color(
                    &vertex.position_to_point(),
                    &vertex.normal_to_vector(),
                    &vertex.color,
                    &scene.camera.direction.normalize(),
                    &self.material_cache[draw_command.material_id],
                    &transformed_lights,
                );

                // 4. Projection transform (View space -> Clip space)
                let mut vertex_pos = self.projection_matrix.mul_point(vertex.position_to_point());

                // 5. Homogeneous divide (w)
                vertex_pos.dehomogen();

                //6. Viewport transformation (Clip Space -> Screen space)
                vertex_pos = self.viewport_matrix.mul_point(vertex_pos);
                vertex.position = [vertex_pos.x, vertex_pos.y, vertex_pos.z]
            }
        }
    }

    /// Rasterization Stage
    fn rasterize(&mut self) {
        // - Triangle setup
        // - Generate fragments
        // - Interpolate vertex attributes

        // For each draw command/mesh
        for draw_command in &self.draw_commands {
            let index_start = draw_command.first_triangle_index_offset;
            let index_length = draw_command.triangle_index_count;
            let index_end = index_length + index_start;

            // Process indices in groups of 3 to form triangles
            for i in (index_start..index_end).step_by(3) {
                // Get vertex indices
                let i0 = self.triangle_index_buffer[i];
                let i1 = self.triangle_index_buffer[i + 1];
                let i2 = self.triangle_index_buffer[i + 2];

                // Get transformed vertices
                let v0: &Vertex = &self.transformed_vertices[i0 as usize];
                let v1 = &self.transformed_vertices[i1 as usize];
                let v2 = &self.transformed_vertices[i2 as usize];

                // Check if triangle is partly on screen
                // if only all vertices are offscreen goto next triangle
                if !self.rasterizer.is_triangle_on_screen(v0, v1, v2) {
                    continue;
                }

                let bounds_min_x: i32;
                let bounds_min_y: i32;
                let bounds_max_x: i32;
                let bounds_max_y: i32;

                // create boundingbox from v0, v1, v2
                (bounds_min_x, bounds_min_y, bounds_max_x, bounds_max_y) =
                    self.rasterizer.calculate_bounding_box(&v0, &v1, &v2);

                // For each pixel in triangle's bounding box:
                // traverse the bounding box in scanline
                for y in (bounds_min_y)..(bounds_max_y) {
                    for x in (bounds_min_x)..(bounds_max_x) {
                        //   - Calculate barycentric coordinates
                        let (alpha, beta, gamma) = Rasterizer::calculate_barycentric(
                            x as f32,
                            y as f32,
                            &[v0.position[0], v0.position[1]],
                            &[v1.position[0], v1.position[1]],
                            &[v2.position[0], v2.position[1]],
                        );

                        //   - Test if pixel is inside triangle
                        if (0.0..=1.0).contains(&alpha)
                            && (0.0..=1.0).contains(&beta)
                            && (0.0..=1.0).contains(&gamma)
                        {
                            //   - Interpolate Z, color, normal using barycentric
                            let interpolated_z = alpha * v0.position[2]
                                + beta * v1.position[2]
                                + gamma * v2.position[2];

                            let interpolated_color = [
                                alpha * v0.color[0] + beta * v1.color[0] + gamma * v2.color[0],
                                alpha * v0.color[1] + beta * v1.color[1] + gamma * v2.color[1],
                                alpha * v0.color[2] + beta * v1.color[2] + gamma * v2.color[2],
                            ];

                            let interpolated_normal = [
                                alpha * v0.normal[0] + beta * v1.normal[0] + gamma * v2.normal[0],
                                alpha * v0.normal[1] + beta * v1.normal[1] + gamma * v2.normal[1],
                                alpha * v0.normal[2] + beta * v1.normal[2] + gamma * v2.normal[2],
                            ];

                            //setup z index to access right place in buffer
                            let z_buffer_idx =
                                y as usize * self.rasterizer.framebuffer.get_width() + x as usize;

                            //   - Create and store fragment if Z-test passes
                            // Z-test before creating fragment
                            if interpolated_z < self.z_buffer[z_buffer_idx] {
                                // Only if closer than what's in zbuffer at coordinates
                                self.z_buffer[z_buffer_idx] = interpolated_z; // Update z-buffer

                                self.fragment_buffer.push(Fragment {
                                    x,
                                    y,
                                    z: interpolated_z,
                                    color: interpolated_color,
                                    normal: interpolated_normal,
                                    material_id: draw_command.material_id,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    /// Fragment Processing Stage
    fn process_fragments(&mut self) {
        // Process each fragment in the fragment buffer
        //
        //     // Apply any per-fragment effects
        //     // Could include:
        //     // - Alpha testing
        //     // - Additional material effects
        //     // - Special effects
    }

    /// Blending Stage
    fn blend(&mut self) {
        //nothing to do so far, since transparency is not added yet
        //
        // - Color blending
        // - Final color output
        // - Framebuffer updates

        // Write final color to framebuffer
        for fragment in &self.fragment_buffer {
            self.rasterizer.framebuffer.set_pixel(
                fragment.x,
                fragment.y,
                ColorRGB::from_rgb(
                    ColorRGB::f32_to_color_component(fragment.color[0]),
                    ColorRGB::f32_to_color_component(fragment.color[1]),
                    ColorRGB::f32_to_color_component(fragment.color[2]),
                ),
            );
        }

        // clear buffer afterwards
        self.fragment_buffer.clear();
        self.vertex_buffer.clear();
        self.transformed_vertices.clear();
        self.draw_commands.clear();
    }

    pub fn render_scene(&mut self, scene: &mut Scene) {
        self.rasterizer
            .framebuffer
            .fill(ColorRGB::from_u32(0x101010));

        // Get camera matrices once
        self.look_at_matrix = scene.camera.get_look_at_matrix();
        self.projection_matrix = scene.camera.get_projection_matrix();
        self.viewport_matrix = self.rasterizer.viewport.get_matrix();
        self.frustum_matrix = scene.camera.get_frustum_matrix();

        // Create frustum from frusutm matrix
        self.view_frustum = Frustum::from_matrix(&self.frustum_matrix);

        //create material cache
        self.material_cache = Material::MATERIAL_ARRAY.to_vec();

        // set zbuffer
        let width = self.rasterizer.framebuffer.get_width();
        let height = self.rasterizer.framebuffer.get_height();
        self.z_buffer = vec![f32::INFINITY; width * height];

        self.process_commands(scene);
        self.process_vertices(scene);
        self.rasterize();
        self.process_fragments();
        self.blend();
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
