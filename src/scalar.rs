use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::interpolation::Interpolation;
use crate::percentage::Percentage;

crate::scalar!(f32 => "f");
crate::scalar!(f64 => "d");

pub trait Scalar:
    Copy
    + Clone
    + Debug
    + Default
    + Display
    + PartialEq
    + PartialOrd
    + Sum<Self>
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + std::ops::Rem<Output = Self>
    + Interpolation<Self>
{
    const NEG_ONE: Self;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const FOUR: Self;
    const FIVE: Self;

    const PI: Self;
    const NAME: &'static str;

    fn sine(self) -> Self;
    fn cosine(self) -> Self;
    fn tangent(self) -> Self;

    fn inverse_sine(self) -> Self;
    fn inverse_cosine(self) -> Self;

    fn power_i(self, i: i32) -> Self;
    fn power_f(self, f: Self) -> Self;
    fn squared(self) -> Self {
        self.power_f(Self::TWO)
    }
    fn square_root(self) -> Self {
        self.power_f(Self::ONE / Self::TWO)
    }
    fn square_root2(self) -> Self;

    fn from_f32(f: f32) -> Self;
    fn from_f64(f: f64) -> Self;

    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;

    fn rads(self) -> Self;
    fn degs(self) -> Self;

    fn s_clamp(self, min: Self, max: Self) -> Self;
    fn s_floor(self) -> Self;
    fn s_ceil(self) -> Self;
    fn s_abs(self) -> Self;

    fn from_u8(value: u8) -> Self;
    fn to_u8(self) -> u8;
    fn to_percentage(self) -> Percentage<Self> {
        Percentage::new(self)
    }

    fn map(a: Self, b: Self, v: Self, x: Self, y: Self) -> Self {
        Self::lerp(x, y, (v - a) / (b - a))
    }
}

#[macro_export]
macro_rules! scalar {
    ($t:ty => $name:literal) => {
        impl Scalar for $t {
            const NEG_ONE: Self = -1f64 as $t;
            const ZERO: Self = 0f64 as $t;
            const ONE: Self = 1f64 as $t;
            const TWO: Self = 2f64 as $t;
            const THREE: Self = 3f64 as $t;
            const FOUR: Self = 4f64 as $t;
            const FIVE: Self = 5f64 as $t;

            const PI: Self = std::f64::consts::PI as $t;
            const NAME: &'static str = $name;

            fn sine(self) -> Self {
                self.sin()
            }

            fn cosine(self) -> Self {
                self.cos()
            }

            fn tangent(self) -> Self {
                self.tan()
            }

            fn inverse_sine(self) -> Self {
                self.asin()
            }

            fn inverse_cosine(self) -> Self {
                self.acos()
            }

            fn power_i(self, i: i32) -> Self {
                self.powi(i)
            }

            fn power_f(self, f: Self) -> Self {
                self.powf(f)
            }

            fn square_root2(self) -> Self {
                self.sqrt()
            }

            fn from_f32(f: f32) -> Self {
                f as $t
            }

            fn from_f64(f: f64) -> Self {
                f as $t
            }

            fn to_f32(self) -> f32 {
                self as f32
            }

            fn to_f64(self) -> f64 {
                self as f64
            }

            fn rads(self) -> Self {
                self.to_radians()
            }

            fn degs(self) -> Self {
                self.to_degrees()
            }

            fn s_floor(self) -> Self {
                self.floor()
            }

            fn s_ceil(self) -> Self {
                self.ceil()
            }

            fn s_abs(self) -> Self {
                self.abs()
            }

            fn s_clamp(self, min: Self, max: Self) -> Self {
                self.max(min).min(max)
            }

            fn from_u8(v: u8) -> Self {
                v as Self
            }

            fn to_u8(self) -> u8 {
                self as u8
            }
        }
    };
}