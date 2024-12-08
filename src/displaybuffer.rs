#![allow(dead_code)]

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
    pub const BLACK: u32 = 0x000000;
    pub const WHITE: u32 = 0xFFFFFF;
    pub const RED: u32 = 0xFF0000;
    pub const GREEN: u32 = 0x00FF00;
    pub const BLUE: u32 = 0x0000FF;
    pub const YELLOW: u32 = 0xFFFF00;
    pub const CYAN: u32 = 0x00FFFF;
    pub const MAGENTA: u32 = 0xFF00FF;

    pub fn new(canvas_height: usize, canvas_width: usize) -> DisplayBuffer {
        let buffer = vec![0; canvas_width * canvas_height];
        DisplayBuffer {
            buffer,
            canvas_width,
            canvas_height,
        }
    }

    /// Ssets the whole background to specified color
    ///
    /// # Arguments
    /// * `color` - the color as u32
    pub fn fill(&mut self, color: u32) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = color;
        }
    }

    /// Converts 2D coordinates (x, y) to a buffer index
    ///
    /// # Arguments
    /// * `x` - The x coordinate
    /// * `y` - The y coordinate
    ///
    /// # Returns
    /// The corresponding buffer index
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.canvas_width + x
    }

    /// Returns the dimensions of the DisplayBuffer
    ///
    /// # Returns
    /// dimensions as (usize, usize)
    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.canvas_height, self.canvas_width)
    }

    /// get coordiantes from index as usize
    ///
    /// # Arguments
    /// * index as usize
    ///
    /// # Returns
    /// coordinates as (usize, usize)
    pub fn get_coordinates(&self, index: usize) -> (usize, usize) {
        let x = index % self.canvas_width;
        let y = index / self.canvas_width;
        (x, y)
    }

    /// Sets a Pixel to a specified color
    ///
    /// # Arguments
    /// * `x` - The x coordinate
    /// * `y` - The x coordinate
    /// * `color` - The color
    ///
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 || x > self.canvas_width as i32 || y > self.canvas_height as i32 {
            return;
        }
        let index = self.get_index(x as usize, y as usize);
        self.buffer[index] = color;
    }

    /// Checks if the given coordinates are within the display buffer bounds.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate to check
    /// * `y` - The y coordinate to check
    ///
    /// # Returns
    ///
    /// `true` if the coordinates are within bounds, `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// let buffer = DisplayBuffer::new(100, 100);
    /// assert!(buffer.is_in_bounds(50, 50));  // Inside bounds
    /// assert!(!buffer.is_in_bounds(-1, 50)); // Outside bounds (negative)
    /// assert!(!buffer.is_in_bounds(100, 50)); // Outside bounds (too large)
    /// ```
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.canvas_width as i32 && y < self.canvas_height as i32
    }

    /// Performs linear interpolation between two points.
    ///
    /// # Arguments
    ///
    /// * `i0` - Starting index/position (e.g., starting x or y coordinate)
    /// * `d0` - Starting value at position i0 (e.g., the corresponding y or x value)
    /// * `i1` - Ending index/position
    /// * `d1` - Ending value at position i1
    ///
    /// # Returns
    ///
    /// A vector of interpolated values as f32, with length equal to abs(i1 - i0).
    /// Each element represents the interpolated value at the corresponding position.
    ///
    /// # Example
    ///
    /// ```
    /// // Interpolate y values for x coordinates 0 to 5, from y=10 to y=20
    /// let interpolated = linear_interpolation(0, 10, 5, 20);
    /// // Returns: [10.0, 12.0, 14.0, 16.0, 18.0, 20.0]
    /// ```
    ///
    /// # Notes
    ///
    /// * If i0 == i1, returns a vector with single value d0
    /// * The function interpolates in the direction from i0 to i1
    /// * Useful for line rasterization where you need to find all points between two endpoints
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
        let a: f32 = (d1 - d0) as f32 / (i1 - i0) as f32;

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

    /// Draws a line between two points using linear interpolation.
    ///
    /// # Arguments
    ///
    /// * `p0` - Starting point of the line
    /// * `p1` - Ending point of the line
    /// * `color` - Color value to draw the line with (32-bit RGB/RGBA)
    ///
    /// # Details
    ///
    /// The algorithm determines whether the line is more horizontal or vertical and chooses
    /// the appropriate axis to iterate over. For each step along the major axis, it calculates
    /// the corresponding coordinate on the minor axis using linear interpolation.
    ///
    /// The points are automatically sorted so that drawing always proceeds from left to right
    /// (for more horizontal lines) or top to bottom (for more vertical lines).
    ///
    /// # Example
    ///
    /// ```
    /// let mut buffer = DisplayBuffer::new(100, 100);
    /// let start = DisplayBufferPoint::new(10, 10);
    /// let end = DisplayBufferPoint::new(50, 30);
    /// buffer.draw_line(start, end, 0xFF0000); // Draws a red line
    /// ```
    ///
    /// # Notes
    ///
    /// * The points are taken as mutable because they may be swapped internally
    /// * Uses linear interpolation rather than Bresenham's algorithm
    /// * Works with both shallow and steep line angles
    pub fn draw_line(
        // TODO Bresenham
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
                self.set_pixel(x, result[i] as i32, color);
                i += 1;
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
                self.set_pixel(result[i] as i32, y, color);
                i += 1;
            }
        }
    }

    pub fn draw_triangle(
        &mut self,
        mut p0: DisplayBufferPoint,
        mut p1: DisplayBufferPoint,
        mut p2: DisplayBufferPoint,
        color: u32,
    ) {
        // sort the y points such that y0 < y1 < y2
        if p1.y < p0.y {
            let temp: DisplayBufferPoint = p0;
            p0 = p1;
            p1 = temp;
        }
        if p2.y < p0.y {
            let temp: DisplayBufferPoint = p0;
            p0 = p2;
            p2 = temp;
        }
        if p2.y < p1.y {
            let temp: DisplayBufferPoint = p2;
            p2 = p1;
            p1 = temp;
        }

        // calculate boundaries of the triangle given by p0,p1,p2
        // we want the x values for each line between two points, thats why the independent value is y. y = i , x = d
        // naming: x01 -> x values between p0 and p1
        let mut x01 = DisplayBuffer::linear_interpolation(p0.y, p0.x, p1.y, p1.x);
        let x02 = DisplayBuffer::linear_interpolation(p0.y, p0.x, p2.y, p2.x);
        let x12 = DisplayBuffer::linear_interpolation(p1.y, p1.x, p2.y, p2.x);

        //pop the last element so that its not counted twice, since its the first in x12
        x01.pop();
        let mut x012 = x01.clone(); // Create new vector as copy of x01
        x012.extend(x12); // append x12 to x01 to create x012

        // create left and right wall as x_left and x_right
        let x_left: Vec<f32>;
        let x_right: Vec<f32>;

        // check which wall is left and which is right
        // only check the middle since x012 is the wall bend
        let m = x012.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            x_right = x012;
        } else {
            x_left = x012;
            x_right = x02;
        }

        // for every row from the left wall+1 to the right wall -1 set pixel to color
        for y in (p0.y)..(p2.y) {
            let current_row = (y - p0.y) as usize;
            let x_start = x_left[current_row] as i32;
            let x_end = x_right[current_row] as i32;

            // Fill pixels for current scanline (excluding edges)
            for x in (x_start + 1)..x_end {
                self.set_pixel(x, y, color);
            }
        }
    }
}
