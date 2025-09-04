use crate::angle::Angle;
use crate::scalar::Scalar;
use crate::vector::Vector;
use std::ffi::c_void;
use std::fmt::Display;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

impl<const L: usize, S: Scalar> Vector<L, S> {
    pub const ZERO: Self = Self([S::ZERO; L]);
    pub const ONE: Self = Self([S::ONE; L]);

    pub const fn from_array(inner: [S; L]) -> Self {
        Self(inner)
    }

    pub fn magnitude(&self) -> S {
        self.length().square_root2()
    }

    pub fn length(&self) -> S {
        self.0.into_iter().map(|s| s.squared()).sum::<S>()
    }

    pub fn dot_product(&self, other: &Self) -> S {
        (0..L)
            .into_iter()
            .map(|i| self.0[i] * other.0[i])
            .sum::<S>()
    }

    pub fn normalized(&self) -> Self {
        let mut out = self.clone();
        let len = out.magnitude();
        for i in 0..L {
            out[i] /= len;
        }
        out
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn angle_between(&self, other: &Self) -> Angle<S> {
        Angle::Radians(
            (self.dot_product(other) / (self.magnitude() * other.magnitude())).inverse_cosine(),
        )
    }

    pub fn as_ptr(&self) -> *const S {
        self.0.as_ptr()
    }

    pub fn as_c_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }

    pub fn up(self) -> Vector<{ L + 1 }, S> {
        let mut buff = [S::ZERO; L + 1];

        buff[..L].copy_from_slice(&self.0);

        Vector::from_array(buff)
    }

    pub fn down(self) -> Vector<{ L - 1 }, S> {
        let l = unsafe {
            let t = &self.0 as *const S as *const [S; L - 1];
            *t
        };
        Vector::from_array(l)
    }
}

impl<const L: usize, S: Scalar> Index<usize> for Vector<L, S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<const L: usize, S: Scalar> IndexMut<usize> for Vector<L, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const L: usize, S: Scalar> Display for Vector<L, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "<{}>",
                self.0
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        )
    }
}

impl<const L: usize, S: Scalar> Add<Self> for Vector<L, S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] += rhs[i];
        }
        out
    }
}

impl <const L: usize, S: Scalar> Add<S> for Vector<L, S> {
    type Output = Self;

    fn add(self, rhs: S) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] += rhs;
        }
        out
    }
}

/*impl<const L: usize, S: Scalar> Add<f32> for Vector<L, S> {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] += S::from_f32(rhs);
        }
        out
    }
}

impl<const L: usize, S: Scalar> Add<f64> for Vector<L, S> {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] += S::from_f64(rhs);
        }
        out
    }
}
*/


impl<const L: usize, S: Scalar> Sub<Self> for Vector<L, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] -= rhs[i];
        }
        out
    }
}

impl<const L: usize, S: Scalar> Sub<f32> for Vector<L, S> {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] -= S::from_f32(rhs);
        }
        out
    }
}

impl<const L: usize, S: Scalar> Sub<f64> for Vector<L, S> {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] -= S::from_f64(rhs);
        }
        out
    }
}

impl <const L: usize, S: Scalar> Mul<S> for Vector<L, S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] *= rhs;
        }
        out
    }
}

impl<const L: usize, S: Scalar> Mul<Self> for Vector<L, S> {
    type Output = S;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot_product(&rhs)
    }
}

impl<const L: usize, S: Scalar> Neg for Vector<L, S> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..L {
            out[i] = -self[i];
        }
        out
    }
}