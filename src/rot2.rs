use crate::{vec2, Scalar, Vector2, Matrix2};

/// A rotational transform in two-dimensional space.
#[repr(transparent)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Rotation2 {
    /// The tangent of half the angle of the rotation (positive values correspond to
    /// counter-clockwise rotations).
    ///
    /// This representation has several advantages over more popular rotation representations:
    ///  * It requires only one scalar value.
    ///  * Unlike angle or complex number representations, it does not require normalization to
    ///    to preserve precision/accuracy after composing many rotations.
    ///  * No need to evaluate transcendental functions to apply or compose rotations.
    ///  * Precision is fairly uniform across the entire range of possible rotations.
    tan_half_angle: Scalar,
}

impl Rotation2 {
    /// The maximum absolute value for a `tan_half_angle` value that can be handled using general
    /// logic.
    const NORMAL_THRESHOLD: Scalar = 1e18;

    /// The identity rotation.
    pub const IDENTITY: Rotation2 = Rotation2 {
        tan_half_angle: 0.0,
    };

    /// A rotation which rotates counter-clockwise by 90 degrees.
    pub const CCW_90: Rotation2 = Rotation2 {
        tan_half_angle: 1.0,
    };

    /// A rotation which rotates clockwise by 90 degrees.
    pub const CW_90: Rotation2 = Rotation2 {
        tan_half_angle: -1.0,
    };

    /// A rotation which rotates by 180 degrees.
    pub const FLIP: Rotation2 = Rotation2 {
        tan_half_angle: Scalar::INFINITY,
    };

    /// Constructs a rotation which rotates counter-clockwise by the given angle, in radians.
    pub fn from_angle(angle: Scalar) -> Self {
        Self {
            tan_half_angle: (angle / 2.0).tan(),
        }
    }

    /// Constructs a rotation which rotates `vec2(1.0, 0.0)` to the given target direction.
    pub fn from_dir(dir: Vector2) -> Self {
        Self::from_angle(Vector2::angle_between(&vec2(1.0, 0.0), &dir))
    }

    /// Gets the "inverse" of this rotation, which rotates by the same amount in the opposite
    /// direction.
    pub const fn inverse(&self) -> Self {
        Self {
            tan_half_angle: -self.tan_half_angle,
        }
    }

    /// Computes the `sin` and `cos` of the angle for this rotation.
    pub fn angle_sin_cos(&self) -> (Scalar, Scalar) {
        let x = self.tan_half_angle;
        if x.abs() < Self::NORMAL_THRESHOLD {
            let x_sqr = x * x;
            let y = 1.0 / (1.0 + x_sqr);
            (2.0 * x * y, (1.0 - x_sqr) * y)
        } else {
            (2.0 / x, -1.0)
        }
    }
}

impl Default for Rotation2 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl core::ops::Mul<Rotation2> for Rotation2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let x = self.tan_half_angle;
        let y = rhs.tan_half_angle;
        if x.abs() <= Self::NORMAL_THRESHOLD {
            if y.abs() <= Self::NORMAL_THRESHOLD {
                Self {
                    tan_half_angle: (x + y) / (1.0 - x * y),
                }
            } else {
                Self {
                    tan_half_angle: 1.0 / (1.0 / y - x),
                }
            }
        } else if y.abs() <= Self::NORMAL_THRESHOLD {
            Self {
                tan_half_angle: 1.0 / (1.0 / x - y),
            }
        } else {
            Self {
                tan_half_angle: (-1.0 / x) + (-1.0 / y),
            }
        }
    }
}

impl core::ops::Mul<Vector2> for Rotation2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        let (sin, cos) = self.angle_sin_cos();
        vec2(cos * rhs.x - sin * rhs.y, sin * rhs.x + cos * rhs.y)
    }
}

impl From<Rotation2> for Matrix2 {
    fn from(rotation: Rotation2) -> Matrix2 {
        let (sin, cos) = rotation.angle_sin_cos();
        Matrix2 {
            x: vec2(cos, sin),
            y: vec2(-sin, cos),
        }
    }
}

#[test]
fn test_compose() {
    let mut angle = 1.0f32;
    let mut rot = Rotation2::from_angle(angle);
    let delta_angle = 1.2f32;
    let delta_rot = Rotation2::from_angle(delta_angle);
    for _ in 0..100 {
        angle += delta_angle;
        rot = rot * delta_rot;
        let vec = vec2(angle.cos(), angle.sin());
        let test_vec = rot * vec2(1.0, 0.0);
        approx::assert_relative_eq!(vec, test_vec, epsilon = 0.001);
    }
}

#[test]
fn test_consts() {
    let vec = vec2(1.0, 0.2);
    approx::assert_relative_eq!(Rotation2::CCW_90 * vec, vec2(-0.2, 1.0));
    approx::assert_relative_eq!(Rotation2::CW_90 * vec, vec2(0.2, -1.0));
    approx::assert_relative_eq!(Rotation2::FLIP * vec, vec2(-1.0, -0.2));
}

#[test]
fn test_into_matrix() {
    let rot = Rotation2::from_angle(0.5);
    let mat: Matrix2 = rot.into();
    let vec = vec2(0.7, 0.3);
    approx::assert_relative_eq!(mat * vec, rot * vec);
    approx::assert_relative_eq!(mat.inverse() * vec, rot.inverse() * vec);
}