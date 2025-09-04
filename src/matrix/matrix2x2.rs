use std::ops::Mul;
use mlua::MetaMethod;
use crate::matrix::imp::MatrixInitializer;
use crate::matrix::square_matrix::SquareMatrix;
use crate::matrix::Matrix;
use crate::scalar::Scalar;
use crate::vector::vec2::Vec2F;
use crate::vector::Vector;

pub type Matrix2x2Initializer<S> = MatrixInitializer<2, 2, S>;

pub type Matrix2x2<S> = Matrix<2, 2, S>;
pub type Matrix2x2F = Matrix2x2<f32>;
pub type Matrix2x2D = Matrix2x2<f64>;

impl<S: Scalar> Mul<Self> for Matrix2x2<S> {
    type Output = Matrix2x2<S>;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.0;
        let b = rhs.0;

        let mut c = [Vector::ZERO; 2];
        c[0][0] = a[0][0] * b[0][0] + a[0][1] * b[1][0];
        c[0][1] = a[0][0] * b[0][1] + a[0][1] * b[1][1];
        c[1][0] = a[1][0] * b[0][0] + a[1][1] * b[1][0];
        c[1][1] = a[1][0] * b[0][1] + a[1][1] * b[1][1];

        Self::from_array(c)
    }
}

impl <S: Scalar> SquareMatrix<S> for Matrix2x2<S> {
    const IDENTITY: Self = Matrix::from_array([
        Vector::from_array([S::ONE, S::ZERO]),
        Vector::from_array([S::ZERO, S::ONE])
    ]);

    fn identity(value: Option<S>) -> Self {
        let mut out = Self::IDENTITY;

        if let Some(value) = value {
            for i in 0..2 {
                out[i][i] = value;
            }
        }

        out
    }
}

crate::lua_matrix!(Matrix2x2F => f32 {
    Args = (Option<Vec2F>, Option<Vec2F>),
    CONSTRUCTOR_NAME = "mat2x2f",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args |
            Ok(Self::from_array([
                args.0.unwrap_or(Vec2F::ZERO),
                args.1.unwrap_or(Vec2F::ZERO)
            ]))
        )
    }
    associated_functions = (lua) [
        fn mat2x2f_identity(arg: Option<f32>) {
            Ok(Self::identity(arg))
        }
    ]
    methods = {

    }
    meta_method = {
        MetaMethod::Mul[this] => b: Self {
            Ok(*this * b)
        }
    }
});