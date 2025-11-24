use crate::renderer::{FrameBuffer, Viewport};
use crate::types::color::ColorRGB;
use crate::types::display::ScreenPoint;
use crate::types::primitives::Vertex;

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
    pub fn draw_line(&mut self, mut p0: ScreenPoint, mut p1: ScreenPoint, color: ColorRGB) {
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

    pub fn calculate_line(&mut self, v0: [i32; 2], v1: [i32; 2]) -> Vec<[i32; 2]> {
        let mut pixel_array: Vec<[i32; 2]> = Vec::new();

        let mut p0 = ScreenPoint::new(v0[0], v0[1]);
        let mut p1 = ScreenPoint::new(v1[0], v1[1]);

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
                pixel_array.push([p0.x, y]);
            }
            return pixel_array;
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
                pixel_array.push([x, p0.y]);
            }
            return pixel_array;
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
                pixel_array.push([pixel_y, x]);
            } else {
                pixel_array.push([x, pixel_y]);
            }
        }
        return pixel_array;
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
