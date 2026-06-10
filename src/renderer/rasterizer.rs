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

    fn for_each_line_point_impl<F>(p0: ScreenPoint, p1: ScreenPoint, mut f: F)
    where
        F: FnMut(i32, i32),
    {
        let dx = (p1.x - p0.x).abs();
        let dy = -(p1.y - p0.y).abs();
        let sx = if p0.x < p1.x { 1 } else { -1 };
        let sy = if p0.y < p1.y { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = p0.x;
        let mut y = p0.y;

        loop {
            f(x, y);
            if x == p1.x && y == p1.y {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_line() {
        let p0 = ScreenPoint::new(0, 0);
        let p1 = ScreenPoint::new(5, 0);
        let mut points = Vec::new();
        Rasterizer::for_each_line_point_impl(p0, p1, |x, y| points.push((x, y)));
        assert_eq!(points, vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]);
    }

    #[test]
    fn test_vertical_line() {
        let p0 = ScreenPoint::new(0, 0);
        let p1 = ScreenPoint::new(0, 5);
        let mut points = Vec::new();
        Rasterizer::for_each_line_point_impl(p0, p1, |x, y| points.push((x, y)));
        assert_eq!(points, vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]);
    }

    #[test]
    fn test_diagonal_line() {
        let p0 = ScreenPoint::new(0, 0);
        let p1 = ScreenPoint::new(3, 3);
        let mut points = Vec::new();
        Rasterizer::for_each_line_point_impl(p0, p1, |x, y| points.push((x, y)));
        assert_eq!(points, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn test_steep_line() {
        let p0 = ScreenPoint::new(0, 0);
        let p1 = ScreenPoint::new(2, 5);
        let mut points = Vec::new();
        Rasterizer::for_each_line_point_impl(p0, p1, |x, y| points.push((x, y)));
        assert_eq!(points, vec![(0, 0), (0, 1), (1, 2), (1, 3), (2, 4), (2, 5)]);
    }

    #[test]
    fn test_single_point() {
        let p0 = ScreenPoint::new(2, 2);
        let p1 = ScreenPoint::new(2, 2);
        let mut points = Vec::new();
        Rasterizer::for_each_line_point_impl(p0, p1, |x, y| points.push((x, y)));
        assert_eq!(points, vec![(2, 2)]);
    }
}
