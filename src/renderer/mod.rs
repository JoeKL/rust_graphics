#![allow(dead_code)]
mod buffer;     // Frame/pixel buffer management
mod rasterizer; // Drawing algorithms
mod viewport;   //Screen space transformations and mapping

pub use viewport::Viewport;
pub use buffer::FrameBuffer;
pub use rasterizer::Rasterizer;