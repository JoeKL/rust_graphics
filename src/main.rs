mod engine;
mod input;
mod renderer;
mod scene;
mod types;
mod utils;

use engine::Engine;
use input::InputHandler;
use minifb::{Key, Window, WindowOptions};

static WINDOW_WIDTH: usize = 1920;
static WINDOW_HEIGHT: usize = 1080;

fn main() {
    let mut window = Window::new(
        "Rust Graphics",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            borderless: false, // Remove window borders
            resize: false,     // Allow window resizing

            ..WindowOptions::default()
        },
    )
    .unwrap();

    // window.set_target_fps(60);

    let mut input_handler = InputHandler::new();

    let mut render_engine = Engine::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        input_handler.update(&window);

        window
            .update_with_buffer(
                render_engine.run(&input_handler),
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
            .unwrap();
    }
}
