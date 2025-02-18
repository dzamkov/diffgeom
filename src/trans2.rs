use crate::{vec2, Matrix2, Rotation2, Scalar, Vector2};

/// A transform in two-dimensional space consisting of rotation and translation.
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Motion2 {
    /// The rotation component of this transform, applied before translation.
    pub rotation: Rotation2,

    /// The offset for the translation component of this transform, applied after rotation.
    pub offset: Vector2,
}

impl Motion2 {
    /// The identity motion.
    pub const fn identity() -> Self {
        Self {
            rotation: Rotation2::IDENTITY,
            offset: vec2(0.0, 0.0),
        }
    }

    /// Constructs a motion which translates by the given offset.
    pub const fn translate(offset: Vector2) -> Self {
        Self {
            rotation: Rotation2::IDENTITY,
            offset,
        }
    }

    /// Gets the inverse of this motion.
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.inverse();
        Self {
            rotation,
            offset: rotation * -self.offset,
        }
    }

    /// Gets the linear component of this motion.
    pub const fn linear(&self) -> Rotation2 {
        self.rotation
    }
}

impl From<Rotation2> for Motion2 {
    fn from(rotation: Rotation2) -> Motion2 {
        Motion2 {
            rotation,
            offset: vec2(0.0, 0.0),
        }
    }
}

impl_trans_mul!(Rotation2, Motion2);

impl core::ops::Mul<Motion2> for Motion2 {
    type Output = Motion2;
    fn mul(self, rhs: Motion2) -> Motion2 {
        Motion2 {
            rotation: self.rotation * rhs.rotation,
            offset: self.rotation * rhs.offset + self.offset,
        }
    }
}

impl core::ops::Mul<Vector2> for Motion2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.rotation * rhs + self.offset
    }
}

/// A transform in two-dimensional space consisting of rotation, translation and uniform scaling.
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Similarity2 {
    /// The rotation component of this transform, applied before translation.
    pub rotation: Rotation2,

    /// The scaling component of this transform, applied before translation.
    pub scaling: Scalar,

    /// The offset for the translation component of this transform, applied after rotation and
    /// scaling.
    pub offset: Vector2,
}

impl Similarity2 {
    /// The identity similarity.
    pub const fn identity() -> Self {
        Self {
            rotation: Rotation2::IDENTITY,
            scaling: 1.0,
            offset: vec2(0.0, 0.0),
        }
    }

    /// Constructs a similarity which translates by the given offset.
    pub const fn translate(offset: Vector2) -> Self {
        Self {
            rotation: Rotation2::IDENTITY,
            scaling: 1.0,
            offset,
        }
    }

    /// Constructs a similarity which scales by the given factor.
    pub const fn scale(scaling: Scalar) -> Self {
        Self {
            rotation: Rotation2::IDENTITY,
            scaling,
            offset: vec2(0.0, 0.0),
        }
    }

    /// Gets the inverse of this similarity.
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.inverse();
        let scaling = 1.0 / self.scaling;
        Self {
            rotation,
            scaling,
            offset: rotation * (-self.offset * scaling),
        }
    }

    /// Gets the linear component (consisting of rotation and scaling) for this similarity.
    pub fn linear(&self) -> Matrix2 {
        Matrix2::from(self.rotation) * self.scaling
    }
}

impl From<Rotation2> for Similarity2 {
    fn from(rotation: Rotation2) -> Similarity2 {
        Similarity2 {
            rotation,
            scaling: 1.0,
            offset: vec2(0.0, 0.0),
        }
    }
}

impl From<Motion2> for Similarity2 {
    fn from(motion: Motion2) -> Similarity2 {
        Similarity2 {
            rotation: motion.rotation,
            scaling: 1.0,
            offset: motion.offset,
        }
    }
}

impl_trans_mul!(Rotation2, Similarity2);
impl_trans_mul!(Motion2, Similarity2);

impl core::ops::Mul<Similarity2> for Similarity2 {
    type Output = Similarity2;
    fn mul(self, rhs: Similarity2) -> Similarity2 {
        Similarity2 {
            rotation: self.rotation * rhs.rotation,
            scaling: self.scaling * rhs.scaling,
            offset: self.rotation * (rhs.offset * self.scaling) + self.offset,
        }
    }
}

impl core::ops::Mul<Vector2> for Similarity2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.rotation * (rhs * self.scaling) + self.offset
    }
}

/// An affine transform in two-dimensional space.
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Affine2 {
    /// The linear component of this transform, applied before translation.
    pub linear: Matrix2,

    /// The offset for the translation component of this transform, applied after the linear
    /// component.
    pub offset: Vector2,
}

impl Affine2 {
    /// The identity transform.
    pub const fn identity() -> Self {
        Self {
            linear: Matrix2::identity(),
            offset: vec2(0.0, 0.0),
        }
    }

    /// Constructs an affine transform which translates by the given offset.
    pub fn translate(offset: Vector2) -> Self {
        Self {
            linear: Matrix2::identity(),
            offset,
        }
    }

    /// Constructs an affine transform which scales non-uniformly by the given factors.
    pub fn scale(x: Scalar, y: Scalar) -> Self {
        Self {
            linear: Matrix2 {
                x: vec2(x, 0.0),
                y: vec2(0.0, y),
            },
            offset: vec2(0.0, 0.0),
        }
    }

    /// Gets the inverse of this affine transform.
    pub fn inverse(&self) -> Self {
        let linear = self.linear.inverse();
        Self {
            linear,
            offset: linear * (-self.offset),
        }
    }
}

impl From<Rotation2> for Affine2 {
    fn from(rotation: Rotation2) -> Affine2 {
        Affine2 {
            linear: Matrix2::from(rotation),
            offset: vec2(0.0, 0.0),
        }
    }
}

impl From<Motion2> for Affine2 {
    fn from(motion: Motion2) -> Affine2 {
        Affine2 {
            linear: Matrix2::from(motion.rotation),
            offset: motion.offset,
        }
    }
}

impl From<Similarity2> for Affine2 {
    fn from(similarity: Similarity2) -> Affine2 {
        Affine2 {
            linear: similarity.linear(),
            offset: similarity.offset,
        }
    }
}

impl_trans_mul!(Rotation2, Affine2);
impl_trans_mul!(Motion2, Affine2);
impl_trans_mul!(Similarity2, Affine2);

impl core::ops::Mul<Affine2> for Affine2 {
    type Output = Affine2;
    fn mul(self, rhs: Affine2) -> Affine2 {
        Affine2 {
            linear: self.linear * rhs.linear,
            offset: self.linear * rhs.offset + self.offset,
        }
    }
}

impl core::ops::Mul<Vector2> for Affine2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.linear * rhs + self.offset
    }
}

#[test]
fn test_affine_compose() {
    let a = Affine2::from(Similarity2 {
        rotation: Rotation2::from_angle(1.0),
        scaling: 2.0,
        offset: vec2(1.0, 2.0),
    });
    let b = Affine2::from(Similarity2 {
        rotation: Rotation2::from_angle(0.5),
        scaling: 0.5,
        offset: vec2(2.0, 1.0),
    });
    let c = Affine2::from(Similarity2 {
        rotation: Rotation2::from_angle(1.5),
        scaling: 1.0,
        offset: vec2(3.0, 3.0),
    });
    let x = vec2(5.0, 7.0);
    approx::assert_relative_eq!(a * (b * (c * x)), ((a * b) * c) * x, epsilon = 0.001);
    approx::assert_relative_eq!(a * (b * (c * x)), (a * (b * c)) * x, epsilon = 0.001);
}