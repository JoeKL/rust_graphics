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
        "Image Viewer",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    render_engine: Engine,
    input_handler: InputHandler,
}

impl MyApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            render_engine: Engine::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
            input_handler: InputHandler::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let content_rect = ui.content_rect();

            let size: (f32, f32) = (content_rect.width(), content_rect.height());
            let frame = self.render_engine.run(&self.input_handler);
            let pixel = &split_rgba_slice(frame);

            ui.add(
                egui::Image::new(egui::load::SizedTexture::new(
                    &ui.load_texture(
                        "buffer",
                        egui::ColorImage::from_rgba_premultiplied(
                            [size.0 as usize, size.1 as usize],
                            pixel,
                        ),
                        Default::default(),
                    ),
                    Vec2::new(size.0, size.1),
                ))
                .fit_to_exact_size(Vec2::new(size.0, size.1)),
            );
        });
    }
}

pub fn get_max_quadratic_size(ui: &mut Ui) -> Vec2 {
    let size = ui.available_size();
    let min_size = if size.x < size.y { size.x } else { size.y };
    let size = Vec2::from([min_size, min_size]);
    size
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
