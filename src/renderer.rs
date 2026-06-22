#![allow(dead_code)]
mod buffer; // Frame/pixel buffer management
pub mod color;
mod core;
mod draw_command;
mod font_provider;
mod fragment;
mod frustum;
mod hud;
mod passes;
mod rasterizer; // Drawing algorithms
pub mod shader;
mod viewport; //Screen space transformations and mapping

pub use buffer::FrameBuffer;
pub use color::ColorRGB;
pub use core::Renderer;
pub use draw_command::DrawCommand;
pub use font_provider::FontProvider;
pub use fragment::Fragment;
pub use frustum::Frustum;
pub use hud::Hud;
pub use passes::{
    FacePass, RasterizerInput, RasterizerOutput, RenderPass, VertexNormalPass, VertexPass,
    WireframePass,
};
pub use rasterizer::Rasterizer;
pub use shader::{FlatShader, Material, ShadingModel};
pub use viewport::Viewport;
