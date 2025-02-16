use crate::{vec2, vec2i, Matrix2, Rotation2, Vector2, Vector2i};
use cantor::Finite;

/// A rotation in discrete (axis-aligned) two-dimensional space.
///
/// Each element is name is a shorthand description of the [`Matrix2`] it represents. The first
/// 2 letters correspond to the direction vector in the first column, the next 2 letters correspond
/// to the next column, and so on.
#[repr(u8)]
#[derive(Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Rotation2i {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "xpyp"))]
    XpYp = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "ypxn"))]
    YpXn = 1,
    #[cfg_attr(feature = "serdere", serde(rename = "xnyn"))]
    XnYn = 2,
    #[cfg_attr(feature = "serdere", serde(rename = "ynxp"))]
    YnXp = 3,
}

impl Rotation2i {
    /// The identity rotation.
    pub const IDENTITY: Self = Self::XpYp;

    /// A rotation which rotates counter-clockwise by 90 degrees.
    pub const CCW_90: Self = Self::YpXn;

    /// A rotation which rotates clockwise by 90 degrees.
    pub const CW_90: Self = Self::YnXp;

    /// A rotation which rotates by 180 degrees.
    pub const FLIP: Self = Self::XnYn;

    /// Gets the inverse of this rotation.
    pub fn inverse(&self) -> Self {
        unsafe { std::mem::transmute::<u8, Self>((4 - *self as u8) & 0b11) }
    }

    /// Converts this rotation to a [`Rotation2`].
    const fn to_rot2(self) -> Rotation2 {
        [
            Rotation2::IDENTITY,
            Rotation2::CCW_90,
            Rotation2::FLIP,
            Rotation2::CW_90,
        ][self as usize]
    }

    /// Applies this rotation to a [`Vector2i`].
    const fn apply_vec2i(&self, source: Vector2i) -> Vector2i {
        match self {
            Self::XpYp => vec2i(source.x, source.y),
            Self::YpXn => vec2i(-source.y, source.x),
            Self::XnYn => vec2i(-source.x, -source.y),
            Self::YnXp => vec2i(source.y, -source.x),
        }
    }

    /// Applies this rotation to a [`Vector2`].
    const fn apply_vec2(&self, source: Vector2) -> Vector2 {
        match self {
            Self::XpYp => vec2(source.x, source.y),
            Self::YpXn => vec2(-source.y, source.x),
            Self::XnYn => vec2(-source.x, -source.y),
            Self::YnXp => vec2(source.y, -source.x),
        }
    }
}

impl core::ops::Mul<Rotation2i> for Rotation2i {
    type Output = Rotation2i;
    fn mul(self, rhs: Rotation2i) -> Rotation2i {
        unsafe { std::mem::transmute::<u8, Self>((self as u8 + rhs as u8) & 0b11) }
    }
}

impl core::ops::Mul<Vector2i> for Rotation2i {
    type Output = Vector2i;
    fn mul(self, rhs: Vector2i) -> Vector2i {
        self.apply_vec2i(rhs)
    }
}

impl core::ops::Mul<Vector2> for Rotation2i {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.apply_vec2(rhs)
    }
}

impl From<Rotation2i> for Rotation2 {
    fn from(rotation: Rotation2i) -> Rotation2 {
        rotation.to_rot2()
    }
}

impl From<Rotation2i> for Matrix2 {
    fn from(rotation: Rotation2i) -> Matrix2 {
        rotation.to_rot2().into()
    }
}

#[test]
fn test_compose_inverse() {
    for a in Rotation2i::iter() {
        assert_eq!(a.inverse() * a, Rotation2i::IDENTITY);
        assert_eq!(a * a.inverse(), Rotation2i::IDENTITY);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_compose_associative() {
    for a in Rotation2i::iter() {
        for b in Rotation2i::iter() {
            for c in Rotation2i::iter() {
                assert_eq!((a * b) * c, a * (b * c));
            }
        }
    }
}

#[test]
fn test_to_rot2() {
    for a in Rotation2i::iter() {
        let b: Rotation2 = a.into();
        let test = vec2(1.0, 2.0);
        approx::assert_relative_eq!(a * test, b * test, max_relative = 1.0e-6);
    }
}
