#![allow(dead_code)]
use crate::types::math::{Mat4x4, Point3D, Vector3D};
use crate::types::primitives::Vertex;
use std::env;
use std::fs;
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
                let v0_idx = self.triangle_indices[triangle_index * 3] as usize;
                let v1_idx = self.triangle_indices[triangle_index * 3 + 1] as usize;
                let v2_idx = self.triangle_indices[triangle_index * 3 + 2] as usize;

                // Get the current vertex we're calculating normal for
                let current_vertex = &self.vertices[vertex_index];

                // Get the other two vertices (order doesn't matter as long as we're consistent)
                let other_vertex1 = &self.vertices[if vertex_index == v0_idx {
                    v1_idx
                } else if vertex_index == v1_idx {
                    v2_idx
                } else {
                    v0_idx
                }];

                let other_vertex2 = &self.vertices[if vertex_index == v0_idx {
                    v2_idx
                } else if vertex_index == v1_idx {
                    v0_idx
                } else {
                    v1_idx
                }];

                // Calculate edges from current vertex
                let edge1 = Vector3D::new(
                    other_vertex1.position[0] - current_vertex.position[0],
                    other_vertex1.position[1] - current_vertex.position[1],
                    other_vertex1.position[2] - current_vertex.position[2],
                );

                let edge2 = Vector3D::new(
                    other_vertex2.position[0] - current_vertex.position[0],
                    other_vertex2.position[1] - current_vertex.position[1],
                    other_vertex2.position[2] - current_vertex.position[2],
                );

                // Calculate triangle normal using cross product
                let triangle_normal = edge1.cross(edge2).normalize();

                // Calculate signed area for weighting
                let triangle_area = triangle_normal.length() / 2.0;

                // Calculate angle at vertex for weighting
                //theta = get angle between the two edges in the triangle it is part of
                let cos_theta = edge1.normalize().dot(edge2.normalize());
                let theta = cos_theta.acos();

                weighted_normal[0] += triangle_normal.x * triangle_area * theta;
                weighted_normal[1] += triangle_normal.y * triangle_area * theta;
                weighted_normal[2] += triangle_normal.z * triangle_area * theta;
            }

            let v_normal_vec =
                Vector3D::new(weighted_normal[0], weighted_normal[1], weighted_normal[2])
                    .normalize();

            self.vertices[vertex_index].normal = [v_normal_vec.x, v_normal_vec.y, v_normal_vec.z];
            // v_normal = normalize(Σ(area_i * theta_i * face_normal_i))
        }
    }

    pub fn create_ball(material_id: u32, color: [f32; 3]) -> Self {
        let mut mesh = Mesh::new();

        // convert raw vertex positions into vertex chunks
        for chunk in BALL_V.chunks(3) {
            let vertex = Vertex {
                position: [chunk[0], chunk[1], chunk[2]],
                normal: [0.0, 0.0, 0.0], // will be calculated later
                color,
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
        mesh.calculate_vertex_normals();
        mesh
    }

    pub fn create_cube(material_id: u32, color: [f32; 3]) -> Self {
        let mut mesh = Mesh::new();

        // convert raw vertex positions into vertex chunks
        for chunk in CUBE_V.chunks(3) {
            let vertex = Vertex {
                position: [chunk[0], chunk[1], chunk[2]],
                normal: [0.0, 0.0, 0.0], // will be calculated later
                color,
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
            mesh.add_triangle(indices, material_id);
        }
        mesh.build_adj_list();
        mesh.calculate_vertex_normals();
        mesh
    }

    pub fn create_plane(material_id: u32, color: [f32; 3]) -> Self {
        let mut mesh = Mesh::new();

        // convert raw vertex positions into vertex chunks
        for chunk in PLANE_V.chunks(3) {
            let vertex = Vertex {
                position: [chunk[0], chunk[1], chunk[2]],
                normal: [0.0, 0.0, 0.0], // will be calculated later
                color,
            };
            mesh.vertices.push(vertex);
        }

        // process triangle indices to triangles
        for triangle_indices in PLANE_F.chunks(3) {
            let indices = [
                triangle_indices[0] as u32,
                triangle_indices[1] as u32,
                triangle_indices[2] as u32,
            ];

            // Add triangle with default material (say, 0)
            mesh.add_triangle(indices, material_id);
        }
        mesh.build_adj_list();
        mesh.calculate_vertex_normals();
        mesh
    }

    pub fn load_obj(obj_path: &str, material_id: u32, color: [f32; 3]) -> Self {
        let mut mesh = Mesh::new();

        let contents = fs::read_to_string(obj_path).expect("Couldnt parse obj file {0}");

        let vertices: Vec<f32> = contents
            .lines()
            .filter(|line| line.starts_with("v"))
            .flat_map(|vertex_line| {
                vertex_line
                    .split(' ')
                    .skip(1)
                    .filter(|s| !s.is_empty())
                    .take(3)
                    .map(|s| s.parse::<f32>().expect("Failed to parse coordinate as f32"))
            })
            .collect();

        let vertex_normals: Vec<f32> = contents
            .lines()
            .filter(|line| line.starts_with("vn"))
            .flat_map(|vertex_line| {
                vertex_line
                    .split(' ')
                    .skip(1)
                    .filter(|s| !s.is_empty())
                    .take(3)
                    .map(|s| s.parse::<f32>().expect("Failed to parse coordinate as f32"))
            })
            .collect();

        let faces: Vec<u32> = contents
            .lines()
            .filter(|line| line.starts_with("f"))
            .flat_map(|vertex_line| {
                vertex_line
                    .split(' ')
                    .skip(1)
                    .filter(|s| !s.is_empty())
                    .take(3)
                    .map(|s| s.parse::<u32>().expect("Failed to parse coordinate as u32"))
            })
            .collect();

        let position_chunks = vertices.chunks_exact(3);

        let normal_chunks = vertex_normals.chunks_exact(3);

        let face_chunks = faces.chunks_exact(3);

        for (pos_slice, normal_slice) in position_chunks.zip(normal_chunks) {
            let position: [f32; 3] = pos_slice.try_into().expect("Position data error");

            let normal: [f32; 3] = normal_slice.try_into().expect("Normal data error");

            let vertex = Vertex {
                position,
                normal,
                color,
            };

            mesh.vertices.push(vertex);
        }

        // process triangle indices to triangles
        for face_slice in face_chunks {
            let indices = [face_slice[0] - 1, face_slice[1] - 1, face_slice[2] - 1];

            // Add triangle with default material (say, 0)
            mesh.add_triangle(indices, material_id);
        }

        mesh.build_adj_list();

        mesh
    }
}
