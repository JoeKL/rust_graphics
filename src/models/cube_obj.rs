// 8 vertices * 3 coordinates = 24 floats
pub const CUBE_V: [f32; 24] = [
    // Each line is one vertex: x, y, z
    -1.0, -1.0,  1.0,    // vertex 0
     1.0, -1.0,  1.0,    // vertex 1
     1.0,  1.0,  1.0,    // vertex 2
    -1.0,  1.0,  1.0,    // vertex 3
    -1.0, -1.0, -1.0,    // vertex 4
     1.0, -1.0, -1.0,    // vertex 5
     1.0,  1.0, -1.0,    // vertex 6
    -1.0,  1.0, -1.0     // vertex 7
];

// 12 triangles * 3 indices each = 36 indices
pub const CUBE_F: [usize; 36] = [
    // Each line represents one triangle using vertex indices
    0, 1, 2,    // front 1
    0, 2, 3,    // front 2
    1, 5, 6,    // right 1
    1, 6, 2,    // right 2
    5, 4, 7,    // back 1
    5, 7, 6,    // back 2
    4, 0, 3,    // left 1
    4, 3, 7,    // left 2
    3, 2, 6,    // top 1
    3, 6, 7,    // top 2
    4, 5, 1,    // bottom 1
    4, 1, 0     // bottom 2
];