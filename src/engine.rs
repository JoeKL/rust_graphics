use eframe::CreationContext;
use egui::Key;

use crate::math::Point3D;
use crate::renderer::{RenderView, Renderer};
use crate::scene::Scene;

pub struct EngineApp {
    renderer: Renderer,
    scene: Scene,

    views: Vec<RenderView>,

    show_panels: bool,

    pub orbit_yaw: f64,
    pub orbit_pitch: f64,
    pub fov_degrees: f64,

    pub draw_axis: bool,
    pub draw_grid: bool,
    pub draw_lights: bool,
}

impl EngineApp {
    pub fn new(_cc: &CreationContext, window_width: usize, window_height: usize) -> EngineApp {
        let renderer = Renderer::new();
        let scene = Scene::new();

        let views = vec![
            RenderView::new("main", "main_camera", window_width, window_height),
            RenderView::new("secondary", "secondary_camera", window_width, window_height),
        ];

        let orbit_yaw = 180.0;
        let orbit_pitch = 15.0;
        let fov_degrees = 15.0;

        let draw_axis = true;
        let draw_grid = true;
        let draw_lights = false;

        EngineApp {
            renderer,
            scene,
            views,

            show_panels: true,

            orbit_yaw,
            orbit_pitch,
            fov_degrees,

            draw_axis,
            draw_grid,
            draw_lights,
        }
    }

    // TODO should be done through scene manipulation
    fn update_camera(&mut self, camera_node_name: &str) {
        if let Some(camera) = self.scene.find_camera_mut(camera_node_name) {
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

    pub fn start() -> eframe::Result {
        let options = eframe::NativeOptions {
            renderer: eframe::Renderer::Glow,
            vsync: false,
            viewport: egui::ViewportBuilder::default()
                .with_maximized(true)
                .with_resizable(true)
                .with_fullscreen(false),
            ..Default::default()
        };

        eframe::run_native(
            "Renderer",
            options,
            Box::new(|cc| Ok(Box::new(EngineApp::new(cc, 800, 600)))),
        )
    }

    pub fn _views(&self) -> &[RenderView] {
        &self.views
    }
}

impl eframe::App for EngineApp {
    // TODO decouple UI from frame rendering
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let orbit_step = 1.5;

        if ui.input(|i| i.key_down(Key::ArrowLeft)) {
            self.orbit_yaw = self.orbit_yaw - orbit_step;
        }
        if ui.input(|i| i.key_down(Key::ArrowRight)) {
            self.orbit_yaw = self.orbit_yaw + orbit_step;
        }

        if self.orbit_yaw >= 360.0 {
            self.orbit_yaw -= 360.0;
        }
        if self.orbit_yaw <= 0.0 {
            self.orbit_yaw += 360.0;
        }

        if ui.input(|i| i.key_down(Key::ArrowUp)) {
            self.orbit_pitch += orbit_step;
        }
        if ui.input(|i| i.key_down(Key::ArrowDown)) {
            self.orbit_pitch -= orbit_step;
        }
        if ui.input(|i| i.key_pressed(Key::F1)) {
            self.show_panels = !self.show_panels;
        }

        if ui.input(|i| i.key_down(Key::O)) {
            self.fov_degrees -= 0.5;
        }
        if ui.input(|i| i.key_down(Key::P)) {
            self.fov_degrees += 0.5;
        }

        // Left Side Panel: Top-down projection view
        if self.show_panels {
            egui::Panel::right("view_panel")
                .resizable(true)
                .show_inside(ui, |ui| {
                    ui.heading("Camera Controls");
                    ui.add(
                        egui::Slider::new(&mut self.orbit_yaw, 0.0..=360.0)
                            .text("Yaw")
                            .clamping(egui::SliderClamping::Never),
                    );
                    ui.add(egui::Slider::new(&mut self.orbit_pitch, -89.0..=89.0).text("Pitch"));
                    ui.add(egui::Slider::new(&mut self.fov_degrees, 1.0..=90.0).text("FOV"));
                });

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
        }

        let available_height = ui.available_height();
        let half_height = available_height / 2.0;

        ui.vertical(|ui| {
            // Top Camera Area
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), half_height),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    self.show_view(ui, 0);
                },
            );

            // Bottom Camera Area
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), half_height),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    self.show_view(ui, 1);
                },
            );
        });

        self.update_camera("main_camera");
        ui.request_repaint();
    }
}

impl EngineApp {
    fn show_view(&mut self, ui: &mut egui::Ui, view_idx: usize) {
        let view = &mut self.views[view_idx];
        let available_size = ui.available_size();
        let width = available_size.x as usize;
        let height = available_size.y as usize;

        // Resize the viewport buffers if egui panel resizes
        if view.viewport.get_width() != width || view.viewport.get_height() != height {
            view.resize(width, height);
            if let Some(camera) = self.scene.find_camera_mut(&view.camera_node_name) {
                camera.set_projection_params(
                    camera.fov_in_degrees,
                    width as f64 / height as f64,
                    camera.near,
                    camera.far,
                );
            }
        }

        self.renderer
            .draw_background_on_framebuffer(&mut view.target);

        let camera = self
            .scene
            .get_camera_by_name(&view.camera_node_name)
            .expect("no camera node with that name found");

        if self.draw_grid {
            self.renderer.render_grid(&self.scene, view, &camera);
        }

        // Render scene to this view's RenderTarget
        self.renderer.render_view(&self.scene, view, &camera);

        // Debug renders
        if self.draw_axis {
            self.renderer.render_axis(&self.scene, view, &camera);
        }
        if self.draw_lights {
            self.renderer
                .render_light_vectors(&self.scene, view, &camera);
        }

        // Upload framebuffer to egui texture
        let raw_pixels = view.target.framebuffer.get_buffer();
        let image = egui::ColorImage::from_rgba_premultiplied([width, height], raw_pixels);

        let texture = view.texture_handle.get_or_insert_with(|| {
            ui.ctx()
                .load_texture(&view.name, image.clone(), egui::TextureOptions::LINEAR)
        });
        texture.set(image, egui::TextureOptions::LINEAR);

        // Display image widget in egui
        ui.image((texture.id(), available_size));
    }
}
