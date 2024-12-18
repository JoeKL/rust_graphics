use crate::types::math::{Point3D, Vector3D};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4x4 {
    pub mat: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn identity() -> Mat4x4 {
        let mat = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        Mat4x4 { mat }
    }

    pub fn new(mat: [[f32; 4]; 4]) -> Mat4x4 {
        Mat4x4 { mat }
    }

    /// Performs matrix multiplication of two 4x4 matrices.
    ///
    /// Multiplies the current matrix (self) with another 4x4 matrix (m) and returns
    /// the resulting matrix. The multiplication is performed following the standard
    /// matrix multiplication rules where each element in the resulting matrix is
    /// the dot product of a row from the first matrix and a column from the second matrix.
    ///
    /// # Arguments
    ///
    /// * `m` - Another 4x4 matrix to multiply with the current matrix
    ///
    /// # Returns
    ///
    /// * A new Mat4x4 containing the result of the multiplication
    ///
    /// # Example
    ///
    /// ```
    /// let mat1 = Mat4x4::new(); // Create first matrix
    /// let mat2 = Mat4x4::new(); // Create second matrix
    /// let result = mat1.mulMat(mat2);
    /// ```
    pub fn mul_mat(&self, m: Mat4x4) -> Mat4x4 {
        let mut result_mat = Mat4x4::identity();

        // First row
        result_mat.mat[0][0] = self.mat[0][0] * m.mat[0][0]
            + self.mat[0][1] * m.mat[1][0]
            + self.mat[0][2] * m.mat[2][0]
            + self.mat[0][3] * m.mat[3][0];
        result_mat.mat[0][1] = self.mat[0][0] * m.mat[0][1]
            + self.mat[0][1] * m.mat[1][1]
            + self.mat[0][2] * m.mat[2][1]
            + self.mat[0][3] * m.mat[3][1];
        result_mat.mat[0][2] = self.mat[0][0] * m.mat[0][2]
            + self.mat[0][1] * m.mat[1][2]
            + self.mat[0][2] * m.mat[2][2]
            + self.mat[0][3] * m.mat[3][2];
        result_mat.mat[0][3] = self.mat[0][0] * m.mat[0][3]
            + self.mat[0][1] * m.mat[1][3]
            + self.mat[0][2] * m.mat[2][3]
            + self.mat[0][3] * m.mat[3][3];

        // Second row
        result_mat.mat[1][0] = self.mat[1][0] * m.mat[0][0]
            + self.mat[1][1] * m.mat[1][0]
            + self.mat[1][2] * m.mat[2][0]
            + self.mat[1][3] * m.mat[3][0];
        result_mat.mat[1][1] = self.mat[1][0] * m.mat[0][1]
            + self.mat[1][1] * m.mat[1][1]
            + self.mat[1][2] * m.mat[2][1]
            + self.mat[1][3] * m.mat[3][1];
        result_mat.mat[1][2] = self.mat[1][0] * m.mat[0][2]
            + self.mat[1][1] * m.mat[1][2]
            + self.mat[1][2] * m.mat[2][2]
            + self.mat[1][3] * m.mat[3][2];
        result_mat.mat[1][3] = self.mat[1][0] * m.mat[0][3]
            + self.mat[1][1] * m.mat[1][3]
            + self.mat[1][2] * m.mat[2][3]
            + self.mat[1][3] * m.mat[3][3];

        // Third row
        result_mat.mat[2][0] = self.mat[2][0] * m.mat[0][0]
            + self.mat[2][1] * m.mat[1][0]
            + self.mat[2][2] * m.mat[2][0]
            + self.mat[2][3] * m.mat[3][0];
        result_mat.mat[2][1] = self.mat[2][0] * m.mat[0][1]
            + self.mat[2][1] * m.mat[1][1]
            + self.mat[2][2] * m.mat[2][1]
            + self.mat[2][3] * m.mat[3][1];
        result_mat.mat[2][2] = self.mat[2][0] * m.mat[0][2]
            + self.mat[2][1] * m.mat[1][2]
            + self.mat[2][2] * m.mat[2][2]
            + self.mat[2][3] * m.mat[3][2];
        result_mat.mat[2][3] = self.mat[2][0] * m.mat[0][3]
            + self.mat[2][1] * m.mat[1][3]
            + self.mat[2][2] * m.mat[2][3]
            + self.mat[2][3] * m.mat[3][3];

        // Fourth row
        result_mat.mat[3][0] = self.mat[3][0] * m.mat[0][0]
            + self.mat[3][1] * m.mat[1][0]
            + self.mat[3][2] * m.mat[2][0]
            + self.mat[3][3] * m.mat[3][0];
        result_mat.mat[3][1] = self.mat[3][0] * m.mat[0][1]
            + self.mat[3][1] * m.mat[1][1]
            + self.mat[3][2] * m.mat[2][1]
            + self.mat[3][3] * m.mat[3][1];
        result_mat.mat[3][2] = self.mat[3][0] * m.mat[0][2]
            + self.mat[3][1] * m.mat[1][2]
            + self.mat[3][2] * m.mat[2][2]
            + self.mat[3][3] * m.mat[3][2];
        result_mat.mat[3][3] = self.mat[3][0] * m.mat[0][3]
            + self.mat[3][1] * m.mat[1][3]
            + self.mat[3][2] * m.mat[2][3]
            + self.mat[3][3] * m.mat[3][3];

        result_mat
    }

    pub fn mul_vec(&self, v: Vector3D) -> Vector3D {
        let x = self.mat[0][0] * v.x + self.mat[0][1] * v.y + self.mat[0][2] * v.z;

        let y = self.mat[1][0] * v.x + self.mat[1][1] * v.y + self.mat[1][2] * v.z;

        let z = self.mat[2][0] * v.x + self.mat[2][1] * v.y + self.mat[2][2] * v.z;

        Vector3D::new(x, y, z)
    }
    pub fn mul_point(&self, v: Point3D) -> Point3D {
        let x = self.mat[0][0] * v.x
            + self.mat[0][1] * v.y
            + self.mat[0][2] * v.z
            + self.mat[0][3] * v.w;

        let y = self.mat[1][0] * v.x
            + self.mat[1][1] * v.y
            + self.mat[1][2] * v.z
            + self.mat[1][3] * v.w;

        let z = self.mat[2][0] * v.x
            + self.mat[2][1] * v.y
            + self.mat[2][2] * v.z
            + self.mat[2][3] * v.w;

        let w = self.mat[3][0] * v.x
            + self.mat[3][1] * v.y
            + self.mat[3][2] * v.z
            + self.mat[3][3] * v.w;

        let mut return_p = Point3D::new(x, y, z);
        return_p.w = w;
        return_p
    }

    pub fn print(&self) {
        println!("Matrix 4x4:");
        for i in 0..4 {
            print!("[ ");
            for j in 0..4 {
                // Format with 8 characters total, 4 after decimal point
                print!("{:8.4} ", self.mat[i][j]);
            }
            println!("]");
        }
        println!(); // Empty line after matrix
    }

    // Optional: function to print with a label
    pub fn print_with_label(&self, label: &str) {
        println!("{}:", label);
        self.print();
    }

    pub(crate) fn from_translation(position: Vector3D) -> Mat4x4 {
        Mat4x4::new([
            [1.0, 0.0, 0.0, position.x],
            [0.0, 1.0, 0.0, position.y],
            [0.0, 0.0, 1.0, position.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub(crate) fn from_scale(scale: Vector3D) -> Mat4x4 {
        Mat4x4::new([
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // Helper function to get matrix minor
    fn get_minor(&self, row: usize, col: usize) -> f32 {
        let mut minor = [[0.0; 3]; 3];
        let mut m_i = 0;

        for i in 0..4 {
            if i == row {
                continue;
            }
            let mut m_j = 0;

            for j in 0..4 {
                if j == col {
                    continue;
                }
                minor[m_i][m_j] = self.mat[i][j];
                m_j += 1;
            }
            m_i += 1;
        }

        // Calculate determinant of 3x3 matrix
        minor[0][0] * (minor[1][1] * minor[2][2] - minor[1][2] * minor[2][1])
            - minor[0][1] * (minor[1][0] * minor[2][2] - minor[1][2] * minor[2][0])
            + minor[0][2] * (minor[1][0] * minor[2][1] - minor[1][1] * minor[2][0])
    }

    // Calculate cofactor
    fn get_cofactor(&self, row: usize, col: usize) -> f32 {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        sign * self.get_minor(row, col)
    }

    // Calculate determinant
    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for j in 0..4 {
            det += self.mat[0][j] * self.get_cofactor(0, j);
        }
        det
    }

    // Calculate complete inverse
    pub fn inverse(&self) -> Self {
        let det = self.determinant();

        // Check if matrix is invertible
        if det.abs() < 1e-6 {
            panic!("matrix not invertible")
        }

        let mut result = Mat4x4 { mat: [[0.0; 4]; 4] };
        let inv_det = 1.0 / det;

        // Calculate adjugate matrix and multiply by 1/determinant
        for i in 0..4 {
            for j in 0..4 {
                result.mat[j][i] = self.get_cofactor(i, j) * inv_det;
            }
        }

        result
    }
}
