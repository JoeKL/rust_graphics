use crate::types::math::Mat4x4;

pub struct Viewport {
    screen_width: usize,
    screen_height: usize,
    transform: Mat4x4,
}

impl Viewport{
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        let transform = Mat4x4 {
            mat: [
                [screen_width as f32 / 2.0, 0.0, 0.0, screen_width as f32 / 2.0],
                [0.0, -(screen_height as f32) / 2.0, 0.0, screen_height as f32 / 2.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        Self {
            screen_width,
            screen_height,
            transform,
        }
    }
}