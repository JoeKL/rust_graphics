use crate::renderer::{FrameBuffer, Viewport, ColorRGB};
use crate::math::ScreenPoint;
use crate::scene::Vertex;

//  To avoid potential confusion, let me define "rasterization":
//  For our present purposes, it's the process of determining which pixels are inside a triangle, and nothing more.
//
//  - Michael Abrash, 2009.

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
    pub fn draw_line(&mut self, p0: ScreenPoint, p1: ScreenPoint, color: ColorRGB) {
        let framebuffer = &mut self.framebuffer;
        Self::for_each_line_point_impl(p0, p1, |x, y| {
            framebuffer.set_pixel(x, y, color);
        });
    }

    /// Evaluates all screen-space points along a line using Bresenham's algorithm,
    /// invoking a closure for each pixel to avoid heap allocations.
    pub fn for_each_line_point<F>(&self, p0: ScreenPoint, p1: ScreenPoint, f: F)
    where
        F: FnMut(i32, i32),
    {
        Self::for_each_line_point_impl(p0, p1, f);
    }

    fn for_each_line_point_impl<F>(mut p0: ScreenPoint, mut p1: ScreenPoint, mut f: F)
    where
        F: FnMut(i32, i32),
    {
        // Handle vertical lines specially
        if p1.x == p0.x {
            let (start_y, end_y) = if p0.y > p1.y {
                (p1.y, p0.y)
            } else {
                (p0.y, p1.y)
            };
            for y in start_y..=end_y {
                f(p0.x, y);
            }
            return;
        }

        // Handle horizontal lines specially
        if p1.y == p0.y {
            let (start_x, end_x) = if p0.x > p1.x {
                (p1.x, p0.x)
            } else {
                (p0.x, p1.x)
            };
            for x in start_x..=end_x {
                f(x, p0.y);
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
                f(pixel_y, x);
            } else {
                f(x, pixel_y);
            }
        }
    }

    pub fn calculate_bounding_box(
        &self,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
    ) -> (i32, i32, i32, i32) {
        // Triangle setup (bounding box)
        //calculate bounding box
        // 50.min(60).min(40) -> 50.min(40) -> 40
        let mut bounds_min_x = v0.position[0]
            .min(v1.position[0])
            .min(v2.position[0])
            .floor() as i32;
        let mut bounds_max_x = v0.position[0]
            .max(v1.position[0])
            .max(v2.position[0])
            .ceil() as i32;
        let mut bounds_min_y = v0.position[1]
            .min(v1.position[1])
            .min(v2.position[1])
            .floor() as i32;
        let mut bounds_max_y = v0.position[1]
            .max(v1.position[1])
            .max(v2.position[1])
            .ceil() as i32;

        // Clamp to screen boundaries before the loops
        bounds_min_x = bounds_min_x.max(0);
        bounds_max_x = bounds_max_x.min(self.framebuffer.get_width() as i32);

        bounds_min_y = bounds_min_y.max(0);
        bounds_max_y = bounds_max_y.min(self.framebuffer.get_height() as i32);

        (bounds_min_x, bounds_min_y, bounds_max_x, bounds_max_y)
    }

    /// takes 3 Points and checks of all of them are on screen
    ///
    ///
    pub fn is_triangle_on_screen(&self, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> bool {
        // this returns true when one of the vertices is on screen
        // and false if all are off
        // cool effect if changed to &&. it then only draws if ALL of them are on screen
        self.framebuffer
            .is_in_bounds(v0.position[0] as i32, v0.position[1] as i32)
            || self
                .framebuffer
                .is_in_bounds(v1.position[0] as i32, v1.position[1] as i32)
            || self
                .framebuffer
                .is_in_bounds(v2.position[0] as i32, v2.position[1] as i32)
    }

    pub fn calculate_barycentric(
        x: f32,
        y: f32,
        v0: &[f32; 2],
        v1: &[f32; 2],
        v2: &[f32; 2],
    ) -> (f32, f32, f32) {
        // Compute vectors
        let v0_to_v1 = [v1[0] - v0[0], v1[1] - v0[1]];
        let v0_to_v2 = [v2[0] - v0[0], v2[1] - v0[1]];

        // Compute denominator once
        let denominator = v0_to_v1[0] * v0_to_v2[1] - v0_to_v2[0] * v0_to_v1[1];

        // Point to v0 vector
        let p_to_v0 = [x - v0[0], y - v0[1]];

        // Calculate barycentric coordinates
        let beta = (p_to_v0[0] * v0_to_v2[1] - v0_to_v2[0] * p_to_v0[1]) / denominator;
        let gamma = (v0_to_v1[0] * p_to_v0[1] - p_to_v0[0] * v0_to_v1[1]) / denominator;
        let alpha = 1.0 - beta - gamma;

        (alpha, beta, gamma)
    }
}
