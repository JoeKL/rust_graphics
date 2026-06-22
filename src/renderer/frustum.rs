use crate::math::{Mat4x4, Point3D, Vector3D};
use crate::scene::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    point: Point3D,
    normal: Vector3D,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            point: Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            normal: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0,
            },
        }
    }
}

impl Plane {
    pub fn new(p0: Point3D, p1: Point3D, p2: Point3D) -> Self {
        let p01 = p1 - p0;
        let p02 = p2 - p0;
        let plane_normal = p01.cross(p02);

        Self {
            point: p0,
            normal: plane_normal.normalize(),
        }
    }

    pub fn distance(&self, p0: Point3D) -> f64 {
        // Calculate the distance using the point-plane distance formula
        // d = n·(p0 - p) where "normal" is normal vector, "point" is the point,
        // and p0 is any point in the plane

        // if return value is -1, its behind the plane
        // if return value is 1, its infront the plane
        // if return value is 0, its on the plane
        self.normal.dot(p0 - self.point)
    }
}

pub struct Frustum {
    planes: [Plane; 6],
}

impl Default for Frustum {
    fn default() -> Self {
        Self::new()
    }
}

impl Frustum {
    pub fn new() -> Frustum {
        Self {
            planes: [Plane::default(); 6],
        }
    }

    // Create frustum from view-projection matrix
    pub fn from_matrix(matrix: &Mat4x4) -> Self {
        // howto construct frustum

        // create cube corners as  Point3D in -1 , 1 in xyz and w = 1

        let p000 = Point3D::new(-1.0, -1.0, -1.0);
        let p001 = Point3D::new(-1.0, -1.0, 1.0);
        let p010 = Point3D::new(-1.0, 1.0, -1.0);
        let p011 = Point3D::new(-1.0, 1.0, 1.0);

        let p100 = Point3D::new(1.0, -1.0, -1.0);
        let p101 = Point3D::new(1.0, -1.0, 1.0);
        let p110 = Point3D::new(1.0, 1.0, -1.0);
        let p111 = Point3D::new(1.0, 1.0, 1.0);

        // multiply all corners by invevrse frustum matrix to map from NDC to world space. This will result in frustum corners in worldspace

        let inverse_matrix = matrix.inverse();

        let mut frustum_p000 = inverse_matrix * p000;
        let mut frustum_p001 = inverse_matrix * p001;
        let mut frustum_p010 = inverse_matrix * p010;
        let mut frustum_p011 = inverse_matrix * p011;

        let mut frustum_p100 = inverse_matrix * p100;
        let mut frustum_p101 = inverse_matrix * p101;
        let mut frustum_p110 = inverse_matrix * p110;
        let mut frustum_p111 = inverse_matrix * p111;

        frustum_p000.dehomogen();
        frustum_p001.dehomogen();
        frustum_p010.dehomogen();
        frustum_p011.dehomogen();

        frustum_p100.dehomogen();
        frustum_p101.dehomogen();
        frustum_p110.dehomogen();
        frustum_p111.dehomogen();

        // construct all 6 planes in worldspace (left, right, top, bottom, front, back)
        let planes = [
            //left -x
            Plane::new(frustum_p001, frustum_p000, frustum_p010),
            //right +x
            Plane::new(frustum_p101, frustum_p111, frustum_p100),
            //top +y
            Plane::new(frustum_p111, frustum_p011, frustum_p110),
            //bottom -y
            Plane::new(frustum_p101, frustum_p000, frustum_p001),
            //back +z
            Plane::new(frustum_p001, frustum_p011, frustum_p101),
            //front -z
            Plane::new(frustum_p000, frustum_p100, frustum_p010),
        ];

        //all normals placed such that they are facing outwards

        Self { planes }
    }

    pub fn point_in_bounds(&self, point: Point3D) -> bool {
        for plane in &self.planes {
            // if outside of plane (not inside viewing frustum)
            if plane.distance(point) >= 0.0 {
                return false;
            }
        }
        true
    }

    pub fn triangle_in_bounds(&self, triangle_vertices: [&Vertex; 3]) -> bool {
        triangle_vertices
            .iter()
            .any(|vertex| self.point_in_bounds(Point3D::from_array(vertex.position)))
    }

    pub fn triangle_in_bounds_conservative(&self, triangle_vertices: [&Vertex; 3]) -> bool {
        triangle_vertices
            .iter()
            .all(|vertex| self.point_in_bounds(Point3D::from_array(vertex.position)))
    }
}
