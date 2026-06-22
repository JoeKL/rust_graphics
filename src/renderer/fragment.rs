pub struct Fragment {
    // Screen position
    pub x: i32, // screen x coordinate
    pub y: i32, // screen y coordinate
    pub z: f64, // depth value for z-buffer

    // Interpolated vertex attributes
    pub color: [f64; 3],  // interpolated vertex colors
    pub normal: [f64; 3], // interpolated normal
    // pub uv: [f64; 2],            // texture coordinates (if using textures)

    // Material info
    pub material_id: usize, // which material to use
}
