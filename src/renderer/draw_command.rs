use crate::types::math::Mat4x4;

#[derive(Debug)]
pub struct DrawCommand {
    pub first_vertex_offset: usize, // where does the mesh begin in the vertex buffer
    pub vertex_count: usize, // how many vertices are in the mesh
    pub first_triangle_index_offset: usize,  // where do the triangle_indices start in the index buffer
    pub triangle_index_count: usize,  // how many triangle_indices are there in the mesh (N triangles = N * 3 indices)
    pub material_id: usize,  // which material does the mesh have
    pub transform: Mat4x4,   // transformation of the mesh to world coordinates
}