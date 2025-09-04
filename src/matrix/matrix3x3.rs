use mlua::{AnyUserData, FromLua};
use crate::angle::{Angle, AngleD, AngleF};
use crate::euler::{Euler, EulerF};
use crate::matrix::imp::MatrixInitializer;
use crate::matrix::square_matrix::SquareMatrix;
use crate::matrix::Matrix;
use crate::scalar::Scalar;
use crate::vector::vec3::{Vec3, Vec3F};
use crate::vector::Vector;

pub type Matrix3x3<S> = Matrix<3, 3, S>;
pub type Matrix3x3F = Matrix3x3<f32>;
pub type Matrix3x3Initializer<S> = MatrixInitializer<3, 3, S>;

#[derive(Debug, Clone, Copy, FromLua)]
pub enum RotationOrder {
    ZYX,
    XYZ
}

impl <S: Scalar> Matrix3x3<S> {
    pub fn x_rotation(theta: Angle<S>) -> Self {
        let one = S::ONE;

        let (sin, cos) = theta.to_radians().sin_cos();

        let mut out = Self::ZERO;

        out[0][0] = one;
        out[1][1] = cos;
        out[2][2] = cos;

        out[1][2] = -sin;
        out[2][1] = sin;

        out
    }

    pub fn y_rotation(theta: Angle<S>) -> Self {
        let one = S::ONE;

        let (sin, cos) = theta.to_radians().sin_cos();

        let mut out = Self::ZERO;

        out[0][0] = cos;
        out[0][2] = sin;
        out[1][1] = one;
        out[2][0] = -sin;
        out[2][2] = cos;

        out
    }
    pub fn z_rotation(theta: Angle<S>) -> Self {
        let one = S::ONE;
        let (sin, cos) = theta.to_radians().sin_cos();

        let mut out = Self::ZERO;

        out[0][0] = cos;
        out[0][1] = -sin;
        out[1][0] = sin;
        out[1][1] = cos;
        out[2][2] = one;

        out
    }

    pub fn rotation(euler: Euler<S>, rotation_order: RotationOrder) -> Self {
        let (yaw, pitch, roll) = euler.take_radians();

        let (yaw_sin, yaw_cos) = (yaw.sine(), yaw.cosine());
        let (pitch_sin, pitch_cos) = (pitch.sine(), pitch.cosine());
        let (roll_sin, roll_cos) = (roll.sine(), roll.cosine());

        match rotation_order {
            RotationOrder::ZYX => {
                eprintln!("Remember to test: [{}] {}:{}", file!(), line!(), column!());
                Self::from_array([
                    Vector::from_array([
                        yaw_cos * pitch_cos,
                        yaw_cos * pitch_sin * roll_sin - yaw_sin * roll_cos,
                        yaw_cos * pitch_sin + yaw_sin * roll_sin
                    ]),
                    Vector::from_array([
                        yaw_sin * pitch_cos,
                         yaw_sin * pitch_sin * roll_sin + yaw_cos * roll_cos,
                        yaw_sin * pitch_sin * roll_cos - yaw_cos * roll_sin
                    ]),
                    Vector::from_array([
                        -pitch_sin,
                        pitch_cos * roll_sin,
                        pitch_cos * roll_cos
                    ])
                ])
            }
            RotationOrder::XYZ => todo!()
        }
    }
}

impl<S: Scalar> SquareMatrix<S> for Matrix3x3<S> {
    const IDENTITY: Self = Self::from_array([
        Vec3::RIGHT,
        Vec3::UP,
        Vec3::FORWARD
    ]);

    fn identity(value: Option<S>) -> Self {
        let mut out = Self::IDENTITY;

        if let Some(v) = value {
            for i in 0..3 {
                out[i][i] = v;
            }
        }
        
        out
    }
}

crate::lua_matrix!(Matrix3x3F => f32 {
    Args = (Option<Vec3F>, Option<Vec3F>, Option<Vec3F>),
    CONSTRUCTOR_NAME = "mat3x3f",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args| Ok(Self::from_array([
            args.0.unwrap_or(Vec3F::ZERO),
            args.1.unwrap_or(Vec3F::ZERO),
            args.1.unwrap_or(Vec3F::ZERO)
        ])))
    }
    associated_functions = (lua) [
        fn mat3x3f_identity(arg: Option<f32>) {
            Ok(Self::identity(arg))
        }

        fn mat3x3f_x_rotation(angle: AnyUserData) {
            if let Ok(af) = angle.borrow::<AngleF>() {
                return Ok(Self::x_rotation(*af));
            }

            if let Ok(ad) = angle.borrow::<AngleD>() {
                return Ok(Self::x_rotation(ad.to_anglef()))
            }

            Ok(Self::ZERO)
        }

        fn mat3x3f_y_rotation(angle: AnyUserData) {
            if let Ok(af) = angle.borrow::<AngleF>() {
                return Ok(Self::y_rotation(*af));
            }

            if let Ok(ad) = angle.borrow::<AngleD>() {
                return Ok(Self::y_rotation(ad.to_anglef()))
            }

            Ok(Self::ZERO)
        }

        fn mat3x3f_z_rotation(angle: AnyUserData) {
            if let Ok(af) = angle.borrow::<AngleF>() {
                return Ok(Self::z_rotation(*af));
            }

            if let Ok(ad) = angle.borrow::<AngleD>() {
                return Ok(Self::z_rotation(ad.to_anglef()))
            }

            Ok(Self::ZERO)
        }

        fn mat3x3f_rotation(data: (Option<EulerF>, Option<RotationOrder>)) {
            let rotation = data.0.unwrap_or(EulerF {
                yaw: Angle::RAD_ZERO,
                pitch: Angle::RAD_ZERO,
                roll: Angle::RAD_ZERO,
                debug_flag: 0
            });

            let order = data.1.unwrap_or(RotationOrder::ZYX);
            Ok(Matrix::rotation(rotation, order))
        }
    ]
    methods = {}
    meta_method = {}
});