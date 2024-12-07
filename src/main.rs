use minifb::{Key, Window, WindowOptions};
mod canvas;
use canvas::Canvas;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HIEGHT: usize = 800;

fn main() {
    
    let mut window = Window::new(
        "Rust Graphics",
        WINDOW_WIDTH,
        WINDOW_HIEGHT,
        WindowOptions::default(),
    ).unwrap();
    
    let mut canvas = Canvas::new(WINDOW_HIEGHT, WINDOW_WIDTH);

    println!("{:?}", canvas.get_dimensions());

    canvas.flush();
    
    for i in 0..WINDOW_WIDTH {
        
        canvas.set_pixel(i, i, 0);
    }


    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.update_with_buffer(&canvas.buffer, WINDOW_WIDTH, WINDOW_HIEGHT).unwrap();
    }
}