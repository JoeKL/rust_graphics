use std::mem::swap;

use crate::renderer::{FrameBuffer, Viewport};
use crate::types::color::ColorRGB;
use crate::types::display::ScreenPoint;
use crate::types::light::PointLight;
use crate::types::math::{Point3D, Vector3D};
use crate::types::shader::{Material, ShadingModel};

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

    /// Performs linear interpolation between two points.
    ///
    /// ### Arguments
    ///
    /// * `i0` - Starting index/position (e.g., starting x or y coordinate)
    /// * `d0` - Starting value at position i0 (e.g., the corresponding y or x value)
    /// * `i1` - Ending index/position
    /// * `d1` - Ending value at position i1
    ///
    /// ### Returns
    ///
    /// A vector of interpolated values as f32, with length equal to abs(i1 - i0).
    /// Each element represents the interpolated value at the corresponding position.
    ///
    /// ### Example
    ///
    /// ```
    /// // Interpolate y values for x coordinates 0 to 5, from y=10 to y=20
    /// let interpolated = linear_interpolation(0, 10, 5, 20);
    /// // Returns: [10.0, 12.0, 14.0, 16.0, 18.0, 20.0]
    /// ```
    ///
    /// ### Notes
    ///
    /// * If i0 == i1, returns a vector with single value d0
    /// * The function interpolates in the direction from i0 to i1
    /// * Useful for line rasterization where you need to find all points between two endpoints
    pub fn linear_interpolation(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<f32> {
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
        result.reserve((steps + 1) as usize);

        for _ in 0..=steps {
            // save d
            result.push(d);
            // with each iteration add another delta to d
            d += a;
        }

        result
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

    /// Draws a triangle in a specified color between three points p0, p1, p2
    ///
    /// # Arguments
    /// * "p0" Point0 as a DisplayBufferPoint
    /// * "p1" Point0 as a DisplayBufferPoint
    /// * "p2" Point0 as a DisplayBufferPoint
    ///
    /// # Details
    ///
    ///
    /// # Example
    ///
    ///
    ///  # Notes
    ///
    pub fn draw_triangle(
        &mut self,
        mut p0: ScreenPoint,
        mut p1: ScreenPoint,
        mut p2: ScreenPoint,
        color: ColorRGB,
    ) {
        // sort the y points such that y0 < y1 < y2
        if p1.y < p0.y {
            swap(&mut p0, &mut p1);
        }
        if p2.y < p0.y {
            swap(&mut p0, &mut p2);
        }
        if p2.y < p1.y {
            swap(&mut p1, &mut p2);
        }

        // calculate boundaries of the triangle given by p0,p1,p2
        // we want the x values for each line between two points, thats why the independent value is y. y = i , x = d
        // naming: x01 -> x values between p0 and p1
        let mut x01 = Rasterizer::linear_interpolation(p0.y, p0.x, p1.y, p1.x);
        let x02 = Rasterizer::linear_interpolation(p0.y, p0.x, p2.y, p2.x);
        let x12 = Rasterizer::linear_interpolation(p1.y, p1.x, p2.y, p2.x);

        // Remove last point to avoid double counting
        if !x01.is_empty() {
            x01.pop();
        }

        // Combine edges to create complete boundary
        let mut x012 = x01;
        x012.extend(x12);


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
            for x in (x_start + 1)..=x_end {
                self.framebuffer.set_pixel(x, y, color);
            }
        }
    }

    pub fn draw_gradient_triangle(
        &mut self,
        mut p0: ScreenPoint,
        mut p1: ScreenPoint,
        mut p2: ScreenPoint,
        mut c0: ColorRGB,
        mut c1: ColorRGB,
        mut c2: ColorRGB,
    ) {
        // sort the y points such that y0 < y1 < y2
        if p1.y < p0.y {
            std::mem::swap(&mut p0, &mut p1);
            std::mem::swap(&mut c0, &mut c1);

        }
        if p2.y < p0.y {
            std::mem::swap(&mut p0, &mut p2);
            std::mem::swap(&mut c0, &mut c2);
        }
        if p2.y < p1.y {
            std::mem::swap(&mut p2, &mut p1);
            std::mem::swap(&mut c2, &mut c1);
        }

        // calculate boundaries of the triangle given by p0,p1,p2
        // we want the x values for each line between two points, thats why the independent value is y. y = i , x = d
        // naming: x01 -> x values between p0 and p1
        let mut x01 = Rasterizer::linear_interpolation(p0.y, p0.x, p1.y, p1.x);
        let x02 = Rasterizer::linear_interpolation(p0.y, p0.x, p2.y, p2.x);
        let x12 = Rasterizer::linear_interpolation(p1.y, p1.x, p2.y, p2.x);

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

        // Precompute some constant terms used in barycentric calculation
        let x0: i32 = p0.x;
        let y0: i32 = p0.y;
        let x1: i32 = p1.x;
        let y1: i32 = p1.y;
        let x2: i32 = p2.x;
        let y2: i32 = p2.y;

        // These terms stay constant for the triangle
        // in naive it this will be called very often:
        // float signedArea = (P1.x - P0.x) * (P2.y - P0.y) - (P1.y - P0.y) * (P2.x - P0.x);
        // we can precalculate these steps:
        let v0x: f32 = x1 as f32 - x0 as f32; // (P1.x - P0.x)
        let v0y: f32 = y1 as f32 - y0 as f32; // (P1.y - P0.y)
        let v1x: f32 = x2 as f32 - x0 as f32; // (P2.x - P0.x)
        let v1y: f32 = y2 as f32 - y0 as f32; // (P2.y - P0.y)

        // and this float signedArea = (P1.x - P0.x) * (P2.y - P0.y) - (P1.y - P0.y) * (P2.x - P0.x);
        // with new terms but as 1/signedArea
        // so we only need to do the division once
        let denom: f32 = 1.0 / (v0x * v1y - v1x * v0y);

        // for every row from the left wall+1 to the right wall -1 set pixel to color
        for y in (p0.y)..=(p2.y) {
            let current_row = (y - p0.y) as usize;
            let x_start = x_left[current_row] as i32;
            let x_end = x_right[current_row] as i32;

            // Fill pixels for current scanline (excluding edges)
            for x in (x_start + 1)..=x_end {
                // Calculate barycentric coordinates more efficiently
                let px: f32 = x as f32 - x0 as f32; // x distance from vertex 0 to current pixel
                let py: f32 = y as f32 - y0 as f32; // y distance from vertex 0 to current pixel

                let alpha = (px * v1y - py * v1x) * denom; // Area(pbc) * (1/Area(abc))
                let beta = (py * v0x - px * v0y) * denom; // Area(pca) * (1/Area(abc))

                // 0x001122
                let r = ((alpha * c0.get_r() as f32
                    + beta * c1.get_r() as f32
                    + (1.0 - alpha - beta) * c2.get_r() as f32)
                    .round()
                    .clamp(0.0, 255.0)) as u8;

                let g = ((alpha * c0.get_g() as f32
                    + beta * c1.get_g() as f32
                    + (1.0 - alpha - beta) * c2.get_g() as f32)
                    .round()
                    .clamp(0.0, 255.0)) as u8;

                let b = ((alpha * c0.get_b() as f32
                    + beta * c1.get_b() as f32
                    + (1.0 - alpha - beta) * c2.get_b() as f32)
                    .round()
                    .clamp(0.0, 255.0)) as u8;

                self.framebuffer.set_pixel(x, y, ColorRGB::from_rgb(r, g, b));
            }
        }
    }

    pub fn shade_triangle(
        vertex: &Point3D,
        camera_position: &Point3D,
        normal: &Vector3D,
        material: &Material,
        lights: &[PointLight],
        shader: &impl ShadingModel,
    ) -> ColorRGB {
        let view_vector = camera_position.sub_p(*vertex).normalize();
        shader.calc_color(vertex, normal, &view_vector, material, lights)
    }

}
