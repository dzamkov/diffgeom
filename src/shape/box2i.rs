use crate::{vec2i, Vector2i};
use std::num::NonZeroU32;

/// An axis-aligned rectangle in discrete two-dimensional space.
///
/// Boxes must always have a positive size and contain at least one point.
#[repr(C)]
#[derive(Default, PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Box2i {
    /// The inclusive minimum coordinates of the box.
    min: Vector2i,

    /// The inclusive maximum coordinates of the box.
    max: Vector2i,
}

impl Box2i {
    /// A [`Box2i`]` that contains all points.
    pub const ALL: Box2i = Self {
        min: vec2i(i32::MIN, i32::MIN),
        max: vec2i(i32::MAX, i32::MAX),
    };

    /// Constructs a [`Box2i`] which contains only the given point.
    #[inline]
    pub fn only(point: Vector2i) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    /// Constructs a [`Box2i`] from its minimum and maximum coordinates.
    ///
    /// This is also the smallest box that contains the two points.
    #[inline]
    pub const fn from_min_max(min: Vector2i, max: Vector2i) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        Self { min, max }
    }

    /// Constructs a [`Box2i`] from its minimum coordinates and size.
    #[inline]
    pub fn from_min_size(min: Vector2i, size: Size2i) -> Self {
        Self {
            min,
            max: Vector2i::new(
                min.x.saturating_add_unsigned(size.x_minus_1),
                min.y.saturating_add_unsigned(size.y_minus_1),
            ),
        }
    }

    /// The inclusive minimum coordinates of the box.
    #[inline]
    pub fn min(&self) -> Vector2i {
        self.min
    }

    /// The inclusive maximum coordinates of the box.
    #[inline]
    pub fn max(&self) -> Vector2i {
        self.max
    }

    /// The size of the box.
    #[inline]
    pub fn size(&self) -> Size2i {
        Size2i {
            x_minus_1: (self.max.x as u32) - (self.min.x as u32),
            y_minus_1: (self.max.y as u32) - (self.min.y as u32),
        }
    }

    /// Determines whether this box contains the given point.
    #[inline]
    pub fn contains(&self, point: Vector2i) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && point.x <= self.max.x
            && point.y <= self.max.y
    }

    /// Determines whether this box has any points in common with the given box.
    #[inline]
    pub fn overlaps(&self, other: Box2i) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && other.min.x <= self.max.x
            && other.min.y <= self.max.y
    }
}

/// Describes the size of a [`Box2i`]. Each component must be positive.
#[repr(C)]
#[derive(Default, PartialEq, Eq, Copy, Clone, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Size2i {
    x_minus_1: u32,
    y_minus_1: u32,
}

impl Size2i {
    /// Constructs a [`Size2i`] from its components.
    #[inline]
    pub const fn new(x: NonZeroU32, y: NonZeroU32) -> Self {
        Self {
            x_minus_1: x.get() - 1,
            y_minus_1: y.get() - 1,
        }
    }

    /// One less than the size in the x direction.
    #[inline]
    pub fn x_minus_1(&self) -> u32 {
        self.x_minus_1
    }

    /// One less than the size in the y direction.
    #[inline]
    pub fn y_minus_1(&self) -> u32 {
        self.y_minus_1
    }
}

/// The error message given when there is an attempt to construct a [`Size2i`] with a zero
/// component.
const SIZE_COMPONENT_ZERO_ERROR: &str = "size component must not be zero";

/// Shortcut for constructing a [`Size2i`] from its components. Panics if any component is zero.
#[inline(always)]
pub const fn size2i(x: u32, y: u32) -> Size2i {
    Size2i::new(
        NonZeroU32::new(x).expect(SIZE_COMPONENT_ZERO_ERROR),
        NonZeroU32::new(y).expect(SIZE_COMPONENT_ZERO_ERROR),
    )
}

impl std::fmt::Debug for Size2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("size2i")
            .field(&(self.x_minus_1 as usize + 1))
            .field(&(self.y_minus_1 as usize + 1))
            .finish()
    }
}

impl core::ops::Add<Size2i> for Size2i {
    type Output = Size2i;
    fn add(self, rhs: Size2i) -> Size2i {
        Size2i {
            x_minus_1: self.x_minus_1 + rhs.x_minus_1 + 1,
            y_minus_1: self.y_minus_1 + rhs.y_minus_1 + 1,
        }
    }
}

impl core::ops::AddAssign<Size2i> for Size2i {
    fn add_assign(&mut self, rhs: Size2i) {
        self.x_minus_1 += rhs.x_minus_1 + 1;
        self.y_minus_1 += rhs.y_minus_1 + 1;
    }
}
