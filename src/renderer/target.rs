use crate::renderer::{ColorRGB, FrameBuffer};

pub struct RenderTarget {
    pub framebuffer: FrameBuffer,
    pub z_buffer: Vec<f64>,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            framebuffer: FrameBuffer::new(width, height),
            z_buffer: vec![f64::INFINITY; width * height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.framebuffer = FrameBuffer::new(width, height);
        self.z_buffer = vec![f64::INFINITY; width * height];
    }

    pub fn clear(&mut self, clear_color: ColorRGB) {
        self.framebuffer.fill(clear_color);
        self.z_buffer.fill(f64::INFINITY);
    }
}
