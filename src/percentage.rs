use crate::scalar::Scalar;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::color::ColorComponent;

pub type PercentageF = Percentage<f32>;
pub type PercentageD = Percentage<f64>;

#[derive(Debug, Clone, Copy)]
pub struct Percentage<S: Scalar>(S);

impl<S: Scalar> Percentage<S> {
    pub fn new(s: S) -> Self {
        Percentage(s.s_clamp(S::ZERO, S::ONE))
    }

    pub fn from(a: S, b: S) -> Percentage<S> {
        Self::new(a / b)
    }

    pub fn from_100(u: u8) -> Self {
        Self::new(S::from_u8(u.min(100)) / S::from_u8(100))
    }

    pub fn take(self) -> S {
        self.0
    }

    pub fn to_color_component(self) -> ColorComponent<S> {
        ColorComponent::Percentage(self)
    }
}

impl<S: Scalar> Add<Self> for Percentage<S> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<S: Scalar> Sub<Self> for Percentage<S> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<S: Scalar> Mul<Self> for Percentage<S> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl<S: Scalar> Div<Self> for Percentage<S> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<S: Scalar> Add<S> for Percentage<S> {
    type Output = Self;
    fn add(self, rhs: S) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl<S: Scalar> Sub<S> for Percentage<S> {
    type Output = Self;
    fn sub(self, rhs: S) -> Self::Output {
        Self(self.0 - rhs)
    }
}
impl<S: Scalar> Mul<S> for Percentage<S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl<S: Scalar> Div<S> for Percentage<S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<S: Scalar> AddAssign<S> for Percentage<S> {
    fn add_assign(&mut self, rhs: S) {
        self.0 += rhs;
    }
}
impl<S: Scalar> SubAssign<S> for Percentage<S> {
    fn sub_assign(&mut self, rhs: S) {
        self.0 -= rhs;
    }
}
impl<S: Scalar> MulAssign<S> for Percentage<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.0 *= rhs;
    }
}
impl<S: Scalar> DivAssign<S> for Percentage<S> {
    fn div_assign(&mut self, rhs: S) {
        self.0 /= rhs;
    }
}

impl<S: Scalar> AddAssign<Self> for Percentage<S> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<S: Scalar> SubAssign<Self> for Percentage<S> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<S: Scalar> MulAssign<Self> for Percentage<S> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<S: Scalar> DivAssign<Self> for Percentage<S> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}
