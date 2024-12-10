use crate::color::ColorRGB;
use crate::light_source::LightSource;
use crate::primitives::*;
use crate::scene::Scene;
use crate::DisplayBuffer;
use crate::DisplayBufferPoint;

pub fn flat_shade_triangle(triangle: Triangle, color: ColorRGB, light: LightSource) -> ColorRGB {
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    // Convert colors to floating point vectors (0-255 -> 0.0-1.0)
    let tri_color: Vector = Vector::new(
        color.get_r() as f32 / 255.0, // Convert 0-255 to 0-1 range
        color.get_g() as f32 / 255.0,
        color.get_b() as f32 / 255.0,
    );
    let light_color_vec: Vector = Vector::new(
        light.get_color().get_r() as f32 / 255.0, // Convert 0-255 to 0-1 range
        light.get_color().get_g() as f32 / 255.0,
        light.get_color().get_b() as f32 / 255.0,
    );

    // Phong lighting coefficients
    let ambient = 0.3;
    let diffuse = 0.7;
    let specular = 0.1;
    let shininess = 10.0;

    // Calculate triangle center point
    let x: Point = Point::new(
        (a.x + b.x + c.x) / 3.0,
        (a.y + b.y + c.y) / 3.0,
        (a.z + b.z + c.z) / 3.0,
    );

    let n = Vector::new(x.x, x.y, x.z + 2.0).normalize();

    // Calculate view vector (from surface point to camera at origin)
    let v: Vector = Vector::new(0.0, 0.0, 50.0).sub_p(x).normalize();

    // Calculate light vector (from surface point to light source)
    let l: Vector = light.get_position().sub_p(x).normalize();

    // Calculate halfway vector for specular reflection
    let h: Vector = v.add(l).normalize();

    // Calculate color components
    let ca = tri_color.mul(ambient); // Ambient color = surface color * ambient coefficient
    let cd = tri_color.mul(diffuse); // Diffuse color = surface color * diffuse coefficient
    let cs = Vector::new(1.0, 1.0, 1.0).mul(specular); // Specular color (white) * specular coefficient

    // Calculate Phong lighting components
    let ambient_part = ca;
    let diffuse_part = cd.mul(f32::max(l.dot(n), 0.0)); // Diffuse = cd * max(0, l·n)
    let specular_part = cs.mul(f32::max(h.dot(n), 0.0).powf(shininess)); // Specular = cs * max(0, h·n)^shininess

    // Combine components and multiply by light color
    let mut flat_color = ambient_part
        .add(diffuse_part)
        .add(specular_part)
        .mul_vec(light_color_vec);

    // Clamp color values between 0 and 1 to prevent overflow
    flat_color.x = f32::min(flat_color.x, 1.0);
    flat_color.y = f32::min(flat_color.y, 1.0);
    flat_color.z = f32::min(flat_color.z, 1.0);

    // Convert back to RGB color (0.0-1.0 -> 0-255)
    ColorRGB::from_rgb(
        ColorRGB::f32_to_color_component(flat_color.x),
        ColorRGB::f32_to_color_component(flat_color.y),
        ColorRGB::f32_to_color_component(flat_color.z),
    )
}

pub struct RenderEngine {
    display_buffer: DisplayBuffer,
    scene: Scene,
    frame: u32,
}

impl RenderEngine {
    pub fn new(window_width: u32, window_height: u32) -> RenderEngine {
        let display_buffer = DisplayBuffer::new(window_width as usize, window_height as usize);

        let mut scene = Scene::new();

        let far: f32 = 75.0;
        let near: f32 = 1.0;
        scene.camera.set_projection_params(
            15.0, // 60 degree FOV
            display_buffer.canvas_width as f32 / display_buffer.canvas_height as f32,
            near,
            far,
        );
        let frame = 0;

        RenderEngine {
            display_buffer,
            scene,
            frame,
        }
    }

    pub fn render_frame(&mut self) -> Vec<u32> {
        self.frame += 1;
        self.display_buffer.fill(ColorRGB::BLACK);

        let look_at_projection = self.scene.camera.get_look_at_projection_matrix();

        let viewport = self.display_buffer.create_viewport_matrix();

        // let point_a = Point::new(-10.0, -10.0, -10.0);
        // let point_b = Point::new(-10.0, -10.0, 10.0);
        // let point_c = Point::new(-10.0, 10.0, -10.0);
        // let point_d = Point::new(-10.0, 10.0, 10.0);
        // let point_e = Point::new(10.0, -10.0, -10.0);
        // let point_f = Point::new(10.0, -10.0, 10.0);
        // let point_g = Point::new(10.0, 10.0, -10.0);
        // let point_h = Point::new(10.0, 10.0, 10.0);

        // let lines: [Line; 12];
        // lines = [
        //     Line::new(point_a, point_b),
        //     Line::new(point_a, point_c),
        //     Line::new(point_a, point_e),
        //     Line::new(point_b, point_f),
        //     Line::new(point_b, point_d),
        //     Line::new(point_d, point_c),
        //     Line::new(point_e, point_f),
        //     Line::new(point_g, point_c),
        //     Line::new(point_g, point_e),
        //     Line::new(point_h, point_f),
        //     Line::new(point_h, point_d),
        //     Line::new(point_h, point_g),
        // ];

        // let p0 = DisplayBufferPoint {x: 75, y: display_buffer.canvas_height as i32 - 100};
        // let p1 = DisplayBufferPoint {x: display_buffer.canvas_width as i32/2, y: 100};
        // let p2 = DisplayBufferPoint { x: display_buffer.canvas_width as i32 - 75,y: display_buffer.canvas_height as i32 - 100 };

        // display_buffer.draw_gradient_triangle(p0, p1, p2, ColorRGB::RED, ColorRGB::BLUE, ColorRGB::GREEN);
        // display_buffer.draw_triangle(p0, p1, p2, ColorRGB::BLUE);

        // for line in lines {
        //     let mut start_point = line.a;
        //     let mut end_point = line.b;

        //     // After look_at & projection
        //     start_point = look_at_projection.mul_point(start_point);
        //     end_point = look_at_projection.mul_point(end_point);

        //     // After perspective divide
        //     start_point.dehomogen();
        //     end_point.dehomogen();

        //     // After viewport
        //     start_point = viewport.mul_point(start_point);
        //     end_point = viewport.mul_point(end_point);

        //     let screen_point_a = DisplayBufferPoint {
        //         y: start_point.y as i32,
        //         x: start_point.x as i32,
        //     };
        //     let screen_point_b = DisplayBufferPoint {
        //         x: end_point.x as i32,
        //         y: end_point.y as i32,
        //     };

        //     // println!("{:#?}", start_point);

        //     display_buffer.draw_line(
        //         screen_point_a,
        //         screen_point_b,
        //         ColorRGB::from_rgb(255, 255, 255),
        //     );
        // }

        // let mut camera_updated_x = self.scene.camera.position.x;
        // camera_updated_x += 1.0;


        // self.scene.camera.set_position(Point::new(
        //     camera_updated_x,
        //     self.scene.camera.position.y,
        //     self.scene.camera.position.z,
        // ));

        let alpha: f32 = 0.01;

        let rot_x_mat = Mat4x4::new([
            [alpha.cos(), 0.0, alpha.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-alpha.sin(), 0.0, alpha.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);


        self.scene.mesh_list[0].transform_mesh(rot_x_mat);

        let origin = Point::new(0.0, 0.0, 0.0);
        let x_end = Point::new(5.0, 0.0, 0.0); // X axis in red
        let y_end = Point::new(0.0, 5.0, 0.0); // Y axis in green
        let z_end = Point::new(0.0, 0.0, 5.0); // Z axis in blue

        let axes = [
            (origin, x_end, ColorRGB::RED),   // X axis - red
            (origin, y_end, ColorRGB::GREEN), // Y axis - green
            (origin, z_end, ColorRGB::BLUE),  // Z axis - blue
            (
                origin,
                self.scene.lights[0].get_position(),
                ColorRGB::YELLOW,
            ), // light source - yellow
        ];

        for (start, end, color) in axes {
            let mut start_point = start;
            let mut end_point = end;

            start_point = look_at_projection.mul_point(start_point);
            end_point = look_at_projection.mul_point(end_point);

            start_point.dehomogen();
            end_point.dehomogen();

            start_point = viewport.mul_point(start_point);
            end_point = viewport.mul_point(end_point);

            let screen_start = DisplayBufferPoint {
                x: start_point.x as i32,
                y: start_point.y as i32,
            };
            let screen_end = DisplayBufferPoint {
                x: end_point.x as i32,
                y: end_point.y as i32,
            };

            self.display_buffer
                .draw_line(screen_start, screen_end, color);
        }

        let mut triangles = self.scene.mesh_list[0].get_triangles();

        // Sort based on distance to eye
        triangles.sort_by(|a, b| {
            // Calculate centers
            let center_a = Point::new(
                (a.a.x + a.b.x + a.c.x) / 3.0,
                (a.a.y + a.b.y + a.c.y) / 3.0,
                (a.a.z + a.b.z + a.c.z) / 3.0,
            );
            let center_b = Point::new(
                (b.a.x + b.b.x + b.c.x) / 3.0,
                (b.a.y + b.b.y + b.c.y) / 3.0,
                (b.a.z + b.b.z + b.c.z) / 3.0,
            );

            // Calculate squared distances to cam.position
            let dist_a = (center_a.x - self.scene.camera.position.x).powi(2)
                + (center_a.y - self.scene.camera.position.y).powi(2)
                + (center_a.z - self.scene.camera.position.z).powi(2);
            let dist_b = (center_b.x - self.scene.camera.position.x).powi(2)
                + (center_b.y - self.scene.camera.position.y).powi(2)
                + (center_b.z - self.scene.camera.position.z).powi(2);

            // Sort furthest first
            dist_b.partial_cmp(&dist_a).unwrap()
        });

        for triangle in triangles {
            let mut point_0: Point = triangle.a;
            let mut point_1: Point = triangle.b;
            let mut point_2: Point = triangle.c;            

            // After look_at_projection
            point_0 = look_at_projection.mul_point(point_0);
            point_1 = look_at_projection.mul_point(point_1);
            point_2 = look_at_projection.mul_point(point_2);

            // After perspective divide
            point_0.dehomogen();
            point_1.dehomogen();
            point_2.dehomogen();

            // After viewport
            point_0 = viewport.mul_point(point_0);
            point_1 = viewport.mul_point(point_1);
            point_2 = viewport.mul_point(point_2);

            let screen_point_0 = DisplayBufferPoint {
                y: point_0.y as i32,
                x: point_0.x as i32,
            };
            let screen_point_1 = DisplayBufferPoint {
                x: point_1.x as i32,
                y: point_1.y as i32,
            };
            let screen_point_2 = DisplayBufferPoint {
                x: point_2.x as i32,
                y: point_2.y as i32,
            };

            self.display_buffer.draw_triangle(
                screen_point_0,
                screen_point_1,
                screen_point_2,
                flat_shade_triangle(
                    triangle,
                    ColorRGB::from_rgb(0, 255, 200),
                    self.scene.lights[0],
                ),
            );
        }
        return self.display_buffer.get_buffer().to_vec();
    }
}
