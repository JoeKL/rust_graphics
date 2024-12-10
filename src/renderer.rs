use crate::color::ColorRGB;
use crate::primitives::*;
use crate::DisplayBuffer;
use crate::DisplayBufferPoint;
use crate::obj_loader::{create_triangles, create_vertices};
use crate::camera::Camera;

// Viewport matrix (just the screen transformation part)
fn create_viewport_matrix(width: f32, height: f32) -> Mat4x4 {
    Mat4x4 {
        mat: [
            [width / 2.0, 0.0, 0.0, width / 2.0],
            [0.0, -height / 2.0, 0.0, height / 2.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn flat_shade_triangle(
    triangle: Triangle,
    color: ColorRGB,
    light_pos: Point,
    light_color: ColorRGB,
) -> ColorRGB {
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
        light_color.get_r() as f32 / 255.0, // Convert 0-255 to 0-1 range
        light_color.get_g() as f32 / 255.0,
        light_color.get_b() as f32 / 255.0,
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
    let l: Vector = light_pos.sub_p(x).normalize();

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

pub fn update(display_buffer: &mut DisplayBuffer, step: u32) {
    display_buffer.fill(ColorRGB::BLACK);

    let point_a = Point::new(-10.0, -10.0, -10.0);
    let point_b = Point::new(-10.0, -10.0, 10.0);
    let point_c = Point::new(-10.0, 10.0, -10.0);
    let point_d = Point::new(-10.0, 10.0, 10.0);
    let point_e = Point::new(10.0, -10.0, -10.0);
    let point_f = Point::new(10.0, -10.0, 10.0);
    let point_g = Point::new(10.0, 10.0, -10.0);
    let point_h = Point::new(10.0, 10.0, 10.0);

    let lines: [Line; 12];
    lines = [
        Line::new(point_a, point_b),
        Line::new(point_a, point_c),
        Line::new(point_a, point_e),
        Line::new(point_b, point_f),
        Line::new(point_b, point_d),
        Line::new(point_d, point_c),
        Line::new(point_e, point_f),
        Line::new(point_g, point_c),
        Line::new(point_g, point_e),
        Line::new(point_h, point_f),
        Line::new(point_h, point_d),
        Line::new(point_h, point_g),
    ];

    let vertices = create_vertices();
    let mut triangles = create_triangles(&vertices);

    // let p0 = DisplayBufferPoint {x: 75, y: display_buffer.canvas_height as i32 - 100};
    // let p1 = DisplayBufferPoint {x: display_buffer.canvas_width as i32/2, y: 100};
    // let p2 = DisplayBufferPoint { x: display_buffer.canvas_width as i32 - 75,y: display_buffer.canvas_height as i32 - 100 };

    // display_buffer.draw_gradient_triangle(p0, p1, p2, ColorRGB::RED, ColorRGB::BLUE, ColorRGB::GREEN);
    // display_buffer.draw_triangle(p0, p1, p2, ColorRGB::BLUE);

    let light_pos: Point = Point::new(15.0, 15.0, 0.0);
    let light_color: ColorRGB = ColorRGB::WHITE;

    let eye = Point::new(0.0, 0.0, 50.0 + step as f32);
    let target = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let far: f32 = 75.0;
    let near: f32 = 1.0;

    let mut cam = Camera::new(eye, target, up);
    cam.set_projection_params(
        45.0, // 60 degree FOV
        display_buffer.canvas_width as f32 / display_buffer.canvas_height as f32,
        near,
        far
    );

    let look_at = cam.get_look_at_matrix();
    let projection = cam.get_projection_matrix();

    look_at.print_with_label("look_at");
    projection.print_with_label("projection");

    let viewport = create_viewport_matrix(display_buffer.canvas_width as f32, display_buffer.canvas_height as f32);


    for line in lines {
        let mut start_point = line.a;
        let mut end_point = line.b;

        // After look_at
        start_point = look_at.mul_point(start_point);
        end_point = look_at.mul_point(end_point);

        // After projection
        start_point = projection.mul_point(start_point);
        end_point = projection.mul_point(end_point);

        // After perspective divide
        start_point.dehomogen();
        end_point.dehomogen();

        // After viewport
        start_point = viewport.mul_point(start_point);
        end_point = viewport.mul_point(end_point);

        let screen_point_a = DisplayBufferPoint {
            y: start_point.y as i32,
            x: start_point.x as i32,
        };
        let screen_point_b = DisplayBufferPoint {
            x: end_point.x as i32,
            y: end_point.y as i32,
        };

        // println!("{:#?}", start_point);

        display_buffer.draw_line(
            screen_point_a,
            screen_point_b,
            ColorRGB::from_rgb(255, 255, 255),
        );
    }


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

        // Calculate squared distances to eye
        let dist_a = (center_a.x - eye.x).powi(2)
            + (center_a.y - eye.y).powi(2)
            + (center_a.z - eye.z).powi(2);
        let dist_b = (center_b.x - eye.x).powi(2)
            + (center_b.y - eye.y).powi(2)
            + (center_b.z - eye.z).powi(2);

        // Sort furthest first
        dist_b.partial_cmp(&dist_a).unwrap()
    });

    for triangle in triangles {
        let mut point_0: Point = triangle.a;
        let mut point_1: Point = triangle.b;
        let mut point_2: Point = triangle.c;

        // After look_at
        point_0 = look_at.mul_point(point_0);
        point_1 = look_at.mul_point(point_1);
        point_2 = look_at.mul_point(point_2);

        // After projection
        point_0 = projection.mul_point(point_0);
        point_1 = projection.mul_point(point_1);
        point_2 = projection.mul_point(point_2);

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

        display_buffer.draw_triangle(
            screen_point_0,
            screen_point_1,
            screen_point_2,
            flat_shade_triangle(
                triangle,
                ColorRGB::from_rgb(0, 255, 200),
                light_pos,
                light_color,
            ),
        );
    }

    let origin = Point::new(0.0, 0.0, 0.0);
    let x_end = Point::new(20.0, 0.0, 0.0); // X axis in red
    let y_end = Point::new(0.0, 20.0, 0.0); // Y axis in green
    let z_end = Point::new(0.0, 0.0, 20.0); // Z axis in blue

    let axes = [
        (origin, x_end, ColorRGB::RED),        // X axis - red
        (origin, y_end, ColorRGB::GREEN),      // Y axis - green
        (origin, z_end, ColorRGB::BLUE),       // Z axis - blue
        (origin, light_pos, ColorRGB::YELLOW), // light source - yellow
    ];

    for (start, end, color) in axes {
        let mut start_point = start;
        let mut end_point = end;

        start_point = look_at.mul_point(start_point);
        end_point = look_at.mul_point(end_point);

        start_point = projection.mul_point(start_point);
        end_point = projection.mul_point(end_point);

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

        display_buffer.draw_line(screen_start, screen_end, color);
    }
}
