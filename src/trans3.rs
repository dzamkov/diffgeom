use crate::{vec3, Matrix3, Rotation3, Scalar, Vector3};

/// A transform in three-dimensional space consisting of rotation and translation.
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Motion3 {
    /// The rotation component of this transform, applied before translation.
    pub rotation: Rotation3,

    /// The offset for the translation component of this transform, applied after rotation.
    pub offset: Vector3,
}

impl Motion3 {
    /// The identity motion.
    pub const fn identity() -> Self {
        Self {
            rotation: Rotation3::IDENTITY,
            offset: vec3(0.0, 0.0, 0.0),
        }
    }

    /// Constructs a motion which translates by the given offset.
    pub const fn translate(offset: Vector3) -> Self {
        Self {
            rotation: Rotation3::IDENTITY,
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
    pub const fn linear(&self) -> Rotation3 {
        self.rotation
    }
}

impl Default for Motion3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Rotation3> for Motion3 {
    fn from(rotation: Rotation3) -> Motion3 {
        Motion3 {
            rotation,
            offset: vec3(0.0, 0.0, 0.0),
        }
    }
}

impl core::ops::Mul<Rotation3> for Motion3 {
    type Output = Motion3;
    fn mul(self, rhs: Rotation3) -> Motion3 {
        Motion3 {
            rotation: self.rotation * rhs,
            offset: self.offset,
        }
    }
}

impl core::ops::Mul<Motion3> for Rotation3 {
    type Output = Motion3;
    fn mul(self, rhs: Motion3) -> Motion3 {
        Motion3 {
            rotation: self * rhs.rotation,
            offset: self * rhs.offset,
        }
    }
}

impl core::ops::Mul<Motion3> for Motion3 {
    type Output = Motion3;
    fn mul(self, rhs: Motion3) -> Motion3 {
        Motion3 {
            rotation: self.rotation * rhs.rotation,
            offset: self.rotation * rhs.offset + self.offset,
        }
    }
}

impl core::ops::Mul<Vector3> for Motion3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.rotation * rhs + self.offset
    }
}

/// A transform in three-dimensional space consisting of rotation, translation and uniform scaling.
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Similarity3 {
    /// The rotation component of this transform, applied before translation.
    pub rotation: Rotation3,

    /// The scaling component of this transform, applied before translation.
    pub scaling: Scalar,

    /// The offset for the translation component of this transform, applied after rotation and
    /// scaling.
    pub offset: Vector3,
}

impl Similarity3 {
    /// The identity similarity.
    pub const fn identity() -> Self {
        Self {
            rotation: Rotation3::IDENTITY,
            scaling: 1.0,
            offset: vec3(0.0, 0.0, 0.0),
        }
    }

    /// Constructs a similarity which translates by the given offset.
    pub const fn translate(offset: Vector3) -> Self {
        Self {
            rotation: Rotation3::IDENTITY,
            scaling: 1.0,
            offset,
        }
    }

    /// Constructs a similarity which scales by the given factor.
    pub const fn scale(scaling: Scalar) -> Self {
        Self {
            rotation: Rotation3::IDENTITY,
            scaling,
            offset: vec3(0.0, 0.0, 0.0),
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
    pub fn linear(&self) -> Matrix3 {
        Matrix3::from(self.rotation) * self.scaling
    }
}

impl Default for Similarity3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Rotation3> for Similarity3 {
    fn from(rotation: Rotation3) -> Similarity3 {
        Similarity3 {
            rotation,
            scaling: 1.0,
            offset: vec3(0.0, 0.0, 0.0),
        }
    }
}

impl From<Motion3> for Similarity3 {
    fn from(motion: Motion3) -> Similarity3 {
        Similarity3 {
            rotation: motion.rotation,
            scaling: 1.0,
            offset: motion.offset,
        }
    }
}

impl core::ops::Mul<Similarity3> for Similarity3 {
    type Output = Similarity3;
    fn mul(self, rhs: Similarity3) -> Similarity3 {
        Similarity3 {
            rotation: self.rotation * rhs.rotation,
            scaling: self.scaling * rhs.scaling,
            offset: self.rotation * (rhs.offset * self.scaling) + self.offset,
        }
    }
}

impl core::ops::Mul<Vector3> for Similarity3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.rotation * (rhs * self.scaling) + self.offset
    }
}

/// An affine transform in three-dimensional space.
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Affine3 {
    /// The linear component of this transform, applied before translation.
    pub linear: Matrix3,

    /// The offset for the translation component of this transform, applied after the linear
    /// component.
    pub offset: Vector3,
}

impl Affine3 {
    /// The identity transform.
    pub const fn identity() -> Self {
        Self {
            linear: Matrix3::identity(),
            offset: vec3(0.0, 0.0, 0.0),
        }
    }

    /// Constructs an affine transform which translates by the given offset.
    pub const fn translate(offset: Vector3) -> Self {
        Self {
            linear: Matrix3::identity(),
            offset,
        }
    }

    /// Constructs an affine transform which scales non-uniformly by the given factors.
    pub const fn scale(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self {
            linear: Matrix3 {
                x: vec3(x, 0.0, 0.0),
                y: vec3(0.0, y, 0.0),
                z: vec3(0.0, 0.0, z),
            },
            offset: vec3(0.0, 0.0, 0.0),
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

impl Default for Affine3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Rotation3> for Affine3 {
    fn from(rotation: Rotation3) -> Affine3 {
        Affine3 {
            linear: Matrix3::from(rotation),
            offset: vec3(0.0, 0.0, 0.0),
        }
    }
}

impl From<Motion3> for Affine3 {
    fn from(motion: Motion3) -> Affine3 {
        Affine3 {
            linear: Matrix3::from(motion.rotation),
            offset: motion.offset,
        }
    }
}

impl From<Similarity3> for Affine3 {
    fn from(similarity: Similarity3) -> Affine3 {
        Affine3 {
            linear: similarity.linear(),
            offset: similarity.offset,
        }
    }
}

impl core::ops::Mul<Affine3> for Affine3 {
    type Output = Affine3;
    fn mul(self, rhs: Affine3) -> Affine3 {
        Affine3 {
            linear: self.linear * rhs.linear,
            offset: self.linear * rhs.offset + self.offset,
        }
    }
}

impl core::ops::Mul<Similarity3> for Affine3 {
    type Output = Affine3;
    fn mul(self, rhs: Similarity3) -> Affine3 {
        self * Affine3::from(rhs)
    }
}

impl core::ops::Mul<Rotation3> for Affine3 {
    type Output = Affine3;
    fn mul(self, rhs: Rotation3) -> Affine3 {
        self * Affine3::from(rhs)
    }
}

impl core::ops::Mul<Vector3> for Affine3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.linear * rhs + self.offset
    }
}

#[test]
fn test_affine_compose() {
    let a = Affine3::from(Similarity3 {
        rotation: Rotation3::from_euler(vec3(1.0, 0.5, 1.0)),
        scaling: 2.0,
        offset: vec3(1.0, 2.0, 3.0),
    });
    let b = Affine3::from(Similarity3 {
        rotation: Rotation3::from_euler(vec3(1.0, 2.0, 3.0)),
        scaling: 0.5,
        offset: vec3(2.0, 1.0, 0.0),
    });
    let c = Affine3::from(Similarity3 {
        rotation: Rotation3::from_euler(vec3(3.0, 2.0, 1.0)),
        scaling: 1.0,
        offset: vec3(3.0, 3.0, 3.0),
    });
    let x = vec3(5.0, 7.0, 9.0);
    approx::assert_relative_eq!(a * (b * (c * x)), ((a * b) * c) * x, epsilon = 0.001);
    approx::assert_relative_eq!(a * (b * (c * x)), (a * (b * c)) * x, epsilon = 0.001);
}
