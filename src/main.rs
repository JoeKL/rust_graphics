mod color;
mod displaybuffer;
mod primitives;

use color::ColorRGB;
use displaybuffer::{DisplayBuffer, DisplayBufferPoint};
use minifb::{Key, Window, WindowOptions};
use primitives::*;

static WINDOW_WIDTH: usize = 800;
static WINDOW_HEIGHT: usize = 800;

static LOOK_AT_MAT: Mat4x4 = Mat4x4 {
    mat: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -250.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

static FAR: f32 = 100.0;
static NEAR: f32 = 1.0;

static PROJECTION_MAT: Mat4x4 = Mat4x4 {
    mat: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, -1.0, 0.0],
    ],
};

static VIEW_PORT_MAT: Mat4x4 = Mat4x4 {
    mat: [
        [
            WINDOW_WIDTH as f32 / 2.0,
            0.0,
            0.0,
            WINDOW_WIDTH as f32 / 2.0,
        ],
        [
            0.0,
            WINDOW_HEIGHT as f32 / 2.0,
            0.0,
            WINDOW_HEIGHT as f32 / 2.0,
        ],
        [
            0.0,
            0.0,
            (-FAR - NEAR) / (FAR - NEAR),
            (-2.0 * FAR) / (FAR - NEAR),
        ],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

static mut MOUSE_ROT_MAT: Mat4x4 = Mat4x4 {
    mat: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

pub fn update(display_buffer: &mut DisplayBuffer) {
    display_buffer.fill(ColorRGB::BLACK);

    let lines: [Line; 12];

    let point_a = Point::new(-100.0, -100.0, -100.0);
    let point_b = Point::new(-100.0, -100.0, 100.0);
    let point_c = Point::new(-100.0, 100.0, -100.0);
    let point_d = Point::new(-100.0, 100.0, 100.0);
    let point_e = Point::new(100.0, -100.0, -100.0);
    let point_f = Point::new(100.0, -100.0, 100.0);
    let point_g = Point::new(100.0, 100.0, -100.0);
    let point_h = Point::new(100.0, 100.0, 100.0);

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

    // let transform: Mat4x4 = PROJECTION_MAT;
    // let transform: Mat4x4 = VIEW_PORT_MAT.mul_mat(PROJECTION_MAT).mul_mat(LOOK_AT_MAT);
    let mut transform = Mat4x4::new_identity();
    transform = unsafe { MOUSE_ROT_MAT.mul_mat(transform) };
    transform = LOOK_AT_MAT.mul_mat(transform);
    transform = PROJECTION_MAT.mul_mat(transform);
    transform = VIEW_PORT_MAT.mul_mat(transform);

    for line in lines {
        let mut start_point = line.a;
        let mut end_point = line.b;

        start_point = transform.mul_point(start_point);
        end_point = transform.mul_point(end_point);

        start_point.dehomogen();
        end_point.dehomogen();

        let screen_point_a = DisplayBufferPoint {
            x: start_point.x as i32,
            y: start_point.y as i32,
        };
        let screen_point_b = DisplayBufferPoint {
            x: end_point.x as i32,
            y: end_point.y as i32,
        };

        display_buffer.draw_line(
            screen_point_a,
            screen_point_b,
            ColorRGB::from_rgb(255, 255, 255),
        );
    }

    // let p0 = DisplayBufferPoint {x: 75, y: WINDOW_HEIGHT as i32 - 100};
    // let p1 = DisplayBufferPoint {x: WINDOW_WIDTH as i32/2, y: 100};
    // let p2 = DisplayBufferPoint { x: WINDOW_WIDTH as i32 - 75,y: WINDOW_HEIGHT as i32 - 100 };

    // display_buffer.draw_gradient_triangle(p0, p1, p2, ColorRGB::RED, ColorRGB::BLUE, ColorRGB::GREEN);
}

fn main() {
    let mut window = Window::new(
        "Rust Graphics",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let mut display_buffer = DisplayBuffer::new(WINDOW_HEIGHT, WINDOW_WIDTH);
    let mut x0 = 0.0;
    let mut y0 = 0.0;

    // Update display
    update(&mut display_buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if window.get_mouse_down(minifb::MouseButton::Left) {
                // Convert mouse coordinates to center-oriented system
                // Shifts origin to center and flips Y axis (screen coordinates to mathematical)
                let x1 = x - WINDOW_WIDTH as f32 / 2.0;
                let y1 = y - WINDOW_HEIGHT as f32 / 2.0;

                // Only proceed if mouse actually moved
                if x0 != x1 || y0 != y1 {
                    // Create vectors from old and new mouse positions
                    // Adding z=size/2 projects points onto a virtual hemisphere
                    let mut v0 = Vector::new(x0, y0, WINDOW_WIDTH as f32 / 2.0);
                    v0.normalize();
                    let mut v1 = Vector::new(x1, y1, WINDOW_WIDTH as f32 / 2.0);
                    v1.normalize();

                    // Check if movement is significant enough
                    if v0.sub(v1).norm() > 0.001 {
                        // Calculate rotation axis and angle
                        // Cross product gives rotation axis
                        let mut n = v0.cross(v1);
                        // Sine of rotation angle
                        let sin = n.norm();
                        // Cosine of rotation angle from dot product
                        let cos = v0.dot(v1);

                        // Normalize rotation axis
                        n.normalize();

                        // Create rotation matrix using Rodrigues' rotation formula
                        let new_mouse_rot_mat = Mat4x4 {
                            mat: [
                                // First row
                                [
                                    (1.0 - cos) * n.x * n.x + cos,       // diagonal term
                                    (1.0 - cos) * n.x * n.y - sin * n.z, // off-diagonal terms
                                    (1.0 - cos) * n.x * n.z + sin * n.y, // with sin terms for rotation
                                    0.0,
                                ],
                                // Second row
                                [
                                    (1.0 - cos) * n.y * n.x + sin * n.z,
                                    (1.0 - cos) * n.y * n.y + cos,
                                    (1.0 - cos) * n.y * n.z - sin * n.x,
                                    0.0,
                                ],
                                // Third row
                                [
                                    (1.0 - cos) * n.z * n.x - sin * n.y,
                                    (1.0 - cos) * n.z * n.y + sin * n.x,
                                    (1.0 - cos) * n.z * n.z + cos,
                                    0.0,
                                ],
                                // Fourth row (homogeneous coordinates)
                                [0.0, 0.0, 0.0, 1.0],
                            ],
                        };
                        unsafe { MOUSE_ROT_MAT = new_mouse_rot_mat.mul_mat(MOUSE_ROT_MAT) };
                        // Combine with previous rotation

                        unsafe { MOUSE_ROT_MAT.print_with_label("Mous_rot") };
                    }

                    // Update display
                    update(&mut display_buffer);

                    // Store current position for next movement
                    x0 = x1;
                    y0 = y1;
                }
            }
            window
                .update_with_buffer(&display_buffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
                .unwrap();
        }
    }
}
