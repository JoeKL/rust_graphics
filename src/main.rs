mod camera;
mod color;
mod displaybuffer;
mod light_source;
mod obj_loader;
mod primitives;
mod renderer;
mod scene;

use displaybuffer::{DisplayBuffer, DisplayBufferPoint};
use minifb::{Key, Window, WindowOptions};
use renderer::RenderEngine;

static WINDOW_WIDTH: usize = 1200;
static WINDOW_HEIGHT: usize = 800;

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

    let mut render_engine = RenderEngine::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    let mut display_buffer = render_engine.render_frame();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((_x, _y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            display_buffer = render_engine.render_frame();

            window
                .update_with_buffer(&display_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
                .unwrap();
        }
    }
    print!("end");
}
