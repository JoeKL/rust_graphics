use crate::types::math::Point3D;


#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],  
    // uv: [f32; 2], todo
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex{
    pub fn new(position: [f32; 3], normal: [f32; 3], color: [f32; 3]) -> Self{
        Self { position, normal, color }
    }

    pub fn to_point(self) -> Point3D {
        Point3D::new(self.position[0], self.position[1], self.position[2])
    }
}