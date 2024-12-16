mod material;
mod model;
use std::sync::atomic::AtomicUsize;
static MATERIAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);


pub use material::Material;
pub use model::{ShadingModel, FlatShader};