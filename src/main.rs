
mod types;
mod scene;
mod models;
mod engine;
mod input;
mod renderer;

use minifb::{Key, Window, WindowOptions};
use engine::Engine;
use input::InputHandler;
use std::time::Instant;

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

    // window.set_target_fps(60);

    let mut input_handler = InputHandler::new();

    let mut render_engine = Engine::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    let mut time_since_title_update = Instant::now();
    let mut frame_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        //increase frames_count
        frame_count += 1;

        if time_since_title_update.elapsed().as_secs() >= 1 {
            //set title to how many frames were generated since last check
            window.set_title(&format!("Rust Graphics - FPS: {}", frame_count));
            //reset frame_count
            frame_count = 0;
            //reset time since last update
            time_since_title_update = Instant::now();
        }

        input_handler.update(&window);

        window
            .update_with_buffer(&render_engine.run(&input_handler), WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}
