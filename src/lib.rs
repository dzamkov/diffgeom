mod rot2;
mod rot3;
mod rot2i;
mod rot3i;
mod trans2;
mod trans3;
mod trans2i;
mod vec2i;
mod vec3i;

pub mod shape;
pub mod time;

pub use diffvec::{vec2, vec3, vec4, Matrix2, Matrix3, Matrix4, Scalar, Vector2, Vector3, Vector4};
pub use rot2::Rotation2;
pub use rot3::Rotation3;
pub use rot2i::Rotation2i;
pub use rot3i::Rotation3i;
pub use trans2::{Affine2, Motion2, Similarity2};
pub use trans3::{Affine3, Motion3, Similarity3};
pub use trans2i::{Motion2i, Ortho2i};
pub use vec2i::{vec2i, Dir2i, Vector2i};
pub use vec3i::{vec3i, Dir3i, Vector3i};

/// Identifies an axis in two-dimensional space.
#[repr(u8)]
#[derive(cantor::Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Axis2 {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "x"))]
    X = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "y"))]
    Y = 1,
}

impl Axis2 {
    /// Constructs an axis from its index, or returns [`None`] if the index is invalid.
    #[inline]
    pub fn new(index: usize) -> Option<Self> {
        if index < 2 {
            Some(Self::new_unchecked(index))
        } else {
            None
        }
    }

    /// Constructs an axis from its index, without checking that the index is valid.
    /// 
    /// # Safety
    /// The caller must ensure `index < 2`.
    #[inline(always)]
    pub fn new_unchecked(index: usize) -> Self {
        unsafe { std::mem::transmute::<u8, Axis2>(index as u8) }
    }

    /// Gets the index associated with this axis.
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }
}

/// Identifies an axis in three-dimensional space.
#[repr(u8)]
#[derive(cantor::Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Axis3 {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "x"))]
    X = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "y"))]
    Y = 1,
    #[cfg_attr(feature = "serdere", serde(rename = "z"))]
    Z = 2,
}

impl Axis3 {
    /// Constructs an axis from its index, or returns [`None`] if the index is invalid.
    #[inline]
    pub fn new(index: usize) -> Option<Self> {
        if index < 3 {
            Some(Self::new_unchecked(index))
        } else {
            None
        }
    }

    /// Constructs an axis from its index, without checking that the index is valid.
    /// 
    /// # Safety
    /// The caller must ensure `index < 3`.
    #[inline(always)]
    pub fn new_unchecked(index: usize) -> Self {
        unsafe { std::mem::transmute::<u8, Axis3>(index as u8) }
    }

    /// Gets the index associated with this axis.
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }
}

/// Identifies an axis in four-dimensional space.
#[repr(u8)]
#[derive(cantor::Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Axis4 {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "x"))]
    X = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "y"))]
    Y = 1,
    #[cfg_attr(feature = "serdere", serde(rename = "z"))]
    Z = 2,
    #[cfg_attr(feature = "serdere", serde(rename = "w"))]
    W = 3,
}

impl Axis4 {
    /// Constructs an axis from its index, or returns [`None`] if the index is invalid.
    #[inline]
    pub fn new(index: usize) -> Option<Self> {
        if index < 4 {
            Some(Self::new_unchecked(index))
        } else {
            None
        }
    }

    /// Constructs an axis from its index, without checking that the index is valid.
    /// 
    /// # Safety
    /// The caller must ensure `index < 4`.
    #[inline(always)]
    pub fn new_unchecked(index: usize) -> Self {
        unsafe { std::mem::transmute::<u8, Axis4>(index as u8) }
    }

    /// Gets the index associated with this axis.
    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }
}

impl core::ops::Index<Axis2> for Vector2 {
    type Output = Scalar;
    #[inline]
    fn index(&self, axis: Axis2) -> &Scalar {
        unsafe { std::mem::transmute::<&Vector2, &[Scalar; 2]>(self).get_unchecked(axis.index()) }
    }
}

impl core::ops::IndexMut<Axis2> for Vector2 {
    #[inline]
    fn index_mut(&mut self, axis: Axis2) -> &mut Scalar {
        unsafe {
            std::mem::transmute::<&mut Vector2, &mut [Scalar; 2]>(self)
                .get_unchecked_mut(axis.index())
        }
    }
}

impl core::ops::Index<Axis3> for Vector3 {
    type Output = Scalar;
    #[inline]
    fn index(&self, axis: Axis3) -> &Scalar {
        unsafe { std::mem::transmute::<&Vector3, &[Scalar; 3]>(self).get_unchecked(axis.index()) }
    }
}

impl core::ops::IndexMut<Axis3> for Vector3 {
    #[inline]
    fn index_mut(&mut self, axis: Axis3) -> &mut Scalar {
        unsafe {
            std::mem::transmute::<&mut Vector3, &mut [Scalar; 3]>(self)
                .get_unchecked_mut(axis.index())
        }
    }
}

impl core::ops::Index<Axis4> for Vector4 {
    type Output = Scalar;
    #[inline]
    fn index(&self, axis: Axis4) -> &Scalar {
        unsafe { std::mem::transmute::<&Vector4, &[Scalar; 4]>(self).get_unchecked(axis.index()) }
    }
}

impl core::ops::IndexMut<Axis4> for Vector4 {
    #[inline]
    fn index_mut(&mut self, axis: Axis4) -> &mut Scalar {
        unsafe {
            std::mem::transmute::<&mut Vector4, &mut [Scalar; 4]>(self)
                .get_unchecked_mut(axis.index())
        }
    }
}

#[test]
fn test_vector_index() {
    let mut a = vec2(1.0, 2.0);
    a[Axis2::X] += 3.0;
    a[Axis2::Y] *= a[Axis2::X];
    assert_eq!(a, vec2(4.0, 8.0));
    let mut b = vec3(1.0, 2.0, 3.0);
    b[Axis3::X] += 3.0;
    b[Axis3::Y] += b[Axis3::X];
    b[Axis3::Z] += b[Axis3::Y];
    assert_eq!(b, vec3(4.0, 6.0, 9.0));
    let mut c = vec4(1.0, 2.0, 3.0, 4.0);
    c[Axis4::X] += 5.0;
    c[Axis4::Y] += c[Axis4::W];
    c[Axis4::Z] += c[Axis4::Y];
    assert_eq!(c, vec4(6.0, 6.0, 9.0, 4.0));
}

/// Identifies a direction in one-dimensional space.
#[repr(u8)]
#[derive(cantor::Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Dir1 {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "p"))]
    P = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "n"))]
    N = 1,
}
