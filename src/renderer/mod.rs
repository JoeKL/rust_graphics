#![allow(dead_code)]
mod buffer; // Frame/pixel buffer management
mod core;
mod draw_command;
mod fragment;
mod rasterizer; // Drawing algorithms
mod viewport; //Screen space transformations and mapping

pub use buffer::FrameBuffer;
pub use core::Renderer;
pub use draw_command::DrawCommand;
pub use fragment::Fragment;
pub use rasterizer::Rasterizer;
pub use viewport::Viewport;

