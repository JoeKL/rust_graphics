#[derive(Debug, Clone, Copy)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

impl ScreenPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}