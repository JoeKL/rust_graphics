use eframe::CreationContext;

use crate::input::InputHandler;
use crate::math::Point3D;
use crate::renderer::{RenderTarget, Renderer, Viewport};
use crate::scene::Scene;

pub struct EngineApp {
    renderer: Renderer,

    scene: Scene,

    target: RenderTarget,
    viewport: Viewport,

    // egui: egui,
    frame_texture: Option<egui::TextureHandle>,

    pub orbit_yaw: f64,
    pub orbit_pitch: f64,
    pub fov_degrees: f64,

    pub draw_axis: bool,
    pub draw_grid: bool,
    pub draw_lights: bool,
}

impl EngineApp {
    pub fn new(_cc: &CreationContext, window_width: u32, window_height: u32) -> EngineApp {
        let renderer = Renderer::new();
        let mut scene = Scene::new();

        let far: f64 = 75.0;
        let near: f64 = 1.0;

        if let Some(camera) = scene.find_camera_mut() {
            camera.set_projection_params(
                30.0, // 60 degree FOV
                window_width as f64 / window_height as f64,
                near,
                far,
            );
        }

        let target = RenderTarget::new(window_width as usize, window_height as usize);
        let viewport = Viewport::new(window_width as usize, window_height as usize);

        let orbit_yaw = 180.0;
        let orbit_pitch = 15.0;
        let fov_degrees = 30.0;

        let draw_axis = true;
        let draw_grid = true;
        let draw_lights = false;

        EngineApp {
            renderer,
            scene,
            target,
            viewport,

            frame_texture: None,

            orbit_yaw,
            orbit_pitch,
            fov_degrees,

            draw_axis,
            draw_grid,
            draw_lights,
        }
    }

    // TODO should be done through scene manipulation
    fn update_camera(&mut self) {
        if let Some(camera) = self.scene.find_camera_mut() {
            let current_position = camera.get_position();

            let target = Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            };

            let distance = current_position.sub_p(target).length();

            self.orbit_pitch = self.orbit_pitch.clamp(-89.0, 89.0);

            let pitch_rad = self.orbit_pitch.to_radians();
            let yaw_rad = self.orbit_yaw.to_radians();

            let h_distance = distance * pitch_rad.cos();
            let x = h_distance * yaw_rad.sin();
            let y = distance * pitch_rad.sin();
            let z = h_distance * yaw_rad.cos();

            camera.set_position(Point3D { x, y, z, w: 1.0 });
            camera.set_fov_in_degrees(self.fov_degrees);

            camera.look_at(target);
        }
    }

    pub fn render_frame(&mut self, frame_width: usize, frame_height: usize) -> &[u8] {
        if self.target.framebuffer.get_width() != frame_width
            || self.target.framebuffer.get_height() != frame_height
        {
            self.target.resize(frame_width, frame_height);
            self.viewport = Viewport::new(frame_width, frame_height);
            if let Some(camera) = self.scene.find_camera_mut() {
                camera.set_projection_params(
                    camera.fov_in_degrees,
                    frame_width as f64 / frame_height as f64,
                    camera.near,
                    camera.far,
                );
            }
        }

        self.renderer
            .draw_background_on_framebuffer(&mut self.target);

        let camera = self.scene.get_active_camera();

        if self.draw_grid {
            self.renderer
                .render_grid(&self.scene, &mut self.target, &self.viewport, &camera);
        }

        // Render
        self.renderer
            .render_scene(&self.scene, &mut self.target, &self.viewport, &camera);

        // Debug renders
        if self.draw_axis {
            self.renderer
                .render_axis(&self.scene, &mut self.target, &self.viewport, &camera);
        }
        if self.draw_lights {
            self.renderer.render_light_vectors(
                &self.scene,
                &mut self.target,
                &self.viewport,
                &camera,
            );
        }

        self.target.framebuffer.get_buffer()
    }

    pub fn start() -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_maximized(true)
                .with_resizable(true)
                .with_fullscreen(false), // if set to true change the height of the window or else panic
            ..Default::default()
        };

        eframe::run_native(
            "Renderer",
            options,
            Box::new(|cc| Ok(Box::new(EngineApp::new(cc, 800, 600)))),
        )
    }
}

impl eframe::App for EngineApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Left Side Panel: Top-down projection view
        egui::Panel::right("view_panel")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.heading("Camera Controls");
                ui.add(egui::Slider::new(&mut self.orbit_yaw, 0.0..=360.0).text("Yaw"));
                ui.add(egui::Slider::new(&mut self.orbit_pitch, -89.0..=89.0).text("Pitch"));
                ui.add(egui::Slider::new(&mut self.fov_degrees, 1.0..=90.0).text("FOV"));
            });

        // Center Panel: Standard 3D perspective view
        egui::Panel::left("").show_inside(ui, |ui| {
            ui.heading("Debug Controls");
            ui.checkbox(&mut self.draw_axis, "draw_axis");
            ui.checkbox(&mut self.draw_grid, "draw_grid");
            ui.checkbox(&mut self.draw_lights, "draw_lights");
            ui.checkbox(&mut self.renderer.draw_wireframe, "draw_wireframe");
            ui.checkbox(&mut self.renderer.draw_z_buffer, "draw_z_buffer");
            ui.checkbox(&mut self.renderer.draw_vertex, "draw_vertex");
            ui.checkbox(
                &mut self.renderer.draw_vertex_normals,
                "draw_vertex_normals",
            );
            ui.checkbox(&mut self.renderer.draw_faces, "draw_faces");
            ui.checkbox(&mut self.renderer.backface_culling, "backface_culling");
        });

        {
            let size = ui.available_size();

            let raw_frame = self.render_frame(size.x as usize, size.y as usize);

            let image = egui::ColorImage::from_rgba_premultiplied(
                [size.x as usize, size.y as usize],
                &raw_frame,
            );

            let texture = self.frame_texture.get_or_insert_with(|| {
                ui.load_texture("render_buffer", image.clone(), egui::TextureOptions::LINEAR)
            });

            texture.set(image, egui::TextureOptions::LINEAR);

            egui::CentralPanel::default().show_inside(ui, |ui| {
                let available_size = ui.available_size();
                ui.image((texture.id(), available_size));
            });
        }

        self.update_camera();
        ui.request_repaint();
    }
}
