use std::sync::atomic::{AtomicUsize, Ordering};
static MESH_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

mod mesh;

pub use mesh::Mesh;