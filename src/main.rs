mod displaybuffer;
mod primitives;
mod color;

use color::ColorRGB;
use displaybuffer::{DisplayBuffer, DisplayBufferPoint};
use minifb::{Key, Window, WindowOptions};

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 800;

fn main() {
    
    let mut window = Window::new(
        "Rust Graphics",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap();
    
    let mut display_buffer = DisplayBuffer::new(WINDOW_HEIGHT, WINDOW_WIDTH);

    display_buffer.fill(ColorRGB::BLACK);

    let p0 = DisplayBufferPoint {x: 75, y: WINDOW_HEIGHT as i32 - 100};
    let p1 = DisplayBufferPoint {x: WINDOW_WIDTH as i32/2, y: 100};
    let p2 = DisplayBufferPoint { x: WINDOW_WIDTH as i32 - 75,y: WINDOW_HEIGHT as i32 - 100 };

    // display_buffer.draw_line(p0, p1, DisplayBuffer::MAGENTA);
    // display_buffer.draw_line(p1, p2, DisplayBuffer::MAGENTA);
    // display_buffer.draw_line(p0, p2, DisplayBuffer::MAGENTA);
    
    display_buffer.draw_gradient_triangle(p0, p1, p2, ColorRGB::RED, ColorRGB::BLUE, ColorRGB::GREEN);
    
    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.update_with_buffer(&display_buffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    }
}