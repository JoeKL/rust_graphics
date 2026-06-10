#![allow(dead_code)]

mod scene_impl;
mod scene_node;
pub mod camera;
pub mod light;
pub mod geometry;
pub mod primitives;

pub use scene_impl::Scene;
pub use scene_node::SceneNode;
pub use camera::Camera;
pub use light::PointLight;
pub use geometry::Mesh;
#[allow(unused_imports)]
pub use primitives::{Vertex, Triangle};
