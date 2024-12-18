// 8 vertices * 3 coordinates = 24 floats
pub const PLANE_V: [f32; 12] = [
    // Each line is one vertex: x, y, z
    -1.0, -1.0,  0.0,    // vertex 0
     1.0, -1.0,  0.0,    // vertex 1
     1.0,  1.0,  0.0,    // vertex 2
    -1.0,  1.0,  0.0,    // vertex 3
];

// 12 triangles * 3 indices each = 36 indices
pub const PLANE_F: [usize; 6] = [
    // Each line represents one triangle using vertex indices
    0, 2, 1,    // front 1
    0, 3, 2,    // front 2
];