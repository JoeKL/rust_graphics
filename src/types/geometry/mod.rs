use std::sync::atomic::AtomicUsize;
static MESH_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

mod mesh;

pub use mesh::Mesh;