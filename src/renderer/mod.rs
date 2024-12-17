#![allow(dead_code)]
mod buffer;     // Frame/pixel buffer management
mod rasterizer; // Drawing algorithms
mod viewport;   //Screen space transformations and mapping
mod core;
mod render_triangle;
mod frustum;
mod draw_command;

pub use viewport::Viewport;
pub use buffer::FrameBuffer;
pub use rasterizer::Rasterizer;
pub use core::Renderer;
pub use render_triangle::RenderTriangle;
pub use frustum::Frustum;
pub use draw_command::DrawCommand;