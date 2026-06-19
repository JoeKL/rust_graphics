use crate::renderer::color::ColorRGB;

pub struct FrameBuffer {
    pub buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            // buffer is multiplied by 4 since every frame is composed of R, G, B, A
            buffer: vec![0; width * height * 4],
            width,
            height,
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
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
        (y * self.width) * 4 + x * 4
    }

    /// get coordiantes from index as usize
    ///
    /// # Arguments
    /// * index as usize
    ///
    /// # Returns
    /// coordinates as (usize, usize)
    pub fn get_coordinates(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
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
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Sets a Pixel to a specified color
    ///
    /// # Arguments
    /// * `x` - The x coordinate
    /// * `y` - The x coordinate
    /// * `color` - The color
    ///
    pub fn set_pixel(&mut self, x: usize, y: usize, color: ColorRGB) {
        if self.is_in_bounds(x, y) {
            let index = self.get_index(x, y);

            self.buffer[index] = color.get_r();
            self.buffer[index + 1] = color.get_g();
            self.buffer[index + 2] = color.get_b();
            self.buffer[index + 3] = color.get_a();
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        if self.is_in_bounds(x, y) {
            let index = self.get_index(x, y);
            self.buffer[index]
        } else {
            0
        }
    }

    pub fn fill(&mut self, color: ColorRGB) {
        for pixel in self.buffer.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color.as_argb_u8_slice());
        }
    }
}
