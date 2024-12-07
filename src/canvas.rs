pub struct Canvas {
    pub buffer: Vec<u32>,
    pub canvas_height: usize,
    pub canvas_width: usize,
}

// Implement methods for the struct (similar to class methods)
impl Canvas {
    // Constructor (called an "associated function" in Rust)
    pub fn new(canvas_height: usize, canvas_width: usize) -> Canvas {
        let buffer = vec![0; canvas_width * canvas_height];
        Canvas {buffer, canvas_width, canvas_height}
    }

    pub fn flush(&mut self){
        for i in 0..self.buffer.len() {
            self.buffer[i] = 0xFFFFFF; 
        }
    }

    // Convert from x,y coordinates to buffer index
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.canvas_width + x
    }
    
    // get canvas dimensions
    pub fn get_dimensions(&self) -> (usize, usize){
        (self.canvas_height, self.canvas_width)
    }

    // Convert from buffer index to x,y coordinates
    pub fn get_coordinates(&self, index: usize) -> (usize, usize) {
        let x = index % self.canvas_width;
        let y = index / self.canvas_width;
        (x, y)
    }

    // Example: Set pixel at specific x,y coordinate
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = self.get_index(x, y);
        self.buffer[index] = color;
    }
}
