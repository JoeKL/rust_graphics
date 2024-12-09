mod color;
mod displaybuffer;
mod primitives;

use color::ColorRGB;
use displaybuffer::{DisplayBuffer, DisplayBufferPoint};
use minifb::{Key, Window, WindowOptions};
use primitives::*;

static WINDOW_WIDTH: usize = 1200;
static WINDOW_HEIGHT: usize = 800;

static mut MOUSE_ROT_MAT: Mat4x4 = Mat4x4 {
    mat: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

// LookAt matrix calculation
fn create_look_at_matrix(eye: Vector, target: Vector, up: Vector) -> Mat4x4 {
    // Calculate forward (negative z-axis)
    let mut forward = target.sub(eye);
    forward.normalize();

    // Calculate right vector
    let mut right = up.cross(forward);
    right.normalize();

    // Calculate up vector
    let up = forward.cross(right);

    Mat4x4 {
        mat: [
            [right.x, right.y, right.z, -right.dot(eye)],
            [up.x, up.y, up.z, -up.dot(eye)],
            [forward.x, forward.y, forward.z, -forward.dot(eye)],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

// Perspective projection matrix
fn create_perspective_matrix(fov_degrees: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4x4 {
    let fov_rad = fov_degrees * std::f32::consts::PI / 180.0;
    let f = 1.0 / (fov_rad / 2.0).tan();

    Mat4x4 {
        mat: [
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0, 0.0,
                (far + near) / (near - far),
                (2.0 * far * near) / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ],
    }
}

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

pub fn update(display_buffer: &mut DisplayBuffer, mouse_rot_mat: &Mat4x4) {
    display_buffer.fill(ColorRGB::BLACK);

    let lines: [Line; 12];

    let point_a = Point::new(-10.0, -10.0, -10.0);
    let point_b = Point::new(-10.0, -10.0, 10.0);
    let point_c = Point::new(-10.0, 10.0, -10.0);
    let point_d = Point::new(-10.0, 10.0, 10.0);
    let point_e = Point::new(10.0, -10.0, -10.0);
    let point_f = Point::new(10.0, -10.0, 10.0);
    let point_g = Point::new(10.0, 10.0, -10.0);
    let point_h = Point::new(10.0, 10.0, 10.0);

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

    // let p0 = DisplayBufferPoint {x: 75, y: WINDOW_HEIGHT as i32 - 100};
    // let p1 = DisplayBufferPoint {x: WINDOW_WIDTH as i32/2, y: 100};
    // let p2 = DisplayBufferPoint { x: WINDOW_WIDTH as i32 - 75,y: WINDOW_HEIGHT as i32 - 100 };

    // display_buffer.draw_gradient_triangle(p0, p1, p2, ColorRGB::RED, ColorRGB::BLUE, ColorRGB::GREEN);

    let eye = Vector::new(0.0, 0.0, 50.0);
    let target = Vector::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let far: f32 = 100.0;
    let near: f32 = 1.0;

    let look_at = create_look_at_matrix(eye, target, up);

    let projection = create_perspective_matrix(
        60.0, // 60 degree FOV
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        near,
        far,
    );
    let viewport = create_viewport_matrix(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

    for line in lines {
        let mut start_point = line.a;
        let mut end_point = line.b;

        // After model (mouse rotation)
        start_point = mouse_rot_mat.mul_point(start_point);
        end_point = mouse_rot_mat.mul_point(end_point);

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
    let mut x_old = 0.0;
    let mut y_old = 0.0;
    let mut is_dragging = false;

    // Move mouse rotation matrix here
    let mut mouse_rot_mat = Mat4x4 {
        mat: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    // Update display
    update(&mut display_buffer, &mouse_rot_mat);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if window.get_mouse_down(minifb::MouseButton::Left) {
                // Convert mouse coordinates to center-oriented system
                // Shifts origin to center and flips Y axis (screen coordinates to mathematical)
                let x_new = WINDOW_WIDTH as f32 / 2.0 - x;
                let y_new = WINDOW_HEIGHT as f32 / 2.0 - y;

                if !is_dragging {
                    // Only update start position when we first click
                    x_old = x_new;
                    y_old = y_new;
                    is_dragging = true;
                }

                // Only proceed if mouse actually moved
                if x_old != x_new || y_old != y_new {
                    // Create vectors from old and new mouse positions
                    // Adding z=size/2 projects points onto a virtual hemisphere
                    let mut v_old = Vector::new(x_old, y_old, 400 as f32 / 2.0);
                    v_old.normalize();
                    let mut v_new = Vector::new(x_new, y_new, 400 as f32 / 2.0);
                    v_new.normalize();

                    // Check if movement is significant enough
                    if v_old.sub(v_new).norm() > 0.001 {
                        // Calculate rotation axis and angle
                        // Cross product gives rotation axis
                        let mut n = v_old.cross(v_new);
                        // Sine of rotation angle
                        let sin = n.norm();
                        // Cosine of rotation angle from dot product
                        let cos = v_old.dot(v_new);

                        println!("{:#?}", n);

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
                        mouse_rot_mat = new_mouse_rot_mat.mul_mat(mouse_rot_mat);
                        // Combine with previous rotation
                        
                    }

                    // Update display
                    update(&mut display_buffer, &mouse_rot_mat);

                    
                    // Store current position for next movement
                    x_old = x_new;
                    y_old = y_new;
                }
            } else {
                is_dragging = false;  
            }
            window
                .update_with_buffer(&display_buffer.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
                .unwrap();
        }
    }
}
