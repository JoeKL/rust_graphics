use crate::input::InputHandler;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::types::color::ColorRGB;
use crate::types::display::ScreenPoint;
use crate::types::math::{Mat4x4, Point2D, Point3D, Vector3D};

pub struct Engine {
    renderer: Renderer,
    scene: Scene,
    frame: u32,
    augmentation_segment: i32,

    draw_axis: bool,
    draw_grid: bool,
    draw_lights: bool,
    draw_mouse_line: bool,

    orbit_yaw: f32,
    orbit_pitch: f32,
}

impl Engine {
    pub fn new(window_width: u32, window_height: u32) -> Engine {
        let renderer = Renderer::new(window_width as usize, window_height as usize);
        let mut scene = Scene::new();

        let far: f32 = 75.0;
        let near: f32 = 1.0;

        scene.camera.set_projection_params(
            30.0, // 60 degree FOV
            window_width as f32 / window_height as f32,
            near,
            far,
        );

        let frame = 0;

        let augmentation_segment = 0;

        let orbit_yaw = 150.0;
        let orbit_pitch = 10.0;

        let draw_axis = true;
        let draw_grid = true;
        let draw_lights = false;
        let draw_mouse_line = false;

        Engine {
            renderer,
            scene,
            frame,
            augmentation_segment,

            draw_axis,
            draw_grid,
            draw_lights,
            draw_mouse_line,

            orbit_yaw,
            orbit_pitch,
        }
    }

    fn process_input(&mut self, input_handler: &InputHandler) {
        // if input_handler.is_key_pressed(minifb::Key::Space) {
        //     self.augmentation_segment += 1;
        //     if self.augmentation_segment > 2 {
        //         self.augmentation_segment = 0;
        //     }
        // }

        if input_handler.is_key_pressed(minifb::Key::K) {
            // toggles draw_axis
            self.draw_axis = !self.draw_axis;
        }

        if input_handler.is_key_pressed(minifb::Key::G) {
            // toggles draw_grid
            self.draw_grid = !self.draw_grid;
        }

        if input_handler.is_key_pressed(minifb::Key::H) {
            // toggles draw_faces
            self.renderer.draw_faces = !self.renderer.draw_faces;
        }

        if input_handler.is_key_pressed(minifb::Key::L) {
            //toggle draw_lights
            self.draw_lights = !self.draw_lights;
        }

        if input_handler.is_key_pressed(minifb::Key::Z) {
            //toggle draw_vertex
            self.renderer.draw_z_buffer = !self.renderer.draw_z_buffer;
        }

        if input_handler.is_key_pressed(minifb::Key::C) {
            //toggle draw_vertex
            self.renderer.draw_vertex = !self.renderer.draw_vertex;
        }

        if input_handler.is_key_pressed(minifb::Key::V) {
            //toggle draw_vertex_normals
            self.renderer.draw_vertex_normals = !self.renderer.draw_vertex_normals;
        }

        if input_handler.is_key_pressed(minifb::Key::X) {
            //next render mode
            self.renderer.draw_wireframe = !self.renderer.draw_wireframe;
        }

        self.change_camera_fov(input_handler);
        self.rotate_lightsources(input_handler);

        self.rotate_model_with_mouse(input_handler);
        self.orbit_camera_with_mouse(input_handler);

        self.orbit_camera(input_handler);
        self.iso_scale_model(input_handler);
    }

    fn change_camera_fov(&mut self, input_handler: &InputHandler) {
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

        let x_rot_delta = 0.05;
        let y_rot_delta = 0.05;

        if input_handler.is_key_down(minifb::Key::W) {
            x_rot += x_rot_delta;
        }
        if input_handler.is_key_down(minifb::Key::S) {
            x_rot -= x_rot_delta;
        }

        if input_handler.is_key_down(minifb::Key::A) {
            y_rot -= y_rot_delta;
        }
        if input_handler.is_key_down(minifb::Key::D) {
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

    fn rotate_model_with_mouse(&mut self, input_handler: &InputHandler) {
        if input_handler.is_mouse_button_down(1) {
            let mut x_rot: f32 = 0.00;
            let mut y_rot: f32 = 0.00;

            let dist_center_threshhold = 50.0;

            let mut mouse_pos_relative_center = input_handler.get_mouse_position();
            mouse_pos_relative_center.x -=
                (self.renderer.rasterizer.framebuffer.get_width() / 2) as f32;
            mouse_pos_relative_center.y -=
                (self.renderer.rasterizer.framebuffer.get_height() / 2) as f32;

            let rotation_factor = 25000.0;

            if mouse_pos_relative_center.x > dist_center_threshhold {
                y_rot += mouse_pos_relative_center.x / rotation_factor;
            }
            if mouse_pos_relative_center.x < -dist_center_threshhold {
                y_rot += mouse_pos_relative_center.x / rotation_factor;
            }

            if mouse_pos_relative_center.y > dist_center_threshhold {
                x_rot -= mouse_pos_relative_center.y / rotation_factor;
            }

            if mouse_pos_relative_center.y < -dist_center_threshhold {
                x_rot -= mouse_pos_relative_center.y / rotation_factor;
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

            let focus_segment = match self.augmentation_segment {
                0 => &mut self.scene.root_node.children[0],
                1 => &mut self.scene.root_node.children[0].children[0],
                2 => &mut self.scene.root_node.children[0].children[0].children[0],
                _ => return, // Or handle other cases
            };

            let combined_rot = rot_x_mat.mul_mat(rot_y_mat);
            focus_segment.rotate(combined_rot);

            self.draw_mouse_line = true;
        } else {
            self.draw_mouse_line = false
        }
    }

    fn orbit_camera_with_mouse(&mut self, input_handler: &InputHandler) {
        if input_handler.is_mouse_button_down(0) {
            let mut current_position = self.scene.camera.get_position();

            let rot_speed = 1.0;
            let target = Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            };
            let distance = current_position.sub_p(target).length();

            let dist_center_threshhold = 25.0;

            let mut mouse_pos_relative_center = input_handler.get_mouse_position();
            mouse_pos_relative_center.x -=
                (self.renderer.rasterizer.framebuffer.get_width() / 2) as f32;
            mouse_pos_relative_center.y -=
                (self.renderer.rasterizer.framebuffer.get_height() / 2) as f32;

            let rotation_factor = 0.005;

            if mouse_pos_relative_center.x > dist_center_threshhold {
                self.orbit_yaw += mouse_pos_relative_center.x * rotation_factor;
            }
            if mouse_pos_relative_center.x < -dist_center_threshhold {
                self.orbit_yaw += mouse_pos_relative_center.x * rotation_factor;
            }

            if mouse_pos_relative_center.y > dist_center_threshhold {
                self.orbit_pitch -= mouse_pos_relative_center.y * rotation_factor;
            }
            if mouse_pos_relative_center.y < -dist_center_threshhold {
                self.orbit_pitch -= mouse_pos_relative_center.y * rotation_factor;
            }

            self.orbit_pitch = self.orbit_pitch.clamp(-89.0, 89.0);

            let pitch_rad = self.orbit_pitch.to_radians();
            let yaw_rad = self.orbit_yaw.to_radians();

            let h_distance = distance * pitch_rad.cos();
            let x = h_distance * yaw_rad.sin();
            let y = distance * pitch_rad.sin();
            let z = h_distance * yaw_rad.cos();

            self.scene.camera.set_position(Point3D { x, y, z, w: 1.0 });

            self.scene.camera.look_at(target);

            self.draw_mouse_line = true;
        } else {
            self.draw_mouse_line = false
        }
    }

    fn orbit_camera(&mut self, input_handler: &InputHandler) {
        let mut current_position = self.scene.camera.get_position();

        let rot_speed = 1.0;
        let target = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        };

        let distance = current_position.sub_p(target).length();

        if input_handler.is_key_down(minifb::Key::Left) {
            self.orbit_yaw -= rot_speed;
        }
        if input_handler.is_key_down(minifb::Key::Right) {
            self.orbit_yaw += rot_speed;
        }
        if input_handler.is_key_down(minifb::Key::Up) {
            self.orbit_pitch += rot_speed;
        }
        if input_handler.is_key_down(minifb::Key::Down) {
            self.orbit_pitch -= rot_speed;
        }

        self.orbit_pitch = self.orbit_pitch.clamp(-89.0, 89.0);

        let pitch_rad = self.orbit_pitch.to_radians();
        let yaw_rad = self.orbit_yaw.to_radians();

        let h_distance = distance * pitch_rad.cos();
        let x = h_distance * yaw_rad.sin();
        let y = distance * pitch_rad.sin();
        let z = h_distance * yaw_rad.cos();

        self.scene.camera.set_position(Point3D { x, y, z, w: 1.0 });

        self.scene.camera.look_at(target);
    }

    fn iso_scale_model(&mut self, input_handler: &InputHandler) {
        let mut scale: f32 = 1.0;

        let scale_delta = 0.01;

        if input_handler.is_key_down(minifb::Key::N) {
            scale -= scale_delta;
        }
        if input_handler.is_key_down(minifb::Key::M) {
            scale += scale_delta;
        }

        let focus_segment = match self.augmentation_segment {
            0 => &mut self.scene.root_node.children[0],
            1 => &mut self.scene.root_node.children[0].children[0],
            2 => &mut self.scene.root_node.children[0].children[0].children[0],
            _ => return, // Or handle other cases
        };

        if scale != 1.0 {
            focus_segment.scale(Vector3D::new(scale, scale, scale));
        }
    }

    pub fn run(&mut self, input_handler: &InputHandler) -> &[u32] {
        self.frame += 1;

        // Handle input
        self.process_input(input_handler);

        self.renderer.draw_background_on_framebuffer();

        if self.draw_grid {
            self.renderer.render_grid(&mut self.scene);
        }

        // Render
        self.renderer.render_scene(&mut self.scene);

        // Debug renders
        if self.draw_axis {
            self.renderer.render_axis(&mut self.scene);
        }
        if self.draw_lights {
            self.renderer.render_light_vectors(&mut self.scene);
        }

        if self.draw_mouse_line {
            let screen_center = Point2D::new(
                (self.renderer.rasterizer.framebuffer.get_width() / 2) as f32,
                (self.renderer.rasterizer.framebuffer.get_height() / 2) as f32,
            );
            let mut mouse_pos_relative_center = input_handler.get_mouse_position();
            mouse_pos_relative_center.x -=
                (self.renderer.rasterizer.framebuffer.get_width() / 2) as f32;
            mouse_pos_relative_center.y -=
                (self.renderer.rasterizer.framebuffer.get_height() / 2) as f32;
            let mouse_center_dist_vec = screen_center.add_p(mouse_pos_relative_center);

            let dp_point_start = ScreenPoint::new(screen_center.x as i32, screen_center.y as i32);
            let dp_point_end = ScreenPoint::new(
                mouse_center_dist_vec.x as i32,
                mouse_center_dist_vec.y as i32,
            );

            self.renderer
                .rasterizer
                .draw_line(dp_point_start, dp_point_end, ColorRGB::WHITE);
        }

        self.renderer.rasterizer.framebuffer.get_buffer()
    }
}
