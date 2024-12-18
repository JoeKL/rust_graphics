pub struct Fragment {
    // Screen position
    pub x: i32,  // screen x coordinate
    pub y: i32,  // screen y coordinate
    pub z: f32,  // depth value for z-buffer

    // Interpolated vertex attributes
    pub color: [f32; 3],      // interpolated vertex colors
    // pub normal: [f32; 3],         // interpolated normal
    // pub uv: [f32; 2],            // texture coordinates (if using textures)
    
    // Material info
    pub material_id: usize,   // which material to use

    // Optional: could also include
    // view_pos: Vec3,    // position in view space for lighting
    // world_pos: Vec3,   // position in world space
}