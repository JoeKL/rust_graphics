use super::{
    font_provider::FontProvider, DrawCommand, FacePass, Fragment, Frustum, Rasterizer, RenderPass,
    VertexNormalPass, VertexPass, WireframePass,
};
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
    utils::bmp::BMP,
};

pub struct Renderer {
    // Input buffers
    pub(crate) vertex_buffer: Vec<Vertex>,
    pub(crate) triangle_index_buffer: Vec<u32>,
    pub(crate) draw_commands: Vec<DrawCommand>,

    // Transformed data
    pub(crate) transformed_vertices: Vec<Vertex>, // After vertex processing
    pub(crate) debug_lines: Vec<[i32; 4]>,

    // Rasterization/Fragment data
    pub(crate) fragment_buffer: Vec<Fragment>, // Output of rasterization
    pub(crate) z_buffer: Vec<f32>,             // Z-buffer for depth testing
    pub(crate) framebuffer: Vec<ColorRGB>,     // Final color buffer

    // Pipeline state
    pub(crate) material_cache: Vec<Material>,

    // Matrices (could also be per-frame data)
    pub(crate) look_at_matrix: Mat4x4,
    pub(crate) projection_matrix: Mat4x4,
    pub(crate) viewport_matrix: Mat4x4,

    pub(crate) frustum_matrix: Mat4x4,
    pub(crate) view_frustum: Frustum,

    pub rasterizer: Rasterizer,
    pub shader: FlatShader,

    pub font_provider: FontProvider,

    pub draw_z_buffer: bool,
    pub draw_wireframe: bool,
    pub draw_vertex: bool,
    pub draw_vertex_normals: bool,
    pub draw_faces: bool,
    pub backface_culling: bool,
}

impl Renderer {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let vertex_buffer: Vec<Vertex> = Vec::new();
        let triangle_index_buffer: Vec<u32> = Vec::new();
        let draw_commands: Vec<DrawCommand> = Vec::new();

        let transformed_vertices: Vec<Vertex> = Vec::new();
        let debug_lines: Vec<[i32; 4]> = Vec::new();

        let fragment_buffer: Vec<Fragment> = Vec::new();
        let z_buffer: Vec<f32> = Vec::new();
        let material_cache: Vec<Material> = Vec::new();

        let look_at_matrix: Mat4x4 = Mat4x4::identity();
        let projection_matrix: Mat4x4 = Mat4x4::identity();
        let viewport_matrix: Mat4x4 = Mat4x4::identity();

        let frustum_matrix: Mat4x4 = Mat4x4::identity();
        let view_frustum: Frustum = Frustum::new();

        let framebuffer: Vec<ColorRGB> = Vec::new();
        let font_provider: FontProvider = FontProvider::new("fonts/monogram.bmp", 3, 6, 12);

        let draw_z_buffer = false;
        let draw_wireframe = false;
        let draw_vertex = false;
        let draw_vertex_normals = false;
        let draw_faces = true;
        let backface_culling = true;

        Self {
            vertex_buffer,
            triangle_index_buffer,
            draw_commands,

            transformed_vertices,
            debug_lines,

            fragment_buffer,
            z_buffer,
            framebuffer,

            material_cache,

            look_at_matrix,
            projection_matrix,
            viewport_matrix,

            frustum_matrix,
            view_frustum,

            rasterizer: Rasterizer::new(window_width, window_height),
            shader: FlatShader,

            font_provider,

            draw_z_buffer,
            draw_wireframe,
            draw_vertex,
            draw_vertex_normals,
            draw_faces,
            backface_culling,
        }
    }

    fn project_point(point: Point3D, matrix: &Mat4x4, viewport_matrix: &Mat4x4) -> ScreenPoint {
        let mut projected = *matrix * point;
        projected.dehomogen();
        projected = *viewport_matrix * projected;
        ScreenPoint {
            x: projected.x as i32,
            y: projected.y as i32,
        }
    }

    /// Command Stream - Collect and prepare draw calls
    fn process_commands(&mut self, scene: &mut Scene) {
        // - Set up vertex/index buffer ranges
        // - Track material changes
        // - Handle state changes

        // collection stage: here we need to collect
        // - vetices in the self.vertex_buffer
        // - triangle indices in the self.triangle_index_buffer
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

                if self.draw_vertex_normals && vertex.has_normal() {
                    let line_len = 0.075;

                    let start_point_view: Point3D = vertex.position_to_point();

                    let end_point_view: Point3D =
                        start_point_view + vertex.normal_to_vector() * line_len;

                    let start_screen = Self::project_point(
                        start_point_view,
                        &self.projection_matrix,
                        &self.viewport_matrix,
                    );
                    let end_screen = Self::project_point(
                        end_point_view,
                        &self.projection_matrix,
                        &self.viewport_matrix,
                    );

                    self.debug_lines.push([
                        start_screen.x,
                        start_screen.y,
                        end_screen.x,
                        end_screen.y,
                    ]);
                }

                // 4. Projection transform (View space -> Clip space)
                let mut vertex_pos = self.projection_matrix * vertex.position_to_point();

                // 5. Homogeneous divide (w)
                vertex_pos.dehomogen();

                //6. Viewport transformation (Clip Space -> Screen space)
                vertex_pos = self.viewport_matrix * vertex_pos;
                vertex.position = [vertex_pos.x, vertex_pos.y, vertex_pos.z];
            }
        }
    }

    /// Rasterization Stage
    fn rasterize(&mut self) {
        if self.draw_faces {
            FacePass.execute(self);
        }
        if self.draw_vertex {
            VertexPass.execute(self);
        }
        if self.draw_wireframe {
            WireframePass.execute(self);
        }
        if self.draw_vertex_normals {
            VertexNormalPass.execute(self);
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

        // get z value range
        let mut z_near: f32 = 0.0;
        let mut z_far: f32 = f32::INFINITY;

        for fragment in &self.fragment_buffer {
            if fragment.z < z_far {
                z_far = fragment.z;
            }
            if fragment.z > z_near {
                z_near = fragment.z;
            }
        }

        let z_range = z_far - z_near; // Pre-calculate the denominator

        // Write final color to framebuffer
        for fragment in &self.fragment_buffer {
            let final_color = if self.draw_z_buffer {
                let z_norm = (fragment.z - z_near) / z_range;
                let color_u8 = (z_norm.clamp(0.0, 1.0) * 255.0) as u8;
                ColorRGB::from_rgb(color_u8, color_u8, color_u8)
            } else {
                // Standard fragment color calculation
                ColorRGB::from_rgb(
                    ColorRGB::f32_to_color_component(fragment.color[0]),
                    ColorRGB::f32_to_color_component(fragment.color[1]),
                    ColorRGB::f32_to_color_component(fragment.color[2]),
                )
            };

            self.rasterizer
                .framebuffer
                .set_pixel(fragment.x, fragment.y, final_color);
        }
    }

    pub fn draw_background_on_framebuffer(&mut self) {
        self.rasterizer
            .framebuffer
            .fill(ColorRGB::from_u32(0x101010));
    }

    pub fn render_scene(&mut self, scene: &mut Scene) {
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

        // clear buffer afterwards
        self.fragment_buffer.clear();
        self.vertex_buffer.clear();
        self.transformed_vertices.clear();
        self.draw_commands.clear();
    }

    pub fn render_axis(&mut self, scene: &mut Scene) {
        let frustum_matrix = scene.camera.get_frustum_matrix();

        let origin = Point3D::new(0.0, 0.0, 0.0);
        let x_end = Point3D::new(1.0, 0.0, 0.0); // X axis in red
        let y_end = Point3D::new(0.0, 1.0, 0.0); // Y axis in green
        let z_end = Point3D::new(0.0, 0.0, 1.0); // Z axis in blue

        let axes = [
            (origin, x_end, ColorRGB::RED),   // X axis - red
            (origin, y_end, ColorRGB::GREEN), // Y axis - green
            (origin, z_end, ColorRGB::BLUE),  // Z axis - blue
        ];

        for (start, end, color) in axes {
            let screen_start = Self::project_point(start, &frustum_matrix, &self.viewport_matrix);
            let screen_end = Self::project_point(end, &frustum_matrix, &self.viewport_matrix);

            self.rasterizer.draw_line(screen_start, screen_end, color);
        }
    }

    pub fn render_grid(&mut self, scene: &mut Scene) {
        let frustum_matrix = scene.camera.get_frustum_matrix();

        let line_color = ColorRGB::from_rgb(32, 32, 32);
        let start_dist = 5.0;
        let grid_lines = 6;
        let y_offset = -0.25;

        let mut axes: Vec<(Point3D, Point3D, ColorRGB)> = Vec::new();
        axes.push((
            Point3D::new(start_dist, y_offset, 0.0),
            Point3D::new(-start_dist, y_offset, 0.0),
            line_color,
        ));
        axes.push((
            Point3D::new(0.0, y_offset, start_dist),
            Point3D::new(0.0, y_offset, -start_dist),
            line_color,
        ));

        for i in 1..grid_lines {
            axes.push((
                Point3D::new(start_dist, y_offset, i as f32),
                Point3D::new(-start_dist, y_offset, i as f32),
                line_color,
            ));
            axes.push((
                Point3D::new(start_dist, y_offset, -i as f32),
                Point3D::new(-start_dist, y_offset, -i as f32),
                line_color,
            ));
            axes.push((
                Point3D::new(i as f32, y_offset, start_dist),
                Point3D::new(i as f32, y_offset, -start_dist),
                line_color,
            ));
            axes.push((
                Point3D::new(-i as f32, y_offset, start_dist),
                Point3D::new(-i as f32, y_offset, -start_dist),
                line_color,
            ));
        }

        for (start, end, color) in axes {
            let screen_start = Self::project_point(start, &frustum_matrix, &self.viewport_matrix);
            let screen_end = Self::project_point(end, &frustum_matrix, &self.viewport_matrix);

            self.rasterizer.draw_line(screen_start, screen_end, color);
        }
    }

    pub fn render_light_vectors(&mut self, scene: &mut Scene) {
        let frustum_matrix = scene.camera.get_frustum_matrix();

        let origin = Point3D::new(0.0, 0.0, 0.0);

        for lights in &scene.lights {
            let start_point = lights.get_position();
            let end_point = origin;

            let screen_start =
                Self::project_point(start_point, &frustum_matrix, &self.viewport_matrix);
            let screen_end = Self::project_point(end_point, &frustum_matrix, &self.viewport_matrix);

            self.rasterizer
                .draw_line(screen_start, screen_end, ColorRGB::YELLOW);
        }
    }

    pub fn render_text(&mut self, text: &str, x_pos: i32, y_pos: i32, color: ColorRGB, scale: u32) {
        let mut x_current = x_pos;

        for char in text.chars() {
            let char_cords = self.font_provider.get_glyph_grid_pos(char);

            let mut character_bmp: BMP = self
                .font_provider
                .get_character(char_cords.0, char_cords.1)
                .scale_up(scale);

            character_bmp.highlight_bmp(ColorRGB::WHITE);

            self.font_provider.draw_as_character(
                &character_bmp,
                &mut self.rasterizer.framebuffer,
                x_current,
                y_pos,
                color,
            );

            x_current += character_bmp.width;
        }
    }
}
