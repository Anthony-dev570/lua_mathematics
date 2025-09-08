use crate::angle::Angle;
use crate::matrix::square_matrix::SquareMatrix;
use crate::matrix::Matrix;
use crate::scalar::Scalar;
use crate::vector::vec3::{Vec3, Vec3F};
use crate::vector::vec4::{Vec4, Vec4F};
use crate::vector::Vector;
use mlua::{AnyUserData, MetaMethod};
use std::ops::Mul;

pub type Matrix4x4<S> = Matrix<4, 4, S>;
pub type Matrix4x4F = Matrix4x4<f32>;
pub type Matrix4x4D = Matrix4x4<f64>;

impl<S: Scalar> SquareMatrix<S> for Matrix4x4<S> {
    const IDENTITY: Self = Matrix::from_array([
        Vector::from_array([S::ONE, S::ZERO, S::ZERO, S::ZERO]),
        Vector::from_array([S::ZERO, S::ONE, S::ZERO, S::ZERO]),
        Vector::from_array([S::ZERO, S::ZERO, S::ONE, S::ZERO]),
        Vector::from_array([S::ZERO, S::ZERO, S::ZERO, S::ONE]),
    ]);

    fn identity(value: Option<S>) -> Self {
        match value {
            None => Self::IDENTITY,
            Some(v) => {
                let mut out = Self::ZERO;
                for i in 0..4 {
                    out[i][i] = v;
                }
                out
            }
        }
    }
}

impl<S: Scalar> Matrix4x4<S> {
    pub fn translation(v: Vec3<S>) -> Self {
        Self::from_array([
            Vector::from_array([S::ONE, S::ZERO, S::ZERO, v[0]]),
            Vector::from_array([S::ZERO, S::ONE, S::ZERO, v[1]]),
            Vector::from_array([S::ZERO, S::ZERO, S::ONE, v[2]]),
            Vector::from_array([S::ZERO, S::ZERO, S::ZERO, S::ONE]),
        ])
    }

    pub fn scale(v: Vec3<S>) -> Self {
        Self::from_array([
            Vector::from_array([v[0], S::ZERO, S::ZERO, S::ZERO]),
            Vector::from_array([S::ZERO, v[1], S::ZERO, S::ZERO]),
            Vector::from_array([S::ZERO, S::ZERO, v[2], S::ZERO]),
            Vector::from_array([S::ZERO, S::ZERO, S::ZERO, S::ONE]),
        ])
    }

    pub fn perspective(aspect_ratio: S, fov: Angle<S>, near: S, far: S) -> Self {
        let fov = (fov.to_radians().take() / S::TWO).tangent();
        Self::from_array([
            Vec4::from_array([S::ONE / (aspect_ratio * fov), S::ZERO, S::ZERO, S::ZERO]),
            Vec4::from_array([S::ZERO, S::ONE / fov, S::ZERO, S::ZERO]),
            Vec4::from_array([
                S::ZERO,
                S::ZERO,
                -(far + near) / (far - near),
                -(S::TWO * far * near) / (far - near),
            ]),
            Vec4::from_array([S::ZERO, S::ZERO, -S::ONE, S::ZERO]),
        ])
    }

    pub fn ortho(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Self {
        Self::from_array([
            Vec4::from_array([
                S::TWO / (right - left),
                S::ZERO,
                S::ZERO,
                -(right + left) / (right - left),
            ]),
            Vec4::from_array([
                S::ZERO,
                S::TWO / (top - bottom),
                S::ZERO,
                -(top + bottom) / (top - bottom),
            ]),
            Vec4::from_array([
                S::ZERO,
                S::ZERO,
                -S::TWO / (far - near),
                -(far + near) / (far - near),
            ]),
            Vec4::from_array([S::ZERO, S::ZERO, S::ZERO, S::ONE]),
        ])
    }

    pub fn look_at(eye: &Vec3<S>, center: &Vec3<S>, up: &Vec3<S>) -> Self {

        let zero = S::ZERO;
        let one = S::ONE;
        let f = (*center - *eye).normalized();
        let s = f.cross(up).normalized();
        let u = s.cross(&f);

        Matrix::from_array([
            Vector::from_array([s.x(), u.x(), -f.x(), zero]),
            Vector::from_array([s.y(), u.y(), -f.y(), zero]),
            Vector::from_array([s.z(), u.z(), -f.z(), zero]),
            Vector::from_array([
                -s.dot_product(&eye),
                -u.dot_product(&eye),
                f.dot_product(&eye),
                one,
            ]),
        ])
    }
}

impl<S: Scalar> Mul<Self> for Matrix4x4<S> {
    type Output = Self;

    fn mul(self, b: Self) -> Self::Output {
        let src1 = self.0;
        let src2 = b.0;

        let mut dest = Self::ZERO;

        dest[0][0] = src1[0][0] * src2[0][0]
            + src1[0][1] * src2[1][0]
            + src1[0][2] * src2[2][0]
            + src1[0][3] * src2[3][0];
        dest[0][1] = src1[0][0] * src2[0][1]
            + src1[0][1] * src2[1][1]
            + src1[0][2] * src2[2][1]
            + src1[0][3] * src2[3][1];
        dest[0][2] = src1[0][0] * src2[0][2]
            + src1[0][1] * src2[1][2]
            + src1[0][2] * src2[2][2]
            + src1[0][3] * src2[3][2];
        dest[0][3] = src1[0][0] * src2[0][3]
            + src1[0][1] * src2[1][3]
            + src1[0][2] * src2[2][3]
            + src1[0][3] * src2[3][3];
        dest[1][0] = src1[1][0] * src2[0][0]
            + src1[1][1] * src2[1][0]
            + src1[1][2] * src2[2][0]
            + src1[1][3] * src2[3][0];
        dest[1][1] = src1[1][0] * src2[0][1]
            + src1[1][1] * src2[1][1]
            + src1[1][2] * src2[2][1]
            + src1[1][3] * src2[3][1];
        dest[1][2] = src1[1][0] * src2[0][2]
            + src1[1][1] * src2[1][2]
            + src1[1][2] * src2[2][2]
            + src1[1][3] * src2[3][2];
        dest[1][3] = src1[1][0] * src2[0][3]
            + src1[1][1] * src2[1][3]
            + src1[1][2] * src2[2][3]
            + src1[1][3] * src2[3][3];
        dest[2][0] = src1[2][0] * src2[0][0]
            + src1[2][1] * src2[1][0]
            + src1[2][2] * src2[2][0]
            + src1[2][3] * src2[3][0];
        dest[2][1] = src1[2][0] * src2[0][1]
            + src1[2][1] * src2[1][1]
            + src1[2][2] * src2[2][1]
            + src1[2][3] * src2[3][1];
        dest[2][2] = src1[2][0] * src2[0][2]
            + src1[2][1] * src2[1][2]
            + src1[2][2] * src2[2][2]
            + src1[2][3] * src2[3][2];
        dest[2][3] = src1[2][0] * src2[0][3]
            + src1[2][1] * src2[1][3]
            + src1[2][2] * src2[2][3]
            + src1[2][3] * src2[3][3];
        dest[3][0] = src1[3][0] * src2[0][0]
            + src1[3][1] * src2[1][0]
            + src1[3][2] * src2[2][0]
            + src1[3][3] * src2[3][0];
        dest[3][1] = src1[3][0] * src2[0][1]
            + src1[3][1] * src2[1][1]
            + src1[3][2] * src2[2][1]
            + src1[3][3] * src2[3][1];
        dest[3][2] = src1[3][0] * src2[0][2]
            + src1[3][1] * src2[1][2]
            + src1[3][2] * src2[2][2]
            + src1[3][3] * src2[3][2];
        dest[3][3] = src1[3][0] * src2[0][3]
            + src1[3][1] * src2[1][3]
            + src1[3][2] * src2[2][3]
            + src1[3][3] * src2[3][3];

        dest
    }
}

crate::lua_matrix!(Matrix4x4F => f32 {
    Args = (Option<Vec4F>, Option<Vec4F>, Option<Vec4F>, Option<Vec4F>),
    CONSTRUCTOR_NAME = "mat4x4f",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args |
            Ok(Self::from_array([
                args.0.unwrap_or(Vec4F::ZERO),
                args.1.unwrap_or(Vec4F::ZERO),
                args.2.unwrap_or(Vec4F::ZERO),
                args.3.unwrap_or(Vec4F::ZERO)
            ]))
        )
    }
    associated_functions = (lua) [
        fn mat4x4f_identity(arg: Option<f32>) {
            Ok(Self::identity(arg))
        }

        fn mat4x4f_translation(arg: AnyUserData) {
            if let Ok(v3) = arg.borrow::<Vec3F>() {
                return Ok(Self::translation(*v3));
            }

            Ok(Self::IDENTITY)
        }

        fn mat4x4f_scale(arg: AnyUserData) {
            if let Ok(v3) = arg.borrow::<Vec3F>() {
                return Ok(Self::scale(*v3));
            }

            Ok(Self::IDENTITY)
        }

        fn mat4x4f_ortho(args: (Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>)) {
            let left = args.0.unwrap_or(0f32);
            let right = args.1.unwrap_or(1f32);
            let bottom = args.2.unwrap_or(0f32);
            let top = args.3.unwrap_or(1f32);
            let near = args.4.unwrap_or(0f32);
            let far = args.5.unwrap_or(1f32);

            Ok(Self::ortho(left, right, bottom, top, near, far))
        }

        fn mat4x4f_perspective(args: (Option<f32>, Option<Angle<f32>>, Option<f32>, Option<f32>)) {
            let aspect_ratio = args.0.unwrap_or(1f32);
            let angle = args.1.unwrap_or(Angle::Degrees(90f32));
            let near = args.2.unwrap_or(0.01f32);
            let far = args.3.unwrap_or(100f32);

            Ok(Self::perspective(aspect_ratio, angle, near, far))
        }

        fn mat4x4f_look_at(args: (Option<Vec3F>, Option<Vec3F>, Option<Vec3F>)) {
            let eye = args.0.unwrap_or(Vec3F::ZERO);
            let center = args.1.unwrap_or(Vec3F::ZERO);
            let up = args.2.unwrap_or(Vec3F::ZERO);

            Ok(Self::look_at(&eye, &center, &up))
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
