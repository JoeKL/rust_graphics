use crate::renderer::{DrawCommand, Fragment, Rasterizer};
use crate::math::ScreenPoint;
use crate::scene::Vertex;

pub struct RasterizerInput<'a> {
    pub draw_commands: &'a [DrawCommand],
    pub triangle_index_buffer: &'a [u32],
    pub transformed_vertices: &'a [Vertex],
    pub backface_culling: bool,
}

pub struct RasterizerOutput<'a> {
    pub fragment_buffer: &'a mut Vec<Fragment>,
    pub z_buffer: &'a mut [f32],
    pub debug_lines: &'a mut Vec<[i32; 4]>,
}

pub trait RenderPass {
    fn execute(&self, rasterizer: &Rasterizer, input: &RasterizerInput, output: &mut RasterizerOutput);
}

pub struct FacePass;

impl RenderPass for FacePass {
    fn execute(&self, rasterizer: &Rasterizer, input: &RasterizerInput, output: &mut RasterizerOutput) {
        // For each draw command/mesh
        for draw_command in input.draw_commands {
            let index_start = draw_command.first_triangle_index_offset;
            let index_length = draw_command.triangle_index_count;
            let index_end = index_length + index_start;

            // Process indices in groups of 3 to form triangles
            for i in (index_start..index_end).step_by(3) {
                // Get vertex indices
                let i0 = input.triangle_index_buffer[i];
                let i1 = input.triangle_index_buffer[i + 1];
                let i2 = input.triangle_index_buffer[i + 2];

                // Get transformed vertices
                let v0: &Vertex = &input.transformed_vertices[i0 as usize];
                let v1 = &input.transformed_vertices[i1 as usize];
                let v2 = &input.transformed_vertices[i2 as usize];

                // Check if triangle is partly on screen
                if !rasterizer.is_triangle_on_screen(v0, v1, v2) {
                    continue;
                }

                let bounds_min_x: i32;
                let bounds_min_y: i32;
                let bounds_max_x: i32;
                let bounds_max_y: i32;

                // create boundingbox from v0, v1, v2
                (bounds_min_x, bounds_min_y, bounds_max_x, bounds_max_y) =
                    rasterizer.calculate_bounding_box(v0, v1, v2);

                // 1. PRE-CALCULATION
                // Create aliases for positions to make math cleaner (p = position)
                let p0 = &v0.position;
                let p1 = &v1.position;
                let p2 = &v2.position;

                // Calculate triangle vectors
                let v0_to_v1_x = p1[0] - p0[0];
                let v0_to_v1_y = p1[1] - p0[1];
                let v0_to_v2_x = p2[0] - p0[0];
                let v0_to_v2_y = p2[1] - p0[1];

                // Calculate denominator once (cross product Z component)
                let denominator = v0_to_v1_x * v0_to_v2_y - v0_to_v2_x * v0_to_v1_y;

                // OPTIMIZATION: Skip degenerate triangles (zero area)
                if denominator.abs() < f32::EPSILON {
                    continue;
                }

                if input.backface_culling && denominator >= 0.0 {
                    continue;
                }

                // Calculate inverse once to replace division with multiplication
                let inv_denominator = 1.0 / denominator;

                // For each pixel in triangle's bounding box:
                // traverse the bounding box in scanline
                for y in (bounds_min_y)..(bounds_max_y) {
                    for x in (bounds_min_x)..(bounds_max_x) {
                        let fx = x as f32;
                        let fy = y as f32;

                        // Vector from Point to p0
                        let p_to_v0_x = fx - p0[0];
                        let p_to_v0_y = fy - p0[1];

                        // Calculate Beta and Gamma using Multiplications
                        let beta =
                            (p_to_v0_x * v0_to_v2_y - v0_to_v2_x * p_to_v0_y) * inv_denominator;
                        let gamma =
                            (v0_to_v1_x * p_to_v0_y - p_to_v0_x * v0_to_v1_y) * inv_denominator;
                        let alpha = 1.0 - beta - gamma;

                        // Test if pixel is inside triangle
                        if (0.0..=1.0).contains(&alpha)
                            && (0.0..=1.0).contains(&beta)
                            && (0.0..=1.0).contains(&gamma)
                        {
                            // Interpolate Z, color, normal using barycentric
                            let interpolated_z = alpha * v0.position[2]
                                + beta * v1.position[2]
                                + gamma * v2.position[2];

                            let interpolated_color = [
                                alpha * v0.color[0] + beta * v1.color[0] + gamma * v2.color[0],
                                alpha * v0.color[1] + beta * v1.color[1] + gamma * v2.color[1],
                                alpha * v0.color[2] + beta * v1.color[2] + gamma * v2.color[2],
                            ];

                            let interpolated_normal = [
                                alpha * v0.normal[0] + beta * v1.normal[0] + gamma * v2.normal[0],
                                alpha * v0.normal[1] + beta * v1.normal[1] + gamma * v2.normal[1],
                                alpha * v0.normal[2] + beta * v1.normal[2] + gamma * v2.normal[2],
                            ];

                            // setup z index to access right place in buffer
                            let z_buffer_idx = y as usize
                                * rasterizer.framebuffer.get_width()
                                + x as usize;

                            // Create and store fragment if Z-test passes
                            // Z-test before creating fragment
                            if interpolated_z < output.z_buffer[z_buffer_idx] {
                                // Only if closer than what's in zbuffer at coordinates
                                output.z_buffer[z_buffer_idx] = interpolated_z; // Update z-buffer

                                output.fragment_buffer.push(Fragment {
                                    x,
                                    y,
                                    z: interpolated_z,
                                    color: interpolated_color,
                                    normal: interpolated_normal,
                                    material_id: draw_command.material_id,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct VertexPass;

impl RenderPass for VertexPass {
    fn execute(&self, rasterizer: &Rasterizer, input: &RasterizerInput, output: &mut RasterizerOutput) {
        // For each draw command/mesh
        for draw_command in input.draw_commands {
            let index_start = draw_command.first_triangle_index_offset;
            let index_length = draw_command.triangle_index_count;
            let index_end = index_length + index_start;

            // Process indices in groups of 3 to form triangles
            for i in (index_start..index_end).step_by(3) {
                // Get vertex indices
                let i0 = input.triangle_index_buffer[i];
                let i1 = input.triangle_index_buffer[i + 1];
                let i2 = input.triangle_index_buffer[i + 2];

                // Get transformed vertices
                let v0: &Vertex = &input.transformed_vertices[i0 as usize];
                let v1 = &input.transformed_vertices[i1 as usize];
                let v2 = &input.transformed_vertices[i2 as usize];

                // Check if triangle is partly on screen
                if !rasterizer.is_triangle_on_screen(v0, v1, v2) {
                    continue;
                }

                let fragment_storage = [
                    [v0.position[0] as i32, v0.position[1] as i32],
                    [v1.position[0] as i32, v1.position[1] as i32],
                    [v2.position[0] as i32, v2.position[1] as i32],
                ];

                for fragment_chunk in fragment_storage {
                    output.fragment_buffer.push(Fragment {
                        x: fragment_chunk[0],
                        y: fragment_chunk[1],
                        z: 0.0,
                        color: [1.0, 1.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                        material_id: 0,
                    });
                }
            }
        }
    }
}

pub struct WireframePass;

impl RenderPass for WireframePass {
    fn execute(&self, rasterizer: &Rasterizer, input: &RasterizerInput, output: &mut RasterizerOutput) {
        // For each draw command/mesh
        for draw_command in input.draw_commands {
            let index_start = draw_command.first_triangle_index_offset;
            let index_length = draw_command.triangle_index_count;
            let index_end = index_length + index_start;

            // Process indices in groups of 3 to form triangles
            for i in (index_start..index_end).step_by(3) {
                // Get vertex indices
                let i0 = input.triangle_index_buffer[i];
                let i1 = input.triangle_index_buffer[i + 1];
                let i2 = input.triangle_index_buffer[i + 2];

                // Get transformed vertices
                let v0: &Vertex = &input.transformed_vertices[i0 as usize];
                let v1 = &input.transformed_vertices[i1 as usize];
                let v2 = &input.transformed_vertices[i2 as usize];

                // Check if triangle is partly on screen
                if !rasterizer.is_triangle_on_screen(v0, v1, v2) {
                    continue;
                }

                let p0 = ScreenPoint::new(v0.position[0] as i32, v0.position[1] as i32);
                let p1 = ScreenPoint::new(v1.position[0] as i32, v1.position[1] as i32);
                let p2 = ScreenPoint::new(v2.position[0] as i32, v2.position[1] as i32);

                let mut push_fragment = |x, y| {
                    output.fragment_buffer.push(Fragment {
                        x,
                        y,
                        z: 0.0,
                        color: [1.0, 1.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                        material_id: 0,
                    });
                };

                rasterizer.for_each_line_point(p0, p1, &mut push_fragment);
                rasterizer.for_each_line_point(p1, p2, &mut push_fragment);
                rasterizer.for_each_line_point(p0, p2, &mut push_fragment);
            }
        }
    }
}

pub struct VertexNormalPass;

impl RenderPass for VertexNormalPass {
    fn execute(&self, rasterizer: &Rasterizer, _input: &RasterizerInput, output: &mut RasterizerOutput) {
        for [x1, y1, x2, y2] in output.debug_lines.drain(..) {
            let p0 = ScreenPoint::new(x1, y1);
            let p1 = ScreenPoint::new(x2, y2);
            rasterizer.for_each_line_point(p0, p1, |x, y| {
                output.fragment_buffer.push(Fragment {
                    x,
                    y,
                    z: 0.0,
                    color: [1.0, 1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                    material_id: 0,
                });
            });
        }
    }
}
