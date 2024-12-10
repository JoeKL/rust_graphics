mod color;
mod displaybuffer;
mod primitives;
mod renderer;
mod obj_loader;
mod camera;

use displaybuffer::{DisplayBuffer, DisplayBufferPoint};
use minifb::{Key, Window, WindowOptions};

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

    let mut display_buffer = DisplayBuffer::new(WINDOW_HEIGHT, WINDOW_WIDTH);

    let mut step: u32 = 0; 

    // Update display
    renderer::update(&mut display_buffer, step);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            
            renderer::update(&mut display_buffer, step);
            step += 1;
            window
            .update_with_buffer(&display_buffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
        }
    }
}
