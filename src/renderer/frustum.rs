
use crate::types::math::Mat4x4;


pub struct Frustum {
    
}

impl Frustum {
    // Create frustum from view-projection matrix
    pub fn from_matrix(matrix: &Mat4x4) -> Self {
        Self{}
    }

}

// howto construct frustum
// create cube corners as  Point3D in -1 , 1 in xyz and w = 1
// multiply all corners by invevrse frustum matrix to map from NDC to world space. This will result in frustum corners in worldspace
// construct all 6 planes in worldspace (left, right, top, bottom, front, back)

// after that check for each point if its in the frustum by
// somehow dot product of plane and point ?? 