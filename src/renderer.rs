#![allow(dead_code)]
mod buffer; // Frame/pixel buffer management
pub mod color;
mod core;
mod draw_command;
mod fragment;
mod frustum;
mod passes;
mod rasterizer; // Drawing algorithms
pub mod shader;
mod target;
mod view;
mod viewport; //Screen space transformations and mapping

pub use buffer::{FrameBuffer, RenderTarget};
pub use color::ColorRGB;
pub use core::Renderer;
pub use draw_command::DrawCommand;
pub use fragment::Fragment;
pub use frustum::Frustum;
pub use passes::{
    FacePass, RasterizerInput, RasterizerOutput, RenderPass, VertexNormalPass, VertexPass,
    WireframePass,
};
pub use rasterizer::Rasterizer;
pub use shader::{FlatShader, Material, ShadingModel};
pub use view::RenderView;
pub use viewport::Viewport;
