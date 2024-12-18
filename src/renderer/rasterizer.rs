use crate::renderer::{FrameBuffer, Viewport};
use crate::types::color::ColorRGB;
use crate::types::display::ScreenPoint;

pub struct Rasterizer {
    pub framebuffer: FrameBuffer,
    pub viewport: Viewport,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            framebuffer: FrameBuffer::new(width, height),
            viewport: Viewport::new(width, height),
        }
    }
    /// Draws a line between two points using bresenham.
    ///
    /// ### Arguments
    ///
    /// * `p0` - Starting point of the line
    /// * `p1` - Ending point of the line
    /// * `color` - Color value to draw the line with (32-bit RGB/RGBA)
    ///
    /// 
    /// ### Example
    ///
    /// ```
    /// let mut buffer = DisplayBuffer::new(100, 100);
    /// let start = DisplayBufferPoint::new(10, 10);
    /// let end = DisplayBufferPoint::new(50, 30);
    /// buffer.draw_line(start, end, 0xFF0000); // Draws a red line
    /// ```
    ///
    /// ### Notes
    ///
    /// * The points are taken as mutable because they may be swapped internally
    /// * Works with both shallow and steep line angles
    pub fn draw_line(
        &mut self,
        mut p0: ScreenPoint,
        mut p1: ScreenPoint,
        color: ColorRGB,
    ) {
        // Handle vertical lines specially
        if p1.x == p0.x {
            let (start_y, end_y) = if p0.y > p1.y {
                //when p0 is further down then p1
                (p1.y, p0.y)
            } else {
                //when p1 is further down then p0
                (p0.y, p1.y)
            };
            //draw frambuffer up down
            for y in start_y..=end_y {
                self.framebuffer.set_pixel(p0.x, y, color);
            }
            return;
        }

        // Handle horizontal lines specially
        if p1.y == p0.y {
            let (start_x, end_x) = if p0.x > p1.x {
                //when p0 is further right then p1
                (p1.x, p0.x)
            } else {
                //when p1 is further right then p0
                (p0.x, p1.x)
            };
            //draw frambuffer up down
            for x in start_x..=end_x {
                self.framebuffer.set_pixel(x, p0.y, color);
            }
            return;
        }

        // Ensure we're always drawing left to right
        if p1.x < p0.x {
            std::mem::swap(&mut p0, &mut p1);
        }

        let slope_m = (p1.y - p0.y) as f32 / (p1.x - p0.x) as f32;
        let steep = slope_m.abs() > 1.0;
        
        if steep {
            // Swap x and y coordinates if slope is steep
            std::mem::swap(&mut p0.x, &mut p0.y);
            std::mem::swap(&mut p1.x, &mut p1.y);
        }

        // Ensure left-to-right again after possible x/y swap
        if p1.x < p0.x {
            std::mem::swap(&mut p0, &mut p1);
        }

        let slope_m = (p1.y - p0.y) as f32 / (p1.x - p0.x) as f32;
        let t = p0.y as f32 - (slope_m * p0.x as f32);

        for x in p0.x..=p1.x {
            let pixel_y = (slope_m * x as f32 + t).round() as i32;
            if steep {
                self.framebuffer.set_pixel(pixel_y, x, color);
            } else {
                self.framebuffer.set_pixel(x, pixel_y, color);
            }
        }
    }

}
