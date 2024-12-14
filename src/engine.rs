use crate::input::InputHandler;
use crate::renderer::Renderer;
use crate::scene::{Scene, SceneNode};
use crate::types::color::ColorRGB;
use crate::types::display::ScreenPoint;
use crate::types::math::{Mat4x4, Point2D, Vector3D};

pub struct Engine {
    renderer: Renderer,
    scene: Scene,
    frame: u32,
    augmentation_segment: i32,
    draw_axis: bool,
    draw_lights: bool,
    draw_ball_line: bool,
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

        let draw_axis = false;
        let draw_lights = false;
        let draw_ball_line = false;

        Engine {
            renderer,
            scene,
            frame,
            augmentation_segment,
            draw_axis,
            draw_lights,
            draw_ball_line,
        }
    }

    fn process_input(&mut self, input_handler: &InputHandler) {
        if input_handler.is_key_pressed(minifb::Key::Space) {
            self.augmentation_segment += 1;
            if self.augmentation_segment > 2 {
                self.augmentation_segment = 0;
            }
        }

        if input_handler.is_key_pressed(minifb::Key::K) {
            // toggles draw_axis
            self.draw_axis = !self.draw_axis;
        }

        if input_handler.is_key_pressed(minifb::Key::L) {
            //toggle draw_lights
            self.draw_lights = !self.draw_lights;
        }

        self.change_camera_fov(input_handler);
        self.rotate_lightsources(input_handler);

        self.rotate_ball_with_mouse(input_handler);
        self.move_ball(input_handler);
        self.iso_scale_ball(input_handler);
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

        let x_rot_delta = 0.1;
        let y_rot_delta = 0.1;

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

    fn rotate_ball_with_mouse(&mut self, input_handler: &InputHandler) {
        if input_handler.is_mouse_button_down(0) {
            let mut x_rot: f32 = 0.00;
            let mut y_rot: f32 = 0.00;

            let dist_center_threshhold = 50.0;

            let mut mouse_pos_relative_center = input_handler.get_mouse_position();
            mouse_pos_relative_center.x -= (self.renderer.get_window_width() / 2) as f32;
            mouse_pos_relative_center.y -= (self.renderer.get_window_height() / 2) as f32;

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

            self.draw_ball_line = true;
        } else {
            self.draw_ball_line = false
        }
    }

    fn move_ball(&mut self, input_handler: &InputHandler) {
        let mut x_move: f32 = 0.0;
        let mut z_move: f32 = 0.0;

        let x_move_delta = 0.1;
        let z_move_delta = 0.1;

        if input_handler.is_key_down(minifb::Key::Up) {
            z_move -= z_move_delta;
        }
        if input_handler.is_key_down(minifb::Key::Down) {
            z_move += z_move_delta;
        }

        if input_handler.is_key_down(minifb::Key::Left) {
            x_move += x_move_delta;
        }
        if input_handler.is_key_down(minifb::Key::Right) {
            x_move -= x_move_delta;
        }

        let focus_segment = match self.augmentation_segment {
            0 => &mut self.scene.root_node.children[0],
            1 => &mut self.scene.root_node.children[0].children[0],
            2 => &mut self.scene.root_node.children[0].children[0].children[0],
            _ => return, // Or handle other cases
        };

        if x_move != 0.0 || z_move != 0.0 {
            focus_segment.translate(Vector3D::new(x_move, 0.0, z_move));
        }
    }

    fn iso_scale_ball(&mut self, input_handler: &InputHandler) {
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

    pub fn run(&mut self, input_handler: &InputHandler) -> Vec<u32> {
        self.frame += 1;

        // Handle input
        self.process_input(input_handler);

        // Render
        self.renderer.render_scene(&mut self.scene);

        // Debug renders
        if self.draw_axis {
            self.renderer.render_axis(&mut self.scene);
        }
        if self.draw_lights {
            self.renderer.render_light_vectors(&mut self.scene);
        }
        if self.draw_ball_line {
            let screen_center = Point2D::new(
                (self.renderer.get_window_width() / 2) as f32,
                (self.renderer.get_window_height() / 2) as f32,
            );
            let mut mouse_pos_relative_center = input_handler.get_mouse_position();
            mouse_pos_relative_center.x -= (self.renderer.get_window_width() / 2) as f32;
            mouse_pos_relative_center.y -= (self.renderer.get_window_height() / 2) as f32;
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

        self.renderer.get_buffer()
    }
}
