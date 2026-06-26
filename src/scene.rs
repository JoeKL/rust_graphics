#![allow(dead_code)]

pub mod camera;
pub mod geometry;
pub mod light;
pub mod primitives;
mod scene;
mod scene_node;

pub use camera::Camera;
pub use geometry::Mesh;
pub use light::PointLight;
#[allow(unused_imports)]
pub use primitives::{Triangle, Vertex};
pub use scene::Scene;
pub use scene_node::SceneNode;
