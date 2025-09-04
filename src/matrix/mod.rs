use mlua::FromLua;
use crate::scalar::Scalar;
use crate::vector::Vector;

pub mod imp;
pub mod square_matrix;
pub mod matrix2x2;
pub mod matrix3x3;
pub mod matrix4x4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromLua)]
#[repr(C)]
pub struct Matrix<const R: usize, const C: usize, S: Scalar>([Vector<C, S>; R]);