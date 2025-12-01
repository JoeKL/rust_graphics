#![allow(dead_code)]
use crate::types::math::{Mat4x4, Point3D, Vector3D};
use crate::types::primitives::Vertex;
use std::fs;
use std::sync::atomic::Ordering;

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
            //wtf am i even doing with that comment. probably need to implement halfmeshes but its
            //so long ago wtf

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

    pub fn load_obj(obj_path: &str, material_id: u32, color: [f32; 3]) -> Self {
        let mut mesh = Mesh::new();

        let contents = fs::read_to_string(obj_path).expect("Couldnt parse obj file {0}");

        let vertices: Vec<f32> = contents
            .lines()
            .filter(|line| line.starts_with("v "))
            .flat_map(|vertex_line| {
                vertex_line
                    .split(' ')
                    .skip(1)
                    .filter(|s| !s.is_empty())
                    .take(3)
                    .map(|s| s.parse::<f32>().expect("Failed to parse coordinate as f32"))
            })
            .collect();

        let vertex_uv_cords: Vec<f32> = contents
            .lines()
            .filter(|line| line.starts_with("vt "))
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
            .filter(|line| line.starts_with("vn "))
            .flat_map(|vertex_line| {
                vertex_line
                    .split(' ')
                    .skip(1)
                    .filter(|s| !s.is_empty())
                    .take(3)
                    .map(|s| s.parse::<f32>().expect("Failed to parse coordinate as f32"))
            })
            .collect();

        // f v/vt/vn v/vt/vn v/vt/vn
        // returns [[Option(v), Option(vt), Option(vn)], ...]
        let raw_faces: Vec<Vec<[Option<u32>; 3]>> = contents
            .lines()
            .filter(|line| line.starts_with("f "))
            .map(|face_line| {
                face_line
                    .split_whitespace()
                    .skip(1)
                    .map(|vertex_str| {
                        let parts: Vec<&str> = vertex_str.split('/').collect();

                        // Helper: Returns Some(u32) if valid, None if empty or missing
                        let get_index = |i: usize| -> Option<u32> {
                            parts
                                .get(i)
                                .filter(|s| !s.is_empty())
                                .and_then(|s| s.parse().ok())
                        };

                        [get_index(0), get_index(1), get_index(2)]
                    })
                    .collect()
            })
            .collect();

        // Input: Vec<Vec<[v, vt, vn]>>
        // Output: Vec<[v, vt, vn]> (Flattened, every 3 is a triangle)
        let faces: Vec<[Option<u32>; 3]> = raw_faces
            .iter()
            .flat_map(|face| {
                let mut triangle_buffer = Vec::new();

                // Triangulation via Fan-Method
                if face.len() >= 3 {
                    let v0 = face[0];

                    for i in 1..face.len() - 1 {
                        let v1 = face[i];
                        let v2 = face[i + 1];

                        triangle_buffer.push(v0);
                        triangle_buffer.push(v1);
                        triangle_buffer.push(v2);
                    }
                }

                triangle_buffer.into_iter()
            })
            .collect();

        for face in faces.chunks_exact(3) {
            let start_index = mesh.vertices.len() as u32;

            let indices = [start_index, start_index + 1, start_index + 2];

            for vertex in face {
                let v_idx_obj = vertex[0].expect("Error: Face missing vertex index");
                // let vt_idx_obj = vertex[1].expect("Error: Face missing normal index") - 1;
                let vn_idx_obj = vertex[2].unwrap_or(v_idx_obj);

                let v_idx = (v_idx_obj - 1) as usize;
                let vn_idx = (vn_idx_obj - 1) as usize;

                // Calculate the "Stride" (Jump 3 floats per vertex)
                let pos_stride = v_idx * 3;
                let position: [f32; 3] = [
                    vertices[pos_stride],
                    vertices[pos_stride + 1],
                    vertices[pos_stride + 2],
                ];

                let normal_stride = vn_idx * 3;
                let normal: [f32; 3] = if normal_stride < vertex_normals.len() {
                    [
                        vertex_normals[normal_stride],
                        vertex_normals[normal_stride + 1],
                        vertex_normals[normal_stride + 2],
                    ]
                } else {
                    [0.0, 0.0, 0.0] // Fallback if file has NO normals provided
                };

                let vertex = Vertex {
                    position,
                    uv: [0.0, 0.0],
                    normal,
                    color,
                };

                mesh.vertices.push(vertex);
            }

            mesh.add_triangle(indices, material_id);
        }

        println!("obj: {:?}", obj_path);
        println!("vertices {:?}", vertices.len() / 3);
        println!("vertex normals {:?}", vertex_normals.len() / 3);
        println!("vertex uv cords {:?}", vertex_uv_cords.len() / 3);
        println!("raw faces {:?}", raw_faces.len());
        println!("triangulated faces {:?}\n", faces.len() / 3);

        mesh.build_adj_list();
        mesh
    }
}
