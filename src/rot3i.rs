use crate::{vec3, vec3i, Rotation3, Scalar, Vector3, Vector3i, Motion3};
use cantor::Finite;

/// A rotation in discrete (axis-aligned) three-dimensional space.
///
/// Each element is named after the result of applying the rotation to `(+X, +Y, +Z)`.
#[repr(u8)]
#[derive(Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Rotation3i {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "xpypzp"))]
    XpYpZp = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "ypxpzn"))]
    YpXpZn = 1,
    #[cfg_attr(feature = "serdere", serde(rename = "ynxpzp"))]
    YnXpZp = 2,
    #[cfg_attr(feature = "serdere", serde(rename = "xnypzn"))]
    XnYpZn = 3,
    #[cfg_attr(feature = "serdere", serde(rename = "xnynzp"))]
    XnYnZp = 4,
    #[cfg_attr(feature = "serdere", serde(rename = "ynxnzn"))]
    YnXnZn = 5,
    #[cfg_attr(feature = "serdere", serde(rename = "ypxnzp"))]
    YpXnZp = 6,
    #[cfg_attr(feature = "serdere", serde(rename = "xpynzn"))]
    XpYnZn = 7,
    #[cfg_attr(feature = "serdere", serde(rename = "zpxpyp"))]
    ZpXpYp = 8,
    #[cfg_attr(feature = "serdere", serde(rename = "xpznyp"))]
    XpZnYp = 9,
    #[cfg_attr(feature = "serdere", serde(rename = "xnzpyp"))]
    XnZpYp = 10,
    #[cfg_attr(feature = "serdere", serde(rename = "znxpyn"))]
    ZnXpYn = 11,
    #[cfg_attr(feature = "serdere", serde(rename = "znxnyp"))]
    ZnXnYp = 12,
    #[cfg_attr(feature = "serdere", serde(rename = "xnznyn"))]
    XnZnYn = 13,
    #[cfg_attr(feature = "serdere", serde(rename = "xpznyn"))]
    XpZpYn = 14,
    #[cfg_attr(feature = "serdere", serde(rename = "zpxnyn"))]
    ZpXnYn = 15,
    #[cfg_attr(feature = "serdere", serde(rename = "ypzpxp"))]
    YpZpXp = 16,
    #[cfg_attr(feature = "serdere", serde(rename = "zpypxn"))]
    ZpYpXn = 17,
    #[cfg_attr(feature = "serdere", serde(rename = "znypxp"))]
    ZnYpXp = 18,
    #[cfg_attr(feature = "serdere", serde(rename = "ynzpzn"))]
    YnZpXn = 19,
    #[cfg_attr(feature = "serdere", serde(rename = "ynznxp"))]
    YnZnXp = 20,
    #[cfg_attr(feature = "serdere", serde(rename = "znynxn"))]
    ZnYnXn = 21,
    #[cfg_attr(feature = "serdere", serde(rename = "zpynxp"))]
    ZpYnXp = 22,
    #[cfg_attr(feature = "serdere", serde(rename = "ypznxn"))]
    YpZnXn = 23,
}

impl Rotation3i {
    /// The identity rotation.
    pub const IDENTITY: Self = Self::XpYpZp;

    /// Gets the inverse of this rotation.
    pub const fn inverse(&self) -> Self {
        const TABLE: [Rotation3i; 24] = {
            let mut table = [Rotation3i::XpYpZp; 24];
            let mut i: u8 = 0;
            while i < 24 {
                let rot: Rotation3i = unsafe { std::mem::transmute(i) };
                let mut j: u8 = 0;
                while j < 24 {
                    let inv: Rotation3i = unsafe { std::mem::transmute(j) };
                    const TEST: Vector3i = vec3i(1, 2, 3);
                    if vec3i_eq(inv.apply_vec3i(rot.apply_vec3i(TEST)), TEST) {
                        table[i as usize] = inv;
                        break;
                    }
                    j += 1;
                }
                i += 1;
            }
            table
        };
        TABLE[*self as usize]
    }

    /// Determines the rotation `a * b`.
    const fn compose(a: Self, b: Self) -> Self {
        const TABLE: [[Rotation3i; 24]; 24] = {
            let mut table = [[Rotation3i::XpYpZp; 24]; 24];
            let mut i: u8 = 0;
            while i < 24 {
                let rot_a: Rotation3i = unsafe { std::mem::transmute(i) };
                let mut j: u8 = 0;
                while j < 24 {
                    let rot_b: Rotation3i = unsafe { std::mem::transmute(j) };
                    let mut k: u8 = 0;
                    while k < 24 {
                        let rot_c: Rotation3i = unsafe { std::mem::transmute(k) };
                        const TEST: Vector3i = vec3i(1, 2, 3);
                        if vec3i_eq(
                            rot_a.apply_vec3i(rot_b.apply_vec3i(TEST)),
                            rot_c.apply_vec3i(TEST),
                        ) {
                            table[i as usize][j as usize] = rot_c;
                            break;
                        }
                        k += 1;
                    }
                    j += 1;
                }
                i += 1;
            }
            table
        };
        TABLE[a as usize][b as usize]
    }

    /// Converts this rotation to a [`Rotation3`].
    const fn to_rot3(self) -> Rotation3 {
        const SQ: Scalar = std::f32::consts::SQRT_2 / 2.0;
        const TABLE: [Rotation3; 24] = [
            Rotation3::new_unchecked(1.0, 0.0, 0.0, 0.0),
            Rotation3::new_unchecked(0.0, SQ, SQ, 0.0),
            Rotation3::new_unchecked(SQ, 0.0, 0.0, SQ),
            Rotation3::new_unchecked(0.0, 0.0, 1.0, 0.0),
            Rotation3::new_unchecked(0.0, 0.0, 0.0, 1.0),
            Rotation3::new_unchecked(0.0, SQ, -SQ, 0.0),
            Rotation3::new_unchecked(SQ, 0.0, 0.0, -SQ),
            Rotation3::new_unchecked(0.0, 1.0, 0.0, 0.0),
            Rotation3::new_unchecked(0.5, 0.5, 0.5, 0.5),
            Rotation3::new_unchecked(SQ, SQ, 0.0, 0.0),
            Rotation3::new_unchecked(0.0, 0.0, SQ, SQ),
            Rotation3::new_unchecked(0.5, -0.5, -0.5, 0.5),
            Rotation3::new_unchecked(0.5, 0.5, -0.5, -0.5),
            Rotation3::new_unchecked(0.0, 0.0, -SQ, SQ),
            Rotation3::new_unchecked(SQ, -SQ, 0.0, 0.0),
            Rotation3::new_unchecked(0.5, -0.5, 0.5, -0.5),
            Rotation3::new_unchecked(0.5, -0.5, -0.5, -0.5),
            Rotation3::new_unchecked(SQ, 0.0, SQ, 0.0),
            Rotation3::new_unchecked(SQ, 0.0, -SQ, 0.0),
            Rotation3::new_unchecked(0.5, -0.5, 0.5, 0.5),
            Rotation3::new_unchecked(0.5, 0.5, -0.5, 0.5),
            Rotation3::new_unchecked(0.0, SQ, 0.0, -SQ),
            Rotation3::new_unchecked(0.0, SQ, 0.0, SQ),
            Rotation3::new_unchecked(0.5, 0.5, 0.5, -0.5),
        ];
        TABLE[self as usize]
    }

    /// Applies this rotation to a [`Vector3i`].
    const fn apply_vec3i(&self, source: Vector3i) -> Vector3i {
        match self {
            Rotation3i::XpYpZp => source,
            Rotation3i::YpXpZn => vec3i(source.y, source.x, -source.z),
            Rotation3i::YnXpZp => vec3i(-source.y, source.x, source.z),
            Rotation3i::XnYpZn => vec3i(-source.x, source.y, -source.z),
            Rotation3i::XnYnZp => vec3i(-source.x, -source.y, source.z),
            Rotation3i::YnXnZn => vec3i(-source.y, -source.x, -source.z),
            Rotation3i::YpXnZp => vec3i(source.y, -source.x, source.z),
            Rotation3i::XpYnZn => vec3i(source.x, -source.y, -source.z),
            Rotation3i::ZpXpYp => vec3i(source.z, source.x, source.y),
            Rotation3i::XpZnYp => vec3i(source.x, -source.z, source.y),
            Rotation3i::XnZpYp => vec3i(-source.x, source.z, source.y),
            Rotation3i::ZnXpYn => vec3i(-source.z, source.x, -source.y),
            Rotation3i::ZnXnYp => vec3i(-source.z, -source.x, source.y),
            Rotation3i::XnZnYn => vec3i(-source.x, -source.z, -source.y),
            Rotation3i::XpZpYn => vec3i(source.x, source.z, -source.y),
            Rotation3i::ZpXnYn => vec3i(source.z, -source.x, -source.y),
            Rotation3i::YpZpXp => vec3i(source.y, source.z, source.x),
            Rotation3i::ZpYpXn => vec3i(source.z, source.y, -source.x),
            Rotation3i::ZnYpXp => vec3i(-source.z, source.y, source.x),
            Rotation3i::YnZpXn => vec3i(-source.y, source.z, -source.x),
            Rotation3i::YnZnXp => vec3i(-source.y, -source.z, source.x),
            Rotation3i::ZnYnXn => vec3i(-source.z, -source.y, -source.x),
            Rotation3i::ZpYnXp => vec3i(source.z, -source.y, source.x),
            Rotation3i::YpZnXn => vec3i(source.y, -source.z, -source.x),
        }
    }

    /// Applies this rotation to a [`Vector3`].
    fn apply_vec3(&self, source: Vector3) -> Vector3 {
        match self {
            Rotation3i::XpYpZp => source,
            Rotation3i::YpXpZn => vec3(source.y, source.x, -source.z),
            Rotation3i::YnXpZp => vec3(-source.y, source.x, source.z),
            Rotation3i::XnYpZn => vec3(-source.x, source.y, -source.z),
            Rotation3i::XnYnZp => vec3(-source.x, -source.y, source.z),
            Rotation3i::YnXnZn => vec3(-source.y, -source.x, -source.z),
            Rotation3i::YpXnZp => vec3(source.y, -source.x, source.z),
            Rotation3i::XpYnZn => vec3(source.x, -source.y, -source.z),
            Rotation3i::ZpXpYp => vec3(source.z, source.x, source.y),
            Rotation3i::XpZnYp => vec3(source.x, -source.z, source.y),
            Rotation3i::XnZpYp => vec3(-source.x, source.z, source.y),
            Rotation3i::ZnXpYn => vec3(-source.z, source.x, -source.y),
            Rotation3i::ZnXnYp => vec3(-source.z, -source.x, source.y),
            Rotation3i::XnZnYn => vec3(-source.x, -source.z, -source.y),
            Rotation3i::XpZpYn => vec3(source.x, source.z, -source.y),
            Rotation3i::ZpXnYn => vec3(source.z, -source.x, -source.y),
            Rotation3i::YpZpXp => vec3(source.y, source.z, source.x),
            Rotation3i::ZpYpXn => vec3(source.z, source.y, -source.x),
            Rotation3i::ZnYpXp => vec3(-source.z, source.y, source.x),
            Rotation3i::YnZpXn => vec3(-source.y, source.z, -source.x),
            Rotation3i::YnZnXp => vec3(-source.y, -source.z, source.x),
            Rotation3i::ZnYnXn => vec3(-source.z, -source.y, -source.x),
            Rotation3i::ZpYnXp => vec3(source.z, -source.y, source.x),
            Rotation3i::YpZnXn => vec3(source.y, -source.z, -source.x),
        }
    }
}

/// Determines whether two [`Vector3i`]s are equal.
const fn vec3i_eq(a: Vector3i, b: Vector3i) -> bool {
    a.x == b.x && a.y == b.y && a.z == b.z
}

impl core::ops::Mul<Rotation3i> for Rotation3i {
    type Output = Rotation3i;
    fn mul(self, rhs: Rotation3i) -> Rotation3i {
        Rotation3i::compose(self, rhs)
    }
}

impl core::ops::Mul<Rotation3> for Rotation3i {
    type Output = Rotation3;
    fn mul(self, rhs: Rotation3) -> Rotation3 {
        self.to_rot3() * rhs
    }
}

impl core::ops::Mul<Motion3> for Rotation3i {
    type Output = Motion3;
    fn mul(self, rhs: Motion3) -> Motion3 {
        self.to_rot3() * rhs
    }
}

impl core::ops::Mul<Vector3i> for Rotation3i {
    type Output = Vector3i;
    fn mul(self, rhs: Vector3i) -> Vector3i {
        self.apply_vec3i(rhs)
    }
}

impl core::ops::Mul<Vector3> for Rotation3i {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.apply_vec3(rhs)
    }
}

impl From<Rotation3i> for Rotation3 {
    fn from(rotation: Rotation3i) -> Rotation3 {
        rotation.to_rot3()
    }
}

#[test]
fn test_compose_inverse() {
    for a in Rotation3i::iter() {
        assert_eq!(a.inverse() * a, Rotation3i::IDENTITY);
        assert_eq!(a * a.inverse(), Rotation3i::IDENTITY);
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_compose_associative() {
    for a in Rotation3i::iter() {
        for b in Rotation3i::iter() {
            for c in Rotation3i::iter() {
                assert_eq!((a * b) * c, a * (b * c));
            }
        }
    }
}

#[test]
fn test_to_rot3() {
    for a in Rotation3i::iter() {
        let b: Rotation3 = a.into();
        let test = vec3(1.0, 2.0, 3.0);
        approx::assert_relative_eq!(a * test, b * test, max_relative = 1.0e-6);
    }
}
