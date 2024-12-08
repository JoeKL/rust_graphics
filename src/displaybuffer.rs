#[derive(Debug, Clone, Copy)]
pub struct DisplayBufferPoint {
    pub x: i32,
    pub y: i32,
}

impl DisplayBufferPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct DisplayBuffer {
    pub buffer: Vec<u32>,
    pub canvas_height: usize,
    pub canvas_width: usize,
}

// Implement methods for the struct (similar to class methods)
impl DisplayBuffer {
    // Constructor (called an "associated function" in Rust)
    pub fn new(canvas_height: usize, canvas_width: usize) -> DisplayBuffer {
        let buffer = vec![0; canvas_width * canvas_height];
        DisplayBuffer {
            buffer,
            canvas_width,
            canvas_height,
        }
    }

    pub fn set_color(&mut self, color: u32) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = color;
        }
    }

    // Convert from x,y coordinates to buffer index
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.canvas_width + x
    }

    // get canvas dimensions
    pub fn get_dimensions(&self) -> (usize, usize) {
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

    fn linear_interpolation(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::new();

        // Special case: if points are the same, return array of size 1
        if i0 == i1 {
            // just use the same d for each i
            //(when horizontal use same height or vice versa)
            result.push(d0 as f32);
            return result;
        }

        // calculate delta
        let a: f32 = ((d1 - d0)) as f32 / ((i1 - i0)) as f32;


        // Calculate number of steps needed
        let steps = (i1 - i0).abs();

        // save starting point
        let mut d: f32 = d0 as f32;

        // Reserve space for efficiency
        result.reserve(steps as usize);

        for _ in 0..steps {
            // save d
            result.push(d);
            // with each iteration add another delta to d
            d = d + a;
        }

        return result;
    }

    // draws line between two points (x0,y0) and (x1,y1) with given color by using setPixel
    pub fn draw_line(
        &mut self,
        mut p0: DisplayBufferPoint,
        mut p1: DisplayBufferPoint,
        color: u32,
    ) {
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            // line is more horizontal then vertical
            // -> this must be true: x0 < x1

            if p0.x > p1.x {
                let temp: DisplayBufferPoint = p0;
                p0 = p1;
                p1 = temp;
            }

            // calculate the corrosponding y for each x
            let result = DisplayBuffer::linear_interpolation(p0.x, p0.y, p1.x, p1.y);

            // draw line by iterating through the results
            let mut i = 0;
            for x in p0.x..p1.x {
                {
                    self.set_pixel(x as usize, result[i] as usize, color);
                    i += 1;
                }
            }
        } else {
            // line is more vertical than horizontal
            // -> this must be true: y0 < y1
            if p0.y > p1.y {
                let temp: DisplayBufferPoint = p0;
                p0 = p1;
                p1 = temp;
            }

            // calculate the corrosponding x for each y
            let result = DisplayBuffer::linear_interpolation(p0.y, p0.x, p1.y, p1.x);

            // draw line by iterating through the results
            let mut i = 0;
            for y in p0.y..p1.y {
                self.set_pixel(result[i] as usize, y as usize, color);
                i += 1;
            }
        }
    }
}
