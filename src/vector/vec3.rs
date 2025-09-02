use crate::scalar::Scalar;
use crate::vector::Vector;
use crate::vector_property;

pub type Vec3<S> = Vector<3, S>;
pub type Vec3F = Vec3<f32>;
pub type Vec3D = Vec3<f64>;

impl<S: Scalar> Vec3<S> {
    vector_property!(self(x) -> S {self.0[0]});
}
