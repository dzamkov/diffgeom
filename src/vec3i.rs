use crate::{vec3, Axis3, Dir1, Vector3};

/// A vector in discrete three-dimensional space.
#[repr(C)]
#[derive(
    Default,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Mul,
    derive_more::MulAssign,
    derive_more::Neg,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Div,
    derive_more::DivAssign,
)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3i {
    /// Constructs a vector from its components.
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Converts this discrete vector into a floating-point vector.
    #[inline(always)]
    pub fn into_float(self) -> Vector3 {
        vec3(self.x as f32, self.y as f32, self.z as f32)
    }
}

/// Shortcut for constructing a vector from its components.
#[inline(always)]
pub const fn vec3i(x: i32, y: i32, z: i32) -> Vector3i {
    Vector3i::new(x, y, z)
}

impl core::ops::Index<Axis3> for Vector3i {
    type Output = i32;
    #[inline]
    fn index(&self, axis: Axis3) -> &i32 {
        unsafe { std::mem::transmute::<&Vector3i, &[i32; 3]>(self).get_unchecked(axis.index()) }
    }
}

impl core::ops::IndexMut<Axis3> for Vector3i {
    #[inline]
    fn index_mut(&mut self, axis: Axis3) -> &mut i32 {
        unsafe {
            std::mem::transmute::<&mut Vector3i, &mut [i32; 3]>(self)
                .get_unchecked_mut(axis.index())
        }
    }
}

impl std::fmt::Debug for Vector3i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("vec3i")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

/// Identifies an orthogonal direction in three-dimensional space.
#[repr(u8)]
#[derive(cantor::Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Dir3i {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "xp"))]
    Xp = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "xn"))]
    Xn = 1,
    #[cfg_attr(feature = "serdere", serde(rename = "yp"))]
    Yp = 2,
    #[cfg_attr(feature = "serdere", serde(rename = "yn"))]
    Yn = 3,
    #[cfg_attr(feature = "serdere", serde(rename = "zp"))]
    Zp = 4,
    #[cfg_attr(feature = "serdere", serde(rename = "zn"))]
    Zn = 5,
}

impl Dir3i {
    /// Constructs a direction based on its axis and polarity along that axis.
    #[inline]
    pub fn new(axis: Axis3, polarity: Dir1) -> Self {
        unsafe { std::mem::transmute(((axis as u8) << 1) + polarity as u8) }
    }

    /// Gets the axis this direction is on.
    #[inline]
    pub fn axis(self) -> Axis3 {
        unsafe { std::mem::transmute((self as u8) >> 1) }
    }

    /// Gets the polairity of this direction along its axis.
    #[inline]
    pub fn polarity(self) -> Dir1 {
        unsafe { std::mem::transmute((self as u8) & 0b1) }
    }
}

impl From<Dir3i> for Vector3i {
    #[inline]
    fn from(dir: Dir3i) -> Self {
        [
            vec3i(1, 0, 0),
            vec3i(-1, 0, 0),
            vec3i(0, 1, 0),
            vec3i(0, -1, 0),
            vec3i(0, 0, 1),
            vec3i(0, 0, -1),
        ][dir as usize]
    }
}

impl From<Dir3i> for Vector3 {
    #[inline]
    fn from(dir: Dir3i) -> Self {
        Vector3i::from(dir).into_float()
    }
}
