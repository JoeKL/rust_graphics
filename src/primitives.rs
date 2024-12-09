#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: u32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let w = 1;
        Self { x, y, z, w }
    }

    pub fn dehomogen(&mut self) {
        if self.z == 0.0 {
            println!("trying to divide by zero")
        }
        self.x = self.x / -self.z as f32;
        self.y = self.y / -self.z as f32;
        self.z = -1.0;
        self.w = 1;
    }

    pub fn sub_v(&self, v: Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    pub fn sub_p(&self, p: Point) -> Vector {
        Vector::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: u32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let w = 0;
        Self { x, y, z, w }
    }

    pub fn dot(&self, v: Vector) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn add(&self, v: Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    pub fn mul(&self, s: f32) -> Vector {
        Vector::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn mulVec(&self, v: Vector) -> Vector {
        Vector::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

    pub fn cross(&self, v: Vector) -> Vector {
        let x = self.y * v.z - self.z * v.y;
        let y = self.z * v.x - self.x * v.z;
        let z = self.x * v.y - self.y * v.x;
        Vector::new(x, y, z)
    }

    pub fn norm(&self) -> f32 {
        (self.dot(*self)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let n = self.norm();
        Vector::new(self.x / n, self.y / n, self.z / n)
    }

    pub fn sub_v(&self, v: Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    pub fn sub_p(&self, p: Point) -> Vector {
        Vector::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4x4 {
    pub mat: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn new_identity() -> Mat4x4 {
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
        let mut result_mat = Mat4x4::new_identity();

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

    pub fn mul_vec(self, v: Vector) -> Point {
        let x = self.mat[0][0] * v.x
            + self.mat[0][1] * v.y
            + self.mat[0][2] * v.z
            + self.mat[0][3] * v.w as f32;
        let y = self.mat[1][0] * v.x
            + self.mat[1][1] * v.y
            + self.mat[1][2] * v.z
            + self.mat[1][3] * v.w as f32;
        let z = self.mat[2][0] * v.x
            + self.mat[2][1] * v.y
            + self.mat[2][2] * v.z
            + self.mat[2][3] * v.w as f32;
        let w = self.mat[3][0] * v.x
            + self.mat[3][1] * v.y
            + self.mat[3][2] * v.z
            + self.mat[3][3] * v.w as f32;
        let mut result_point = Point::new(x, y, z);
        result_point.w = w as u32;
        result_point
    }

    pub fn mul_point(self, v: Point) -> Point {
        let x = self.mat[0][0] * v.x
            + self.mat[0][1] * v.y
            + self.mat[0][2] * v.z
            + self.mat[0][3] * v.w as f32;
        let y = self.mat[1][0] * v.x
            + self.mat[1][1] * v.y
            + self.mat[1][2] * v.z
            + self.mat[1][3] * v.w as f32;
        let z = self.mat[2][0] * v.x
            + self.mat[2][1] * v.y
            + self.mat[2][2] * v.z
            + self.mat[2][3] * v.w as f32;
        let w = self.mat[3][0] * v.x
            + self.mat[3][1] * v.y
            + self.mat[3][2] * v.z
            + self.mat[3][3] * v.w as f32;
        let mut result_point = Point::new(x, y, z);
        result_point.w = w as u32;
        result_point
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
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    pub fn calc_triangle_area(self) -> f32 {
        let signed_area = (self.b.x - self.a.x) * (self.c.y - self.a.y)
            - (self.b.y - self.a.y) * (self.c.x - self.a.x);
        signed_area as f32 / 2.0
    }
}
