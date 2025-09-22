use crate::euler::Euler;
use crate::matrix::Matrix;
use crate::prelude::vec3;
use crate::scalar::Scalar;
use crate::vector::vec3::Vec3;
use crate::vector::Vector;
use rlua::{AnyUserData, Integer, MetaMethod, Number, UserData, UserDataMethods, Lua};
use mlua::FromLua;
use std::ops::{Add, Mul};

pub type QuatF = Quaternion<f32>;
pub type QuatD = Quaternion<f64>;

#[derive(Debug, Clone, Copy, PartialEq, FromLua)]
pub struct Quaternion<S: Scalar> {
    pub w: S,
    pub xyz: Vec3<S>,
}

impl<S: Scalar> Quaternion<S> {
    pub const IDENTITY: Self = Self {
        w: S::ONE,
        xyz: Vec3::ZERO,
    };

    pub fn from_euler(euler: Euler<S>) -> Self {
        let (mut roll, mut pitch, mut yaw) = euler.to_radians().take();

        roll /= S::TWO;
        pitch /= S::TWO;
        yaw /= S::TWO;

        let (cr, sr) = (roll.cosine(), roll.sine());
        let (cp, sp) = (pitch.cosine(), pitch.sine());
        let (cy, sy) = (yaw.cosine(), yaw.sine());

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        Quaternion { w, xyz: vec3(x, y, z) }
    }

    pub fn pure(xyz: Vec3<S>) -> Self {
        Self { w: S::ZERO, xyz }
    }

    pub fn conjugate(self) -> Self {
        Self {
            w: self.w,
            xyz: -self.xyz
        }
    }

    pub fn magnitude(self) -> S {
        (self.w.squared() + self.xyz[0].squared() + self.xyz[1].squared() + self.xyz[2].squared()).square_root2()
    }

    pub fn inverse(self) -> Self {
        self.conjugate() * (S::ONE / self.magnitude().squared())
    }

    pub fn to_rotation_3x3(self) -> Matrix<3, 3, S> {
        let (q0, q1, q2, q3) = (self.w, self.xyz[0], self.xyz[1], self.xyz[2]);

        let one = S::ONE;
        let two = S::TWO;

        let a = Vector::from_array([one - two * (q2.squared() + q3.squared()), two * (q1 * q2 - q0 * q3), two * (q1 * q3 + q0 * q2)]);
        let b = Vector::from_array([
            two * (q1 * q2 + q0 * q3),
            one - two * (q1.squared() + q3.squared()),
            two * (q2 * q3 - q0 * q1)
        ]);
        let c = Vector::from_array([
            two * q1 * q3 - q0 * q2,
            two * (q2 * q3 + q0 * q1),
            one - two * (q1.squared() + q2.squared())
        ]);

        Matrix::from_array([
            a, b, c
        ])
    }

    pub fn to_rotation_4x4(self) -> Matrix<4, 4, S> {
        let (q0, q1, q2, q3) = (self.w, self.xyz[0], self.xyz[1], self.xyz[2]);

        let one = S::ONE;
        let two = S::TWO;

        let a = Vector::from_array([one - two * (q2.squared() + q3.squared()), two * (q1 * q2 - q0 * q3), two * (q1 * q3 + q0 * q2), S::ZERO]);
        let b = Vector::from_array([
            two * (q1 * q2 + q0 * q3),
            one - two * (q1.squared() + q3.squared()),
            two * (q2 * q3 - q0 * q1),
            S::ZERO
        ]);
        let c = Vector::from_array([
            two * q1 * q3 - q0 * q2,
            two * (q2 * q3 + q0 * q1),
            one - two * (q1.squared() + q2.squared()),
            S::ZERO
        ]);

        Matrix::from_array([
            a, b, c, Vector::from_array([S::ZERO, S::ZERO, S::ZERO, S::ONE])
        ])
    }
}

impl<S: Scalar> Add<S> for Quaternion<S> {
    type Output = Self;

    fn add(self, rhs: S) -> Self::Output {
        Self {
            w: self.w + rhs,
            xyz: self.xyz + rhs,
        }
    }
}

impl<S: Scalar> Add<Self> for Quaternion<S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w + rhs.w,
            xyz: self.xyz + rhs.xyz,
        }
    }
}

impl<S: Scalar> Mul<Self> for Quaternion<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (aw, ax, ay, az) = (self.w, self.xyz[0], self.xyz[1], self.xyz[2]);
        let (bw, bx, by, bz) = (rhs.w, rhs.xyz[0], rhs.xyz[1], rhs.xyz[2]);

        let w = aw * bw - ax * bx - ay * by - az * bz;
        let x = aw * bx + ax * bw + ay * bz - az * by;
        let y = aw * by - ax * bz + ay * bw + az * bx;
        let z = aw * bz + ax * by - ay * bx + az * bw;

        Self {
            w,
            xyz: vec3(x, y, z),
        }
    }
}

impl<S: Scalar> Mul<Vec3<S>> for Quaternion<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: Vec3<S>) -> Self::Output {
        (self * Self::pure(rhs) * self.conjugate()).xyz
    }
}

impl<S: Scalar> Mul<S> for Quaternion<S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self {
            w: self.w * rhs,
            xyz: self.xyz * rhs
        }
    }
}

impl UserData for QuatF {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        crate::quaternion_methods!(methods => f32);
    }
}

pub mod quatf {
    use crate::euler::EulerF;
    use crate::quaternion::QuatF;
    use crate::vector::vec3::Vec3F;
    use crate::{LuaAssociatedFunction, LuaObject};
    use mlua::{Function, Lua};
    impl LuaObject for QuatF {
        type Args = (Option<f32>, Option<Vec3F>);
        const CONSTRUCTOR_NAME: &'static str = "quatf";

        fn create_constructor(lua: &Lua) -> mlua::Result<Function> {
            lua.create_function(|_lua, args: Self::Args| {
                Ok(Self {
                    w: args.0.unwrap_or(0f32),
                    xyz: args.1.unwrap_or(Vec3F::ZERO),
                })
            })
        }

        fn associated_functions(_lua: &Lua) -> mlua::Result<Vec<LuaAssociatedFunction>> {
            Ok(vec![
                LuaAssociatedFunction {
                    function: _lua.create_function(|_, _args: ()| Ok(Self::IDENTITY))?,
                    name: "quatf_identity"
                },
                LuaAssociatedFunction {
                    function: _lua.create_function(|_, args: Vec3F| Ok(Self::pure(args)))?,
                    name: "quatf_pure"
                },
                LuaAssociatedFunction {
                    function: _lua.create_function(|_, args: EulerF| Ok(args.to_quat()))?,
                    name: "quatf_from_euler"
                },
            ])
        }
    }
}

pub mod quatd {}

#[macro_export]
macro_rules! quaternion_methods {
    (
        $methods:ident => $f:ty
    ) => {
        $methods.add_method("magnitude", |_lua, this, ()| Ok(this.magnitude()));
        $methods.add_method("inverse", |_lua, this, ()| Ok(this.inverse()));
        $methods.add_method("conjugate", |_lua, this, ()| Ok(this.conjugate()));

        $methods.add_method(format!("to_mat3x3{}", <$f>::NAME), |_lua, this, ()| Ok(this.to_rotation_3x3()));
        $methods.add_method(format!("to_mat4x4{}", <$f>::NAME), |_lua, this, ()| Ok(this.to_rotation_4x4()));

        $methods.add_meta_method(MetaMethod::ToString, |_lua, this, ()| {
            Ok(format!("{this:?}"))
        });

        $methods.add_meta_method(MetaMethod::Add, |_lua, this, b: AnyUserData| {
            if let Ok(b) = b.borrow::<Self>() {
                return Ok(*this + *b);
            }

            if let Ok(b) = b.borrow::<Number>() {
                return Ok(*this + *b as $f);
            }
            if let Ok(b) = b.borrow::<Integer>() {
                return Ok(*this + *b as $f);
            }

            Ok(*this)
        });

        $methods.add_meta_method(MetaMethod::Mul, |_lua, this, b: AnyUserData| {
            if let Ok(b) = b.borrow::<Self>() {
                return Ok(*this * *b);
            }
            if let Ok(b) = b.borrow::<Vec3<$f>>() {
                return Ok(Quaternion::pure(*this * *b));
            }

            if let Ok(b) = b.borrow::<Number>() {
                return Ok(*this * *b as $f);
            }
            if let Ok(b) = b.borrow::<Integer>() {
                return Ok(*this * *b as $f);
            }

            Ok(*this)
        });
    };
}