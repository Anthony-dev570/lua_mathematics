use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

crate::scalar!(f32);
crate::scalar!(f64);

pub trait Scalar:
Copy + Clone + Debug + Default + Display +
PartialEq + PartialOrd
+ Sum<Self> + Neg<Output=Self>

+ Add<Output = Self>
+ Sub<Output = Self>
+ Mul<Output = Self>
+ Div<Output = Self>

+ AddAssign
+ SubAssign
+ MulAssign
+ DivAssign
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const PI: Self;

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
}

#[macro_export]
macro_rules! scalar {
    ($t:ty) => {
        impl Scalar for $t {
            const ZERO: Self = 0f64 as $t;
            const ONE: Self = 1f64 as $t;
            const TWO: Self = 2f64 as $t;
            const PI: Self = std::f64::consts::PI as $t;

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
        }
    };
}