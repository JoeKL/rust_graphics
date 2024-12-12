use crate::input::InputHandler;
use crate::scene::Scene;
use crate::types::color::ColorRGB;
use crate::types::math::{Mat4x4, Point2D, Point3D};
use crate::types::mesh::Mesh;
use crate::types::primitives::Triangle;
use crate::types::display::ScreenPoint;
use crate::renderer::Rasterizer;
use crate::types::shader::{FlatShader, Material};

pub struct Engine {
    window_width: u32,
    window_height: u32,
    rasterizer: Rasterizer,
    scene: Scene,
    frame: u32,
    draw_axis: bool,
    draw_lights: bool,
}

impl Engine {
    pub fn new(window_width: u32, window_height: u32) -> Engine {
        let rasterizer = Rasterizer::new(window_width as usize, window_height as usize);

        let mut scene = Scene::new();

        let far: f32 = 75.0;
        let near: f32 = 1.0;
        scene.camera.set_projection_params(
            30.0, // 60 degree FOV
            rasterizer.framebuffer.get_width() as f32 / rasterizer.framebuffer.get_height() as f32,
            near,
            far,
        );
        let frame = 0;

        let draw_axis = false;
        let draw_lights = false;

        Engine {
            window_width,
            window_height,
            rasterizer,
            scene,
            frame,
            draw_axis,
            draw_lights,
        }
    }

    pub fn z_face_sort(mesh_list: &Vec<Mesh>, camera_position: Point3D) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = Vec::new();

        for i in 0..mesh_list.len() {
            for triangle in mesh_list[i].get_triangles() {
                triangles.push(triangle);
            }
        }

        // Sort based on distance to eye
        triangles.sort_by(|a, b| {
            // Calculate centers
            let center_a = Point3D::new(
                (a.a.x + a.b.x + a.c.x) / 3.0,
                (a.a.y + a.b.y + a.c.y) / 3.0,
                (a.a.z + a.b.z + a.c.z) / 3.0,
            );
            let center_b = Point3D::new(
                (b.a.x + b.b.x + b.c.x) / 3.0,
                (b.a.y + b.b.y + b.c.y) / 3.0,
                (b.a.z + b.b.z + b.c.z) / 3.0,
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

    pub fn draw_flat_triangles(&mut self, triangles: &Vec<Triangle>) {
        let look_at_projection_matrix = self.scene.camera.get_look_at_projection_matrix();
        let viewport_matrix = self.rasterizer.viewport.get_matrix();
        
        let ambient = 0.1;
        let diffuse = 0.5;
        let specular = 0.5;
        let shininess = 50.0;
        
        let material= Material::new(ColorRGB::from_rgb(0, 255, 200), ambient, diffuse, specular, shininess);
        let shader = FlatShader;

        for triangle in triangles {
            let mut point_0: Point3D = triangle.a;
            let mut point_1: Point3D = triangle.b;
            let mut point_2: Point3D = triangle.c;

            // After look_at_projection
            point_0 = look_at_projection_matrix.mul_point(point_0);
            point_1 = look_at_projection_matrix.mul_point(point_1);
            point_2 = look_at_projection_matrix.mul_point(point_2);

            // After perspective divide
            point_0.dehomogen();
            point_1.dehomogen();
            point_2.dehomogen();

            // After viewport
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
                Rasterizer::shade_triangle(triangle, &self.scene.camera.get_position(), &material, &self.scene.lights, &shader)
  
            );
        }
    }

    pub fn draw_axis(&mut self) {
        let look_at_projection_matrix = self.scene.camera.get_look_at_projection_matrix();
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

    fn rotate_ball_with_mouse(&mut self, input_handler: &InputHandler) {
        if input_handler.is_mouse_button_down(0) {
            let mut x_rot: f32 = 0.00;
            let mut y_rot: f32 = 0.00;

            let dist_center_threshhold = 50.0;

            let screen_center = Point2D::new(
                (self.window_width / 2) as f32,
                (self.window_height / 2) as f32,
            );
            let mut mouse_pos_relative_center = input_handler.get_mouse_position();
            mouse_pos_relative_center.x -= (self.window_width / 2) as f32;
            mouse_pos_relative_center.y -= (self.window_height / 2) as f32;
            let mouse_center_dist_vec = screen_center.add_p(mouse_pos_relative_center);

            if mouse_pos_relative_center.x > dist_center_threshhold {
                y_rot += mouse_pos_relative_center.x / 5000.0;
            }
            if mouse_pos_relative_center.x < -dist_center_threshhold {
                y_rot += mouse_pos_relative_center.x / 5000.0;
            }

            if mouse_pos_relative_center.y > dist_center_threshhold {
                x_rot -= mouse_pos_relative_center.y / 5000.0;
            }

            if mouse_pos_relative_center.y < -dist_center_threshhold {
                x_rot -= mouse_pos_relative_center.y / 5000.0;
            }

            let rot_x_mat = Mat4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, x_rot.cos(), -x_rot.sin(), 0.0],
                [0.0, x_rot.sin(), x_rot.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            let rot_y_mat = Mat4x4::new([
                [y_rot.cos(), 0.0, y_rot.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-y_rot.sin(), 0.0, y_rot.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            self.scene.mesh_list[0].transform_mesh(rot_x_mat);
            self.scene.mesh_list[0].transform_mesh(rot_y_mat);

            let dp_point_start = ScreenPoint::new(screen_center.x as i32, screen_center.y as i32);
            let dp_point_end = ScreenPoint::new(
                mouse_center_dist_vec.x as i32,
                mouse_center_dist_vec.y as i32,
            );

            self.rasterizer
                .draw_line(dp_point_start, dp_point_end, ColorRGB::WHITE);
        }
    }

    fn move_camera(&mut self, input_handler: &InputHandler) {
        if input_handler.is_key_down(minifb::Key::O) {
            let mut current_fov = self.scene.camera.get_fov_in_degrees();
            current_fov += 0.5;
            self.scene.camera.set_fov_in_degrees(current_fov);
        }
        if input_handler.is_key_down(minifb::Key::P) {
            let mut current_fov = self.scene.camera.get_fov_in_degrees();
            current_fov -= 0.5;
            self.scene.camera.set_fov_in_degrees(current_fov);
        }
    }

    fn rotate_lightsources(&mut self, input_handler: &InputHandler) {
        let mut x_rot: f32 = 0.00;
        let mut y_rot: f32 = 0.00;

        let x_rot_delta = 0.1;
        let y_rot_delta = 0.1;

        if input_handler.is_key_down(minifb::Key::Up) {
            x_rot += x_rot_delta;
        }
        if input_handler.is_key_down(minifb::Key::Down) {
            x_rot -= x_rot_delta;
        }

        if input_handler.is_key_down(minifb::Key::Left) {
            y_rot -= y_rot_delta;
        }
        if input_handler.is_key_down(minifb::Key::Right) {
            y_rot += y_rot_delta;
        }

        if x_rot != 0.0 || y_rot != 0.0 {
            let rot_x_mat = Mat4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, x_rot.cos(), -x_rot.sin(), 0.0],
                [0.0, x_rot.sin(), x_rot.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            let rot_y_mat = Mat4x4::new([
                [y_rot.cos(), 0.0, y_rot.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-y_rot.sin(), 0.0, y_rot.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            for light in &mut self.scene.lights {
                let current_light_pos = light.get_position();
                let mut new_light_pos = rot_x_mat.mul_point(current_light_pos);
                new_light_pos = rot_y_mat.mul_point(new_light_pos);
                light.set_position(new_light_pos);
            }
        }
    }

    pub fn draw_light_vectors(&mut self) {
        let look_at_projection_matrix = self.scene.camera.get_look_at_projection_matrix();
        let viewport_matrix = self.rasterizer.viewport.get_matrix();


        let origin = Point3D::new(0.0, 0.0, 0.0);

        for lights in &self.scene.lights {
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

    pub fn run(&mut self, input_handler: &InputHandler) -> Vec<u32> {
        self.frame += 1;
        self.rasterizer.framebuffer.fill(ColorRGB::BLACK);

        if input_handler.is_key_pressed(minifb::Key::Space) {
            if self.draw_axis {
                self.draw_axis = false
            } else {
                self.draw_axis = true
            }
        }

        if input_handler.is_key_pressed(minifb::Key::L) {
            if self.draw_lights {
                self.draw_lights = false
            } else {
                self.draw_lights = true
            }
        }

        self.move_camera(input_handler);

        self.rotate_ball_with_mouse(input_handler);
        self.rotate_lightsources(input_handler);

        if self.draw_axis {
            self.draw_axis();
        }

        let triangles =
            Engine::z_face_sort(&self.scene.mesh_list, self.scene.camera.get_position());

        self.draw_flat_triangles(&triangles);

        if self.draw_lights {
            self.draw_light_vectors();
        }

        return self.rasterizer.framebuffer.get_buffer().to_vec();
    }
}
