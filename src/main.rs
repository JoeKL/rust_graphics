mod engine;
mod input;
mod math;
mod renderer;
mod scene;
mod utils;

use eframe::{egui, CreationContext};
use egui::{Ui, Vec2};
use engine::Engine;
use input::InputHandler;

const WINDOW_WIDTH: usize = 1920;
const WINDOW_HEIGHT: usize = 1008;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1008.0])
            .with_maximized(true)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Renderer",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    render_engine: Engine,
    input_handler: InputHandler,
    frame_texture: Option<egui::TextureHandle>,
}

impl MyApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            render_engine: Engine::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
            input_handler: InputHandler::new(),
            frame_texture: None,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let content_rect = ui.content_rect();
        let size: (f32, f32) = (content_rect.width(), content_rect.height());

        let raw_frame = self.render_engine.run(&self.input_handler);
        let rgba_pixels = split_rgba_slice(raw_frame);

        let image = egui::ColorImage::from_rgba_premultiplied(
            [size.0 as usize, size.1 as usize],
            &rgba_pixels,
        );

        let texture = self.frame_texture.get_or_insert_with(|| {
            ui.load_texture("render_buffer", image.clone(), egui::TextureOptions::LINEAR)
        });

        texture.set(image, egui::TextureOptions::LINEAR);

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let available_size = ui.available_size();
            ui.image((texture.id(), available_size));
        });

        ui.request_repaint();
    }
}

fn split_rgba_slice(colors: &[u32]) -> Vec<u8> {
    let mut result = Vec::with_capacity(colors.len() * 4);

    for &color in colors {
        let bytes = color.to_ne_bytes();

        result.push(bytes[2]); // Extract Red
        result.push(bytes[1]); // Extract Green
        result.push(bytes[0]); // Extract Blue
        result.push(255); // Force Alpha to 255 (Opaque)
    }

    result
}
