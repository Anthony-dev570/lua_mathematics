use crate::scalar::Scalar;
use crate::vector::vec2::{Vec2, Vec2D, Vec2F};
use crate::vector::vec3::{Vec3, Vec3D, Vec3F};
use crate::vector::vec4::{Vec4, Vec4D, Vec4F};

pub fn vec2<S: Scalar>(x: S, y: S) -> Vec2<S> {
    Vec2::from_array([x, y])
}

pub fn vec2f(x: f32, y: f32) -> Vec2F {
    Vec2F::from_array([x, y])
}

pub fn vec2d(x: f64, y: f64) -> Vec2D {
    Vec2D::from_array([x, y])
}

pub fn vec3<S: Scalar>(x: S, y: S, z: S) -> Vec3<S> {
    Vec3::from_array([x, y, z])
}

pub fn vec3f(x: f32, y: f32, z: f32) -> Vec3F {
    vec3(x, y, z)
}

pub fn vec3d(x: f64, y: f64, z: f64) -> Vec3D {
    vec3(x, y, z)
}

pub fn vec4<S: Scalar>(x: S, y: S, z: S, w: S) -> Vec4<S> {
    Vec4::from_array([x, y, z, w])
}

pub fn vec4f(x: f32, y: f32, z: f32, w: f32) -> Vec4F {
    Vec4F::from_array([x, y, z, w])
}

pub fn vec4d(x: f64, y: f64, z: f64, w: f64) -> Vec4D {
    Vec4D::from_array([x, y, z, w])
}