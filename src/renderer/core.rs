use crate::{scene::Scene, types::{color::ColorRGB, display::ScreenPoint, math::{Mat4x4, Point3D}, geometry::Mesh, primitives::Triangle, shader::{FlatShader, Material}}};
use super::Rasterizer;

pub struct Renderer {
    pub rasterizer: Rasterizer,
    pub shader: FlatShader,
}

impl Renderer {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        Self {
            rasterizer: Rasterizer::new(window_width, window_height),
            shader: FlatShader,
        }
    }

    pub fn get_window_width(&self) -> usize {
        self.rasterizer.framebuffer.get_width()
    }

    pub fn get_window_height(&self) -> usize {
        self.rasterizer.framebuffer.get_height()
    }

    pub fn get_buffer(&self) -> Vec<u32>{
        self.rasterizer.framebuffer.get_buffer().to_vec()
    }


    pub fn render_scene(&mut self, scene: &mut Scene) {
        self.rasterizer.framebuffer.fill(ColorRGB::BLACK);
        // Get camera matrices once
        let look_at_projection = &scene.camera.get_look_at_projection_matrix();
        let viewport = self.rasterizer.viewport.get_matrix();

        // Sort triangles
        let triangles = Renderer::z_face_sort(&scene.mesh_list, &scene.camera.get_position());

        // Render them
        self.render_triangles(&triangles, look_at_projection, &viewport, scene);

    }


    pub fn z_face_sort(mesh_list: &Vec<Mesh>, camera_position: &Point3D) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = Vec::new();

        for mesh in mesh_list {
            for triangle in mesh.get_triangles() {
                triangles.push(triangle);
            }
        }

        // Sort based on distance to eye
        triangles.sort_by(|a, b| {
            // Calculate centers
            let center_a = Point3D::new(
                (a.a.position.x + a.b.position.x + a.c.position.x) / 3.0,
                (a.a.position.y + a.b.position.y + a.c.position.y) / 3.0,
                (a.a.position.z + a.b.position.z + a.c.position.z) / 3.0,
            );
            let center_b = Point3D::new(
                (b.a.position.x + b.b.position.x + b.c.position.x) / 3.0,
                (b.a.position.y + b.b.position.y + b.c.position.y) / 3.0,
                (b.a.position.z + b.b.position.z + b.c.position.z) / 3.0,
            );

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
        triangles
    }


    pub fn render_triangles(&mut self, triangles: &Vec<Triangle>, look_at_projection_matrix: &Mat4x4, viewport_matrix: &Mat4x4, scene: &Scene) {
       
        let ambient = 0.1;
        let diffuse = 0.5;
        let specular = 0.5;
        let shininess = 50.0;
        
        let material= Material::new(ColorRGB::from_rgb(0, 255, 200), ambient, diffuse, specular, shininess);

        for triangle in triangles {
            let mut point_0: Point3D = triangle.a.position;
            let mut point_1: Point3D = triangle.b.position;
            let mut point_2: Point3D = triangle.c.position;

            point_0 = look_at_projection_matrix.mul_point(point_0);
            point_1 = look_at_projection_matrix.mul_point(point_1);
            point_2 = look_at_projection_matrix.mul_point(point_2);

            // perspective divide
            point_0.dehomogen();
            point_1.dehomogen();
            point_2.dehomogen();

            point_0 = viewport_matrix.mul_point(point_0);
            point_1 = viewport_matrix.mul_point(point_1);
            point_2 = viewport_matrix.mul_point(point_2);

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

            self.rasterizer.draw_triangle(
                screen_point_0,
                screen_point_1,
                screen_point_2,
                Rasterizer::shade_triangle(triangle, &scene.camera.get_position(), &material, &scene.lights, &self.shader)
  
            );
        }
    }


    pub fn render_axis(&mut self, scene: &mut Scene) {
        let look_at_projection_matrix = scene.camera.get_look_at_projection_matrix();
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

            start_point = look_at_projection_matrix.mul_point(start_point);
            end_point = look_at_projection_matrix.mul_point(end_point);

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
                .draw_line(screen_start, screen_end, color);
        }
    }


    pub fn render_light_vectors(&mut self, scene: &mut Scene) {
        let look_at_projection_matrix = scene.camera.get_look_at_projection_matrix();
        let viewport_matrix = self.rasterizer.viewport.get_matrix();


        let origin = Point3D::new(0.0, 0.0, 0.0);

        for lights in &scene.lights {
            let mut start_point = lights.get_position();
            let mut end_point = origin;

            start_point = look_at_projection_matrix.mul_point(start_point);
            end_point = look_at_projection_matrix.mul_point(end_point);

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