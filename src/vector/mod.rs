use crate::scalar::Scalar;

pub mod vec2;
pub mod vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Vector<const L: usize, S: Scalar>([S; L]);

impl<const L: usize, S: Scalar> Vector<L, S> {
    pub fn from_array(inner: [S; L]) -> Self {
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
}

#[macro_export]
macro_rules! vector_property {
    ($this:ident($name:ident) -> $t:ty $path:block) => {
        pub fn $name(&$this) -> $t {
            $path
        }
    };
}
