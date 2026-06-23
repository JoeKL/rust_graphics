use crate::renderer::{RenderTarget, Viewport};

pub struct RenderView {
    pub name: String,
    pub camera_node_name: String,
    pub viewport: Viewport,
    pub target: RenderTarget,
    pub texture_handle: Option<egui::TextureHandle>,
}

impl RenderView {
    pub fn new(name: &str, camera_node_name: &str, width: usize, height: usize) -> Self {
        Self {
            name: name.to_string(),
            camera_node_name: camera_node_name.to_string(),
            viewport: Viewport::new(width, height),
            target: RenderTarget::new(width, height),
            texture_handle: None,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.viewport = Viewport::new(width, height);
        self.target.resize(width, height);
    }
}
