use crate::renderer::RenderTriangle;
use crate::types::math::{Mat4x4, Point3D, Vector3D};
use crate::types::primitives::Vertex;
use std::sync::atomic::Ordering;

use crate::models::*;

use super::MESH_ID_COUNTER;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub id: usize,
    pub vertices: Vec<Vertex>,
    pub triangle_indices: Vec<u32>, // triple of indices represent a triangle [1,2,3,4,5,6] -> triangle between vertex 1,2,3 and 4,5,6
    pub material_indices: Vec<u32>, // each index in this array represents one triangle in triangle_indices
    pub vertex_triangle_adj_list: Vec<Vec<usize>>, // 1:[721, 733, 744] //vertex_index:[triangle_index, triangle_index, triangle_index]
}
impl Mesh {
    pub fn new() -> Self {
        let id = MESH_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            vertices: Vec::new(),
            triangle_indices: Vec::new(),
            material_indices: Vec::new(),
            vertex_triangle_adj_list: Vec::new(),
        }
    }

    pub fn build_adj_list(&mut self) {
        self.vertex_triangle_adj_list = vec![Vec::new(); self.vertices.len()]; // correctly initialize it since the amount of vertecies is now known

        let current_triangles = self.triangle_indices.chunks(3).enumerate(); // builds a list of chunks and enumerates it

        for (triangle_index, vertex_index) in current_triangles {
            // triangle:
            // (
            //     797, //triangle_index
            //     [
            //         197, //vertex_index a
            //         124, //vertex_index b
            //         125, //vertex_index c
            //     ],
            // )

            self.vertex_triangle_adj_list[vertex_index[0] as usize].push(triangle_index);
            self.vertex_triangle_adj_list[vertex_index[1] as usize].push(triangle_index);
            self.vertex_triangle_adj_list[vertex_index[2] as usize].push(triangle_index);
        }
    }

    pub fn add_triangle(&mut self, indices: [u32; 3], material_id: u32) {
        for &index in &indices {
            if index as usize >= self.vertices.len() {
                panic!(
                    "Invalid vertex index: {}. Mesh only has {} vertices",
                    index,
                    self.vertices.len()
                );
            }
        }

        self.triangle_indices.extend_from_slice(&indices); // extends_from_slice instead of append to not remove items from indices array
        self.material_indices.push(material_id);
    }

    pub fn transform(&mut self, transform: Mat4x4) {
        for vertex in &mut self.vertices {
            let vertex_point =
                Point3D::new(vertex.position[0], vertex.position[1], vertex.position[2]);

            let transformed = transform.mul_point(vertex_point);

            vertex.position = [transformed.x, transformed.y, transformed.z];
        }

        //#TODO: Dirty Normals in Vertex when scaling applied. need to recalc vertecies
        //WARNING: WHEN SCALING IS NOT ISO IT WILL LEAD TO INCORRECT WARNINGS. WE NEED TO RECALCULATE THEN
        //transform normals

        self.calculate_vertex_normals();
    }

    pub fn calculate_vertex_normals(&mut self) {
        //vertex_index:[triangle_index, triangle_index, triangle_index]
        // 1:[721, 733, 744]
        for (vertex_index, vertex_entry) in self.vertex_triangle_adj_list.iter().enumerate() {
            let mut weighted_normal: [f32; 3] = [0.0, 0.0, 0.0];

            //https://github.com/vijaiaeroastro/HalfMesh/tree/master/include ?? :(

            for triangle_index in vertex_entry {
                let mut v0_idx = self.triangle_indices[triangle_index * 3] as usize;
                let mut v1_idx = self.triangle_indices[triangle_index * 3 + 1] as usize;
                let mut v2_idx = self.triangle_indices[triangle_index * 3 + 2] as usize;

                //check which one is the one were focusing on vertex_index == v0 || v1 || v2
                if vertex_index == v1_idx {
                    // rotate to make v1 become v0
                    // rotate once left [v1, v2, v0]

                    let temp = v1_idx; // Save v1 (your focus vertex)
                    v1_idx = v2_idx; // Move v2 to middle position
                    v2_idx = v0_idx; // Move v0 to last position
                    v0_idx = temp; // Put your focus vertex (original v1) in first position
                } else if vertex_index == v2_idx {
                    // rotate to make v2 become v0
                    let temp = v2_idx; // Save v2 (your focus vertex)
                    v2_idx = v1_idx; // Move v1 to last position
                    v1_idx = v0_idx; // Move v0 to middle position
                    v0_idx = temp; // Put your focus vertex (original v2) in first position
                }

                // triangle is made of these vertecies:
                let v0 = &self.vertices[v0_idx];
                let v1 = &self.vertices[v1_idx];
                let v2 = &self.vertices[v2_idx];

                // Calculate triangle normal using cross product of two edges
                let edge1 = Vector3D::new(
                    v1.position[0] - v0.position[0],
                    v1.position[1] - v0.position[1],
                    v1.position[2] - v0.position[2],
                );

                let edge2 = Vector3D::new(
                    v2.position[0] - v0.position[0],
                    v2.position[1] - v0.position[1],
                    v2.position[2] - v0.position[2],
                );

                // Calculate triangle normal using cross product
                let triangle_normal = edge1.cross(edge2).normalize();

                // Calculate signed area for weighting
                let signed_area = ((v1.position[0] - v0.position[0])
                    * (v2.position[1] - v0.position[1])
                    - (v1.position[1] - v0.position[1]) * (v2.position[0] - v0.position[0]))
                    / 2.0;

                // Calculate angle at vertex for weighting
                //theta = get angle between the two edges in the triangle it is part of
                let cos_theta = edge1.normalize().dot(edge2.normalize());
                let theta = cos_theta.acos();

                weighted_normal[0] += triangle_normal.x * signed_area * theta;
                weighted_normal[1] += triangle_normal.y * signed_area * theta;
                weighted_normal[2] += triangle_normal.z * signed_area * theta;
            }

            let v_normal_vec =
                Vector3D::new(weighted_normal[0], weighted_normal[1], weighted_normal[2])
                    .normalize();

            self.vertices[vertex_index].normal = [v_normal_vec.x, v_normal_vec.y, v_normal_vec.z];
            // v_normal = normalize(Σ(area_i * theta_i * face_normal_i))
        }
    }

    pub fn construct_triangles(
        vertices: Vec<(usize, Vec<Vertex>)>,
        mesh_refs: Vec<&Mesh>,
    ) -> Vec<RenderTriangle> {
        let mut triangles = Vec::new();

        for (mesh_id, mesh_ref) in mesh_refs.iter().enumerate() {
            if let Some(mesh_vertices) = vertices.iter().find(|(id, _)| *id == mesh_id) {
                // Now we have the correct vertices for this mesh
                let transformed_vertices = &mesh_vertices.1;
                // Process indices in groups of 3
                for triangle_idx in 0..(mesh_ref.triangle_indices.len() / 3) {
                    let i0 = mesh_ref.triangle_indices[triangle_idx * 3] as usize;
                    let i1 = mesh_ref.triangle_indices[triangle_idx * 3 + 1] as usize;
                    let i2 = mesh_ref.triangle_indices[triangle_idx * 3 + 2] as usize;

                    // Get the transformed vertices using our indices
                    let v0 = &transformed_vertices[i0];
                    let v1 = &transformed_vertices[i1];
                    let v2 = &transformed_vertices[i2];

                    // Calculate triangle normal using cross product of two edges
                    let edge1 = Vector3D::new(
                        v1.position[0] - v0.position[0],
                        v1.position[1] - v0.position[1],
                        v1.position[2] - v0.position[2],
                    );

                    let edge2 = Vector3D::new(
                        v2.position[0] - v0.position[0],
                        v2.position[1] - v0.position[1],
                        v2.position[2] - v0.position[2],
                    );

                    // Calculate triangle normal using cross product
                    let triangle_normal = edge1.cross(edge2).normalize();

                    // Create triangle with copied vertex data
                    let triangle = RenderTriangle {
                        vertices: [
                            vertices[mesh_id].1[i0],
                            vertices[mesh_id].1[i1],
                            vertices[mesh_id].1[i2],
                        ],
                        normal: [triangle_normal.x, triangle_normal.y, triangle_normal.z],
                        material_id: mesh_ref.material_indices[triangle_idx],
                    };
                    triangles.push(triangle);
                }
            }
        }
        triangles
    }

    pub fn create_ball(material_id: u32) -> Self {
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
            mesh.add_triangle(indices, material_id);
        }
        mesh.build_adj_list();
        mesh
    }

    // pub fn create_cube() -> Self {
    //     let mut mesh = Mesh::new();

    //     // convert raw vertex positions into vertex chunks
    //     for chunk in CUBE_V.chunks(3) {
    //         let vertex = Vertex {
    //             position: [chunk[0], chunk[1], chunk[2]],
    //             normal: [0.0, 0.0, 0.0], // will be calculated later
    //             color: [1.0, 1.0, 1.0],  // Default white color
    //         };
    //         mesh.vertices.push(vertex);
    //     }

    //     // process triangle indices to triangles
    //     for triangle_indices in CUBE_F.chunks(3) {
    //         let indices = [
    //             triangle_indices[0] as u32,
    //             triangle_indices[1] as u32,
    //             triangle_indices[2] as u32,
    //         ];

    //         // Add triangle with default material (say, 0)
    //         mesh.add_triangle(indices, 0);
    //     }
    //     mesh.build_adj_list();
    //     mesh
    // }
}
