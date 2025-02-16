//! This module defines helper functions that assume a particular convention for coordinate
//! systems and projection transforms. If this convention does not fit your needs, you can
//! simply ignore this module.
//!
//! The convention assumes that, for all objects:
//! * Positive X points to the right
//! * Positive Y points up
//! * Negative Z points forward
//!
//! This is the convention used by
//! [Godot](https://docs.godotengine.org/en/stable/tutorials/3d/introduction_to_3d.html#coordinate-system)
//! and [glTF](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#view-matrix) cameras (but
//! not objects, where positive Z points forward).
//!
//! Furthermore, "normalized device coordinates", which are the target of all projection transforms
//! in this module, are in the range [-1, 1] × [-1, 1] × [0, 1] with higher Z values being further
//! away from the camera. This is the convention used by D3D12 and Metal.
use crate::{vec3, vec4, Affine3, Matrix3, Matrix4, Motion3, Projective3, Rotation3, Scalar, Vector3};

/// A transformation which supports the [`LookTowards::look_towards`] method.
pub trait LookTowards {
    /// Constructs an object-to-world transform which rotates an object to face the given
    /// direction.
    fn look_towards(dir: Vector3) -> Self;
}

impl LookTowards for Matrix3 {
    fn look_towards(dir: Vector3) -> Self {
        let z = -dir.normalize();
        let x = vec3(0.0, 1.0, 0.0).cross(&z).normalize();
        let y = z.cross(&x);
        Self { x, y, z }
    }
}

impl LookTowards for Rotation3 {
    #[inline]
    fn look_towards(dir: Vector3) -> Self {
        Self::from_matrix(Matrix3::look_towards(dir))
    }
}

/// A transformation which supports the [`LookAt::look_at`] method.
pub trait LookAt {
    /// Constructs an object-to-world transform which positions the object at the given position
    /// and rotates it to face the given direction.
    ///
    /// Note that this needs to be inverted to get a world-to-view transform which is more
    /// useful for cameras.
    fn look_at(pos: Vector3, target: Vector3) -> Self;
}

impl LookAt for Motion3 {
    #[inline]
    fn look_at(pos: Vector3, target: Vector3) -> Self {
        Motion3 {
            rotation: Rotation3::look_towards(target - pos),
            offset: pos,
        }
    }
}

impl LookAt for Affine3 {
    #[inline]
    fn look_at(pos: Vector3, target: Vector3) -> Self {
        Affine3 {
            linear: Matrix3::look_towards(target - pos),
            offset: pos,
        }
    }
}

#[test]
fn test_look_at() {
    use std::f32::consts::SQRT_2;
    let trans = Affine3::look_at(vec3(1.0, 1.0, 1.0), vec3(1.0, 0.0, 0.0));
    approx::assert_relative_eq!(trans * vec3(1.0, 0.0, -SQRT_2), vec3(2.0, 0.0, 0.0));
}

/// A transformation which supports constructing perspective projection transforms.
pub trait Perspective {
    /// Constructs a perspective transform.
    fn perspective(aspect_ratio: Scalar, fov_y: Scalar, near_z: Scalar, far_z: Scalar) -> Self;
}

impl Perspective for Projective3 {
    fn perspective(aspect_ratio: Scalar, fov_y: Scalar, near_z: Scalar, far_z: Scalar) -> Self {
        let y_y = 1.0 / (fov_y / 2.0).tan();
        let x_x = y_y / aspect_ratio;
        let z_z;
        let w_z;
        if far_z == Scalar::INFINITY {
            z_z = -1.0;
            w_z = -near_z;
        } else {
            z_z = far_z / (near_z - far_z);
            w_z = near_z * far_z / (near_z - far_z);
        };
        Self::new(Matrix4 {
            x: vec4(x_x, 0.0, 0.0, 0.0),
            y: vec4(0.0, y_y, 0.0, 0.0),
            z: vec4(0.0, 0.0, z_z, -1.0),
            w: vec4(0.0, 0.0, w_z, 0.0),
        })
    }
}

#[test]
fn test_perspective() {
    let proj = Projective3::perspective(2.0, crate::PI / 2.0, 1.0, 5.0);
    approx::assert_relative_eq!(proj * vec3(-2.0, -1.0, -1.0), vec3(-1.0, -1.0, 0.0));
    approx::assert_relative_eq!(proj * vec3(2.0, -1.0, -1.0), vec3(1.0, -1.0, 0.0));
    approx::assert_relative_eq!(proj * vec3(10.0, 5.0, -5.0), vec3(1.0, 1.0, 1.0));
}