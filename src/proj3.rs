use crate::shape::Box2;
use crate::{vec3, vec4, Affine3, Matrix4, Motion3, Rotation3, Scalar, Similarity3, Vector3};

/// A projective transform in three-dimensional space.
#[repr(transparent)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Projective3(Matrix4);

impl Projective3 {
    /// The identity projective transform.
    #[inline]
    pub const fn identity() -> Self {
        Self(Matrix4::identity())
    }

    /// Constructs a projective transform from its 4x4 matrix representation.
    #[inline]
    pub const fn new(matrix: Matrix4) -> Self {
        Self(matrix)
    }

    /// Constructs a perspective transform which maps coordinates within a "view frustum" to
    /// the box `[-1, 1]³`.
    ///
    /// The view frustum consists of the points `(x, y, z)` such that `near_z <= z <= far_z`,
    /// `-z <= x <= z` and `-z <= y <= z`.
    #[inline]
    pub const fn perspective_simple(near_z: Scalar, far_z: Scalar) -> Self {
        let z_z = (far_z + near_z) / (far_z - near_z);
        let w_z = -2.0 * far_z * near_z / (far_z - near_z);
        Self(Matrix4 {
            x: vec4(1.0, 0.0, 0.0, 0.0),
            y: vec4(0.0, 1.0, 0.0, 0.0),
            z: vec4(0.0, 0.0, z_z, 1.0),
            w: vec4(0.0, 0.0, w_z, 0.0),
        })
    }

    /// Constructs a perspective transform which maps coordinates within a "view frustum" to
    /// the box `[-1, 1]³`.
    ///
    /// The view frustum consists of the points `(x, y, z)` such that `near_z <= z <= far_z` and
    /// `bounds.contains(vec2(x, y) / z)`.
    #[inline]
    pub const fn perspective_skew(near_z: Scalar, far_z: Scalar, bounds: Box2) -> Self {
        let min = bounds.min();
        let max = bounds.max();
        let x_x = 2.0 / (max.x - min.x);
        let y_y = 2.0 / (max.y - min.y);
        let z_z = (far_z + near_z) / (far_z - near_z);
        let z_x = -(max.x + min.x) / (max.x - min.x);
        let z_y = -(max.y + min.y) / (max.y - min.y);
        let w_z = -2.0 * far_z * near_z / (far_z - near_z);
        Self(Matrix4 {
            x: vec4(x_x, 0.0, 0.0, 0.0),
            y: vec4(0.0, y_y, 0.0, 0.0),
            z: vec4(z_x, z_y, z_z, 1.0),
            w: vec4(0.0, 0.0, w_z, 0.0),
        })
    }

    /// Constructs a perspective transform which maps coordinates within a "view frustum" to
    /// the box `[-1, 1]³`.
    ///
    /// The view frustum is bounded by the planes `near_z <= z <= far_z`, has an aspect ratio
    /// of `aspect_ratio`, and has an angle between the top and bottom planes of `fov_y` radians.
    #[inline]
    pub fn perspective_aspect_fov(
        near_z: Scalar,
        far_z: Scalar,
        aspect_ratio: Scalar,
        fov_y: Scalar,
    ) -> Self {
        let y_y = 1.0 / (fov_y / 2.0).tan();
        let x_x = aspect_ratio * y_y;
        let z_z = (far_z + near_z) / (far_z - near_z);
        let w_z = -2.0 * far_z * near_z / (far_z - near_z);
        Self(Matrix4 {
            x: vec4(x_x, 0.0, 0.0, 0.0),
            y: vec4(0.0, y_y, 0.0, 0.0),
            z: vec4(0.0, 0.0, z_z, 1.0),
            w: vec4(0.0, 0.0, w_z, 0.0),
        })
    }

    /// Gets the 4x4 matrix representation of this projective transform.
    #[inline]
    pub const fn as_matrix(&self) -> &Matrix4 {
        &self.0
    }
}

impl From<Rotation3> for Projective3 {
    #[inline]
    fn from(rotation: Rotation3) -> Self {
        Affine3::from(rotation).into()
    }
}

impl From<Motion3> for Projective3 {
    #[inline]
    fn from(motion: Motion3) -> Self {
        Affine3::from(motion).into()
    }
}

impl From<Similarity3> for Projective3 {
    #[inline]
    fn from(similarity: Similarity3) -> Self {
        Affine3::from(similarity).into()
    }
}

impl From<Affine3> for Projective3 {
    #[inline]
    fn from(affine: Affine3) -> Self {
        Self(Matrix4 {
            x: vec4(affine.linear.x.x, affine.linear.x.y, affine.linear.x.z, 0.0),
            y: vec4(affine.linear.y.x, affine.linear.y.y, affine.linear.y.z, 0.0),
            z: vec4(affine.linear.z.x, affine.linear.z.y, affine.linear.z.z, 0.0),
            w: vec4(affine.offset.x, affine.offset.y, affine.offset.z, 1.0),
        })
    }
}

impl_trans_mul!(Rotation3, Projective3);
impl_trans_mul!(Motion3, Projective3);
impl_trans_mul!(Similarity3, Projective3);
impl_trans_mul!(Affine3, Projective3);

impl core::ops::Mul<Projective3> for Projective3 {
    type Output = Projective3;
    #[inline]
    fn mul(self, rhs: Projective3) -> Projective3 {
        Self(self.0 * rhs.0)
    }
}

impl core::ops::Mul<Vector3> for Projective3 {
    type Output = Vector3;
    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        let r = self.0 * vec4(rhs.x, rhs.y, rhs.z, 1.0);
        vec3(r.x, r.y, r.z) / r.w
    }
}

#[test]
fn test_perspective() {
    use crate::vec2;
    let proj = Projective3::perspective_skew(
        2.0,
        16.0,
        Box2::from_min_max(vec2(-1.0, -1.0), vec2(2.0, 3.0)),
    );
    approx::assert_relative_eq!(proj * vec3(-2.0, -2.0, 2.0), vec3(-1.0, -1.0, -1.0));
    approx::assert_relative_eq!(proj * vec3(4.0, 6.0, 2.0), vec3(1.0, 1.0, -1.0));
    approx::assert_relative_eq!(proj * vec3(-16.0, -16.0, 16.0), vec3(-1.0, -1.0, 1.0));
    approx::assert_relative_eq!(proj * vec3(32.0, 48.0, 16.0), vec3(1.0, 1.0, 1.0));
}
