use crate::{vec2, Scalar, Vector2};

/// An axis-aligned rectangle in two-dimensional space.
#[repr(C)]
#[derive(Default, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serdere", derive(serdere::Serialize, serdere::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
pub struct Box2 {
    min: Vector2,
    max: Vector2,
}

impl Box2 {
    /// A [`Box2`] that contains all points.
    pub const ALL: Box2 = Self {
        min: vec2(Scalar::NEG_INFINITY, Scalar::NEG_INFINITY),
        max: vec2(Scalar::INFINITY, Scalar::INFINITY),
    };

    /// Constructs a [`Box2`] which contains only the given point.
    #[inline]
    pub const fn only(point: Vector2) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    /// Constructs a [`Box2`] from its minimum and maximum coordinates.
    #[inline]
    pub const fn from_min_max(min: Vector2, max: Vector2) -> Self {
        Self { min, max }
    }

    /// The minimum coordinates of the box.
    #[inline]
    pub const fn min(&self) -> Vector2 {
        self.min
    }

    /// The maximum coordinates of the box.
    #[inline]
    pub const fn max(&self) -> Vector2 {
        self.max
    }

    /// Determines whether this box contains the given point.
    #[inline]
    pub const fn contains(&self, point: Vector2) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && point.x <= self.max.x
            && point.y <= self.max.y
    }

    /// Determines whether this box has any points in common with the given box.
    #[inline]
    pub const fn overlaps(&self, other: Box2) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && other.min.x <= self.max.x
            && other.min.y <= self.max.y
    }

    /// Gets the smallest box that contains both this box and the given box.
    #[inline]
    pub fn bound(&self, other: Box2) -> Box2 {
        Self {
            min: vec2(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: vec2(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }

    /// Gets the smallest box that contains all of the given boxes, or returns [`None`] if the
    /// iterator is empty.
    #[inline]
    pub fn bound_many(mut boxes: impl Iterator<Item = Box2>) -> Option<Box2> {
        let mut min = vec2(Scalar::INFINITY, Scalar::INFINITY);
        let mut max = vec2(Scalar::NEG_INFINITY, Scalar::NEG_INFINITY);
        let mut bx = boxes.next()?;
        loop {
            min.x = min.x.min(bx.min.x);
            min.y = min.y.min(bx.min.y);
            max.x = max.x.max(bx.max.x);
            max.y = max.y.max(bx.max.y);
            if let Some(next_bx) = boxes.next() {
                bx = next_bx;
            } else {
                return Some(Box2 { min, max });
            }
        }
    }
}
