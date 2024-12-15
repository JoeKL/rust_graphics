use crate::renderer::RenderTriangle;
use crate::types::math::{Point3D, Mat4x4};
use crate::types::primitives::{Triangle, Vertex};


use crate::models::*;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangle_indices: Vec<u32>,   // triple of indices represent a triangle [1,2,3,4,5,6] -> triangle between vertex 1,2,3 and 4,5,6
    pub triangle_normals: Vec<[f32; 3]>, // represent x:f32, y:f32, z:f32
    pub material_indices: Vec<u32>, // each index in this array represents one triangle in triangle_indices

}
impl Mesh {    
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangle_indices: Vec::new(),
            triangle_normals: Vec::new(),
            material_indices: Vec::new(),
        }
    }

    pub fn add_triangle(&mut self, indices: [u32; 3], material_id: u32){
        let triangle_idx = self.triangle_indices.len() / 3; // since 3 vertecies corrospond to one triangle
        self.triangle_indices.extend_from_slice(&indices); // extends_from_slice instead of append to not remove items from indices array

        let v0 = self.vertices[indices[0] as usize].position;
        let v1 = self.vertices[indices[1] as usize].position;
        let v2 = self.vertices[indices[2] as usize].position;

        let normal = Mesh::calculate_triangle_normal(v0,v1,v2);
        self.triangle_normals.push(normal);
        self.material_indices.push(material_id);

    }

    fn calculate_triangle_normal(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
        // Calculate vectors from v0 to v1 and v0 to v2
        let edge1 = [
            v1[0] - v0[0],
            v1[1] - v0[1],
            v1[2] - v0[2],
        ];
        let edge2 = [
            v2[0] - v0[0],
            v2[1] - v0[1],
            v2[2] - v0[2],
        ];
    
        // Cross product
        let normal = [
            edge1[1] * edge2[2] - edge1[2] * edge2[1],
            edge1[2] * edge2[0] - edge1[0] * edge2[2],
            edge1[0] * edge2[1] - edge1[1] * edge2[0],
        ];
    
        // Normalize
        let length = (normal[0] * normal[0] + 
                     normal[1] * normal[1] + 
                     normal[2] * normal[2]).sqrt();
        [
            normal[0] / length,
            normal[1] / length,
            normal[2] / length,
        ]
    }

    pub fn recalculate_face_normals(&mut self) {
        // Clear existing normals and prepare new ones
        self.triangle_normals.clear();
        self.triangle_normals.reserve(self.triangle_indices.len() / 3);

        // Process triangles in groups of 3 indices
        for triangle_indices in self.triangle_indices.chunks(3) {
            // Get vertex positions for this triangle
            let v0 = Point3D::new(
                self.vertices[triangle_indices[0] as usize].position[0],
                self.vertices[triangle_indices[0] as usize].position[1],
                self.vertices[triangle_indices[0] as usize].position[2]
            );
            let v1 = Point3D::new(
                self.vertices[triangle_indices[1] as usize].position[0],
                self.vertices[triangle_indices[1] as usize].position[1],
                self.vertices[triangle_indices[1] as usize].position[2]
            );
            let v2 = Point3D::new(
                self.vertices[triangle_indices[2] as usize].position[0],
                self.vertices[triangle_indices[2] as usize].position[1],
                self.vertices[triangle_indices[2] as usize].position[2]
            );

            // Calculate edges
            let edge1 = v1.sub_p(v0);  // Vector from v0 to v1
            let edge2 = v2.sub_p(v0);  // Vector from v0 to v2

            // Calculate normal using cross product
            let normal = edge1.cross(edge2).normalize();
            
            // Store the normal
            self.triangle_normals.push([normal.x, normal.y, normal.z]);
        }
    }

    pub fn transform(&mut self, transform: Mat4x4) {
        for vertex in &mut self.vertices {

            let vertex_point = Point3D::new(
                vertex.position[0],
                vertex.position[1],
                vertex.position[2]
            );
            
            let transformed = transform.mul_point(vertex_point);

            vertex.position = [transformed.x, transformed.y, transformed.z];
        }
        self.recalculate_face_normals(); // we need to recalulate face normals. otherwise the normals are stuck in space while the triangles are moved away
        //#TODO: Dirty Normals in Vertex when scaling applied. need to recalcc vertecies 
    }

    pub fn get_render_triangles(&self) -> Vec<RenderTriangle> {
        let mut triangles = Vec::new();

        // Process indices in groups of 3
        for triangle_idx in 0..(self.triangle_indices.len() / 3) {
            let i0 = self.triangle_indices[triangle_idx * 3] as usize;
            let i1 = self.triangle_indices[triangle_idx * 3 + 1] as usize;
            let i2 = self.triangle_indices[triangle_idx * 3 + 2] as usize;

            // Create triangle with copied vertex data
            let triangle = RenderTriangle {
                vertices: [
                    self.vertices[i0],
                    self.vertices[i1],
                    self.vertices[i2],
                ],
                normal: self.triangle_normals[triangle_idx],
                material_id: self.material_indices[triangle_idx],
            };
            
            triangles.push(triangle);
        }
        triangles
    }

    pub fn create_ball() -> Self {
        let mut mesh = Mesh::new();
        
        // convert raw vertex positions into vertex chunks
        for chunk in BALL_V.chunks(3) {
            let vertex = Vertex {
                position: [chunk[0], chunk[1], chunk[2]],
                normal: [0.0, 0.0, 0.0], // will be calculated later
                color: [1.0, 1.0, 1.0],  // Default white color
            };
            mesh.vertices.push(vertex);
        }

        // process triangle indices to triangles
        for triangle_indices in BALL_F.chunks(3) {
            let indices = [
                triangle_indices[0] as u32,
                triangle_indices[1] as u32,
                triangle_indices[2] as u32,
            ];
            
            // Add triangle with default material (say, 0)
            mesh.add_triangle(indices, 0);
        }

        mesh
    }

    pub fn create_cube() -> Self {
        let mut mesh = Mesh::new();
        
        // convert raw vertex positions into vertex chunks
        for chunk in CUBE_V.chunks(3) {
            let vertex = Vertex {
                position: [chunk[0], chunk[1], chunk[2]],
                normal: [0.0, 0.0, 0.0], // will be calculated later
                color: [1.0, 1.0, 1.0],  // Default white color
            };
            mesh.vertices.push(vertex);
        }

        // process triangle indices to triangles
        for triangle_indices in CUBE_F.chunks(3) {
            let indices = [
                triangle_indices[0] as u32,
                triangle_indices[1] as u32,
                triangle_indices[2] as u32,
            ];
            
            // Add triangle with default material (say, 0)
            mesh.add_triangle(indices, 0);
        }

        mesh
    }
}
