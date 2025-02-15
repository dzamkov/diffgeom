use crate::{vec2, Axis2, Dir1, Vector2};

/// A vector in discrete two-dimensional space.
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
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    /// Constructs a vector from its components.
    #[inline(always)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Converts this discrete vector into a floating-point vector.
    #[inline(always)]
    pub const fn to_float(&self) -> Vector2 {
        vec2(self.x as f32, self.y as f32)
    }
}

/// Shortcut for constructing a vector from its components.
#[inline(always)]
pub const fn vec2i(x: i32, y: i32) -> Vector2i {
    Vector2i::new(x, y)
}

impl core::ops::Index<Axis2> for Vector2i {
    type Output = i32;
    #[inline]
    fn index(&self, axis: Axis2) -> &i32 {
        unsafe { std::mem::transmute::<&Vector2i, &[i32; 2]>(self).get_unchecked(axis.index()) }
    }
}

impl core::ops::IndexMut<Axis2> for Vector2i {
    #[inline]
    fn index_mut(&mut self, axis: Axis2) -> &mut i32 {
        unsafe {
            std::mem::transmute::<&mut Vector2i, &mut [i32; 2]>(self)
                .get_unchecked_mut(axis.index())
        }
    }
}

impl std::fmt::Debug for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("vec2i")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

/// Identifies an orthogonal direction in two-dimensional space.
#[repr(u8)]
#[derive(cantor::Finite, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Deserialize, serdere::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
pub enum Dir2i {
    #[default]
    #[cfg_attr(feature = "serdere", serde(rename = "xp"))]
    Xp = 0,
    #[cfg_attr(feature = "serdere", serde(rename = "xn"))]
    Xn = 1,
    #[cfg_attr(feature = "serdere", serde(rename = "yp"))]
    Yp = 2,
    #[cfg_attr(feature = "serdere", serde(rename = "yn"))]
    Yn = 3,
}

impl Dir2i {
    /// Constructs a direction based on its axis and polarity along that axis.
    #[inline]
    pub fn new(axis: Axis2, polarity: Dir1) -> Self {
        unsafe { std::mem::transmute(((axis as u8) << 1) + polarity as u8) }
    }

    /// Gets the axis this direction is on.
    #[inline]
    pub fn axis(self) -> Axis2 {
        unsafe { std::mem::transmute((self as u8) >> 1) }
    }

    /// Gets the polairity of this direction along its axis.
    #[inline]
    pub fn polarity(self) -> Dir1 {
        unsafe { std::mem::transmute((self as u8) & 0b1) }
    }
}

impl From<Dir2i> for Vector2i {
    #[inline]
    fn from(dir: Dir2i) -> Self {
        [vec2i(1, 0), vec2i(-1, 0), vec2i(0, 1), vec2i(0, -1)][dir as usize]
    }
}

impl From<Dir2i> for Vector2 {
    #[inline]
    fn from(dir: Dir2i) -> Self {
        Vector2i::from(dir).to_float()
    }
}
