use crate::{vec2i, Motion2, Rotation2i, Vector2i};
use diffvec::{vec2, Vector2};
use std::num::{NonZeroI32, NonZeroI8};

/// A transform in discrete two-dimensional space consisting of rotation and translation.
///
/// This is the most general type of transform in discrete two-dimensional space that is
/// invertible.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub struct Motion2i {
    /// The rotation component of this transform, applied before translation.
    pub rotation: Rotation2i,

    /// The offset for the translation component of this transform, applied after rotation.
    pub offset: Vector2i,
}

impl Motion2i {
    /// The identity motion.
    pub const fn identity() -> Self {
        Self {
            rotation: Rotation2i::IDENTITY,
            offset: Vector2i::new(0, 0),
        }
    }

    /// Constructs a motion which translates by the given offset.
    pub const fn translate(offset: Vector2i) -> Self {
        Self {
            rotation: Rotation2i::IDENTITY,
            offset,
        }
    }

    /// Gets the linear component of this motion.
    pub const fn linear(&self) -> Rotation2i {
        self.rotation
    }

    /// Gets the inverse of this motion.
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.inverse();
        Self {
            rotation,
            offset: rotation * -self.offset,
        }
    }
}

impl From<Rotation2i> for Motion2i {
    fn from(value: Rotation2i) -> Self {
        Motion2i {
            rotation: value,
            offset: vec2i(0, 0),
        }
    }
}

impl_trans_mul!(Rotation2i, Motion2i);

impl core::ops::Mul<Motion2i> for Motion2i {
    type Output = Motion2i;
    fn mul(self, rhs: Motion2i) -> Motion2i {
        Motion2i {
            rotation: self.rotation * rhs.rotation,
            offset: self.rotation * rhs.offset + self.offset,
        }
    }
}

impl core::ops::Mul<Vector2i> for Motion2i {
    type Output = Vector2i;
    fn mul(self, rhs: Vector2i) -> Vector2i {
        self.rotation * rhs + self.offset
    }
}

impl core::ops::Mul<Vector2> for Motion2i {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.rotation * rhs + self.offset.to_float()
    }
}

impl From<Motion2i> for Motion2 {
    fn from(value: Motion2i) -> Self {
        Motion2 {
            rotation: value.rotation.into(),
            offset: value.offset.to_float(),
        }
    }
}

/// A transform in discrete two-dimensional space consisting of rotation, non-uniform scaling,
/// reflection and translation, i.e. a transform that preserves orthogonality of the axes.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Ortho2i {
    /// The scaling applied to the X axis. Can be negative to reflect about Y axis.
    scaling_x: NonZeroI8,

    /// The scaling applied to the Y axis. Can be negative to reflect about X axis.
    scaling_y: NonZeroI8,

    /// If true, the X and Y axes are swapped after scaling.
    swap_axes: bool,

    /// The offset for the translation component of this transform, applied after rotation and
    /// scaling.
    pub offset: Vector2i,
}

impl Ortho2i {
    /// The identity transform.
    pub const fn identity() -> Self {
        Self {
            scaling_x: NonZeroI8::new(1).unwrap(),
            scaling_y: NonZeroI8::new(1).unwrap(),
            swap_axes: false,
            offset: vec2i(0, 0),
        }
    }

    /// Constructs an orthogonal transform which translates by the given offset.
    pub const fn translate(offset: Vector2i) -> Self {
        Self {
            scaling_x: NonZeroI8::new(1).unwrap(),
            scaling_y: NonZeroI8::new(1).unwrap(),
            swap_axes: false,
            offset,
        }
    }

    /// Constructs an orthogonal transform which scales by the given factors.
    pub fn scale(x: i32, y: i32) -> Self {
        let scaling_x: NonZeroI32 = x.try_into().unwrap();
        let scaling_y: NonZeroI32 = y.try_into().unwrap();
        Self {
            scaling_x: scaling_x.try_into().unwrap(),
            scaling_y: scaling_y.try_into().unwrap(),
            swap_axes: false,
            offset: vec2i(0, 0),
        }
    }
}

impl From<Rotation2i> for Ortho2i {
    fn from(value: Rotation2i) -> Self {
        Motion2i::from(value).into()
    }
}

impl From<Motion2i> for Ortho2i {
    fn from(value: Motion2i) -> Self {
        let (scaling_x, scaling_y, swap_axes) = [
            (
                NonZeroI8::new(1).unwrap(),
                NonZeroI8::new(1).unwrap(),
                false,
            ),
            (
                NonZeroI8::new(1).unwrap(),
                NonZeroI8::new(-1).unwrap(),
                true,
            ),
            (
                NonZeroI8::new(-1).unwrap(),
                NonZeroI8::new(-1).unwrap(),
                false,
            ),
            (
                NonZeroI8::new(-1).unwrap(),
                NonZeroI8::new(1).unwrap(),
                true,
            ),
        ][value.rotation as usize];
        Self {
            scaling_x,
            scaling_y,
            swap_axes,
            offset: value.offset,
        }
    }
}

impl_trans_mul!(Rotation2i, Ortho2i);
impl_trans_mul!(Motion2i, Ortho2i);

impl core::ops::Mul<Ortho2i> for Ortho2i {
    type Output = Ortho2i;
    fn mul(self, rhs: Ortho2i) -> Ortho2i {
        let mut scaling_x = self.scaling_x;
        let mut scaling_y = self.scaling_y;
        if rhs.swap_axes {
            std::mem::swap(&mut scaling_x, &mut scaling_y);
        }
        let scaling_x = NonZeroI8::try_from(scaling_x.get() * rhs.scaling_x.get()).unwrap();
        let scaling_y = NonZeroI8::try_from(scaling_y.get() * rhs.scaling_y.get()).unwrap();
        let swap_axes = self.swap_axes ^ rhs.swap_axes;
        Ortho2i {
            scaling_x,
            scaling_y,
            swap_axes,
            offset: self * rhs.offset,
        }
    }
}

impl core::ops::Mul<Vector2i> for Ortho2i {
    type Output = Vector2i;
    fn mul(self, rhs: Vector2i) -> Vector2i {
        let mut x = rhs.x * i32::from(self.scaling_x.get());
        let mut y = rhs.y * i32::from(self.scaling_y.get());
        if self.swap_axes {
            std::mem::swap(&mut x, &mut y);
        }
        vec2i(x, y) + self.offset
    }
}

impl core::ops::Mul<Vector2> for Ortho2i {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        let mut x = rhs.x * f32::from(self.scaling_x.get());
        let mut y = rhs.y * f32::from(self.scaling_y.get());
        if self.swap_axes {
            std::mem::swap(&mut x, &mut y);
        }
        vec2(x, y) + self.offset.to_float()
    }
}

#[test]
fn test_compose_ortho() {
    let a = Rotation2i::CW_90 * Motion2i::translate(vec2i(1, 2));
    let b = Ortho2i::scale(-5, 7);
    let c = Rotation2i::CCW_90 * Motion2i::translate(vec2i(3, 4));
    let x = vec2i(-4, 9);
    assert_eq!(Ortho2i::from(a) * b * Ortho2i::from(c), Ortho2i::from(a) * (b * Ortho2i::from(c)));
    assert_eq!(Ortho2i::from(a) * b * Ortho2i::from(c) * x, a * (b * (c * x)));
}