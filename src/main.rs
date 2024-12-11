mod camera;
mod color;
mod displaybuffer;
mod light_source;
mod obj_loader;
mod primitives;
mod engine;
mod scene;
mod mesh;
mod inputhandler;

use displaybuffer::{DisplayBuffer, DisplayBufferPoint};
use minifb::{Key, Window, WindowOptions};
use engine::Engine;
use inputhandler::InputHandler;

static WINDOW_WIDTH: usize = 1280;
static WINDOW_HEIGHT: usize = 720;

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

    window.set_target_fps(60);

    let mut input_handler = InputHandler::new();

    let mut render_engine = Engine::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    let mut display_buffer ;


    while window.is_open() && !window.is_key_down(Key::Escape) {

        input_handler.update(&window);
        
        display_buffer = render_engine.render_frame(&input_handler);

        window
            .update_with_buffer(&display_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

    }
}
