use crate::scalar::Scalar;

pub trait Interpolation<S: Scalar> {
    fn lerp(a: Self, b: Self, t: S) -> Self;
    fn inverse_lerp<F: Fn(&Self) -> S>(a: Self, b: Self, v: Self, f: F) -> S;
}

impl <S: Scalar> Interpolation<S> for f64 {
    fn lerp(a: Self, b: Self, t: S) -> Self {
        a + (b - a) * t.to_f64()
    }

    fn inverse_lerp<F: Fn(&Self) -> S>(a: Self, b: Self, v: Self, f: F) -> S {
        f(&((v - a) / (b - a)))
    }
}

impl <S: Scalar> Interpolation<S> for f32 {
    fn lerp(a: Self, b: Self, t: S) -> Self {
        a + (b - a) * t.to_f32()
    }

    fn inverse_lerp<F: Fn(&Self) -> S>(a: Self, b: Self, v: Self, f: F) -> S {
        f(&((v - a) / (b - a)))
    }
}