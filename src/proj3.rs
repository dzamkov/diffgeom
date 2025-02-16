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