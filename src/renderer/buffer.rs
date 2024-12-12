use crate::types::color::ColorRGB;

pub struct FrameBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn get_buffer(&self) -> &[u32] {
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
        y * self.width + x
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
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
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
    pub fn set_pixel(&mut self, x: i32, y: i32, color: ColorRGB) {
        if self.is_in_bounds(x, y) {
            let index = self.get_index(x as usize, y as usize);
            self.buffer[index] = color.get_as_u32();
        }
    }

    pub fn fill(&mut self, color: ColorRGB) {
        self.buffer.fill(color.get_as_u32());
    }


}
