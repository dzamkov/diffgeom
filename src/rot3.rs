use crate::{vec3, Matrix3, Rotation2, Scalar, Vector3};

/// A rotation in three-dimensional space.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Rotation3 {
    /// The vector part of the quaternion.
    x_y_z: Vector3,

    /// The scalar part of the quaternion.
    w: Scalar,
}

impl Rotation3 {
    /// Constructs a [`Rotation3`] from its quaternion components.
    ///
    /// This assumes that the input is normalized, i.e. `w² + x² + y² + z² = 1`.
    #[inline]
    pub const fn new_unchecked(w: Scalar, x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self {
            x_y_z: vec3(x, y, z),
            w,
        }
    }

    /// The identity rotation.
    pub const IDENTITY: Rotation3 = Self::new_unchecked(1.0, 0.0, 0.0, 0.0);

    /// Constructs a rotation which applies a two-dimensional rotation about the given axis,
    /// following the right-hand rule.
    ///
    /// The axis is assumed to be a unit vector.
    pub fn about(axis: Vector3, amount: Rotation2) -> Self {
        let (sin, cos) = amount.angle_sin_cos();
        let (h_sin, h_cos) = if cos > 0.0 {
            let h_cos = ((1.0 + cos) / 2.0).sqrt();
            let h_sin = sin * h_cos / (1.0 + cos);
            (h_sin, h_cos)
        } else {
            let h_sin = ((1.0 - cos) / 2.0).sqrt();
            let h_cos = sin * h_sin / (1.0 - cos);
            (h_sin, h_cos)
        };
        Self {
            x_y_z: axis * h_sin,
            w: h_cos,
        }
    }

    /// Constructs a rotation about the given vector with an angle which is equal to its magnitude,
    /// in radians, following the right-hand rule.
    #[inline]
    pub fn from_euler(vec: Vector3) -> Self {
        let len = vec.norm();
        Self::about(vec / len, Rotation2::from_angle(len))
    }

    /// Assuming the given matrix is a rotation, constructs a [`Rotation3`] from it.
    ///
    /// This is forgiving to small numerical errors in the input matrix.
    pub fn from_matrix(matrix: Matrix3) -> Self {
        let trace = matrix.x.x + matrix.y.y + matrix.z.z;
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt();
            let inv_s = 0.5 / s;
            Self::new_unchecked(
                0.5 * s,
                (matrix.y.z - matrix.z.y) * inv_s,
                (matrix.z.x - matrix.x.z) * inv_s,
                (matrix.x.y - matrix.y.x) * inv_s,
            )
        } else if matrix.x.x >= matrix.y.y && matrix.x.x >= matrix.z.z {
            let s = (1.0 + matrix.x.x - matrix.y.y - matrix.z.z).sqrt();
            let inv_s = 0.5 / s;
            Self::new_unchecked(
                (matrix.y.z - matrix.z.y) * inv_s,
                0.5 * s,
                (matrix.x.y + matrix.y.x) * inv_s,
                (matrix.z.x + matrix.x.z) * inv_s,
            )
        } else if matrix.y.y > matrix.z.z {
            let s = (1.0 + matrix.y.y - matrix.z.z - matrix.x.x).sqrt();
            let inv_s = 0.5 / s;
            Self::new_unchecked(
                (matrix.z.x - matrix.x.z) * inv_s,
                (matrix.x.y + matrix.y.x) * inv_s,
                0.5 * s,
                (matrix.y.z + matrix.z.y) * inv_s,
            )
        } else {
            let s = (1.0 + matrix.z.z - matrix.x.x - matrix.y.y).sqrt();
            let inv_s = 0.5 / s;
            Self::new_unchecked(
                (matrix.x.y - matrix.y.x) * inv_s,
                (matrix.z.x + matrix.x.z) * inv_s,
                (matrix.y.z + matrix.z.y) * inv_s,
                0.5 * s,
            )
        }
    }

    /// Gets the inverse of this rotation.
    #[inline]
    pub fn inverse(&self) -> Self {
        Self {
            x_y_z: -self.x_y_z,
            w: self.w,
        }
    }
}

impl Default for Rotation3 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl core::ops::Mul<Rotation3> for Rotation3 {
    type Output = Rotation3;
    fn mul(self, rhs: Rotation3) -> Rotation3 {
        let w = self.w * rhs.w - self.x_y_z.dot(&rhs.x_y_z);
        let x_y_z = self.w * rhs.x_y_z + rhs.w * self.x_y_z + self.x_y_z.cross(&rhs.x_y_z);

        // Use a polynomial approximation for square root centered around 1, since a full square
        // root for normalization is overkill
        let norm_sqr = w * w + x_y_z.norm_squared();
        let i_norm = 2.0 / (1.0 + norm_sqr);
        Self {
            x_y_z: x_y_z * i_norm,
            w: w * i_norm,
        }
    }
}

impl std::ops::Mul<Vector3> for Rotation3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        // TODO: Better implementation
        Matrix3::from(self) * rhs
    }
}

impl From<Rotation3> for Matrix3 {
    fn from(rot: Rotation3) -> Matrix3 {
        let wx2 = 2.0 * rot.w * rot.x_y_z.x;
        let wy2 = 2.0 * rot.w * rot.x_y_z.y;
        let wz2 = 2.0 * rot.w * rot.x_y_z.z;
        let xx2 = 2.0 * rot.x_y_z.x * rot.x_y_z.x;
        let xy2 = 2.0 * rot.x_y_z.x * rot.x_y_z.y;
        let xz2 = 2.0 * rot.x_y_z.x * rot.x_y_z.z;
        let yy2 = 2.0 * rot.x_y_z.y * rot.x_y_z.y;
        let yz2 = 2.0 * rot.x_y_z.y * rot.x_y_z.z;
        let zz2 = 2.0 * rot.x_y_z.z * rot.x_y_z.z;
        Matrix3 {
            x: vec3(1.0 - yy2 - zz2, xy2 + wz2, xz2 - wy2),
            y: vec3(xy2 - wz2, 1.0 - xx2 - zz2, yz2 + wx2),
            z: vec3(xz2 + wy2, yz2 - wx2, 1.0 - xx2 - yy2),
        }
    }
}

impl approx::AbsDiffEq for Rotation3 {
    type Epsilon = <Scalar as approx::AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        Scalar::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x_y_z.abs_diff_eq(&other.x_y_z, epsilon) && self.w.abs_diff_eq(&other.w, epsilon)
    }
}

impl approx::RelativeEq for Rotation3 {
    fn default_max_relative() -> Self::Epsilon {
        Scalar::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.x_y_z.relative_eq(&other.x_y_z, epsilon, max_relative)
            && self.w.relative_eq(&other.w, epsilon, max_relative)
    }
}

#[test]
fn test_compose_1() {
    approx::assert_abs_diff_eq!(
        Rotation3::about(vec3(0.0, 0.0, 1.0), Rotation2::CCW_90) * vec3(1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0)
    );
    approx::assert_abs_diff_eq!(
        Rotation3::about(vec3(1.0, 0.0, 0.0), Rotation2::CCW_90) * vec3(0.0, 0.0, 1.0),
        vec3(0.0, -1.0, 0.0)
    );
    approx::assert_abs_diff_eq!(
        Rotation3::about(vec3(1.0, 0.0, 0.0), Rotation2::CW_90) * vec3(0.0, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0)
    );
    approx::assert_abs_diff_eq!(
        Rotation3::about(vec3(1.0, 0.0, 0.0), Rotation2::FLIP) * vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, -1.0)
    );
}

#[test]
fn test_compose_2() {
    let a = Rotation3::about(vec3(1.0, 0.0, 0.0), Rotation2::from_angle(1.0));
    let b = Rotation3::about(vec3(0.0, 1.0, 0.0), Rotation2::from_angle(1.5));
    let c = Rotation3::about(vec3(0.0, 0.0, 1.0), Rotation2::from_angle(0.5));
    let x = vec3(5.0, 7.0, 11.0);
    approx::assert_abs_diff_eq!(a * b * c * x, a * (b * (c * x)), epsilon = 1e-5);
}

#[test]
fn test_matrix_roundtrip() {
    let rot = Rotation3::from_euler(vec3(1.0, 2.0, 3.0));
    let mat: Matrix3 = rot.into();
    approx::assert_abs_diff_eq!(rot, Rotation3::from_matrix(mat), epsilon = 1e-6);
}