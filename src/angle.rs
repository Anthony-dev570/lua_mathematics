use crate::scalar::Scalar;
use rlua::{AnyUserData, Lua, MetaMethod, Number, UserData, UserDataMethods};
use mlua::FromLua;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::interpolation::Interpolation;

pub type AngleF = Angle<f32>;
pub type AngleD = Angle<f64>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AngleOperatorValue<S: Scalar> {
    Angle(Angle<S>),
    Scalar(S),
}

#[derive(Debug, Clone, Copy, PartialOrd, FromLua)]
pub enum Angle<S: Scalar> {
    Radians(S),
    Degrees(S)
}

impl <S: Scalar> Interpolation<S> for Angle<S> {
    fn lerp(a: Self, b: Self, t: S) -> Self {
        match a {
            Angle::Radians(a) => {
                Self::Radians(S::lerp(a, b.take_radians(), t))
            }
            Angle::Degrees(d) => {
                Self::Degrees(S::lerp(d, b.take_degrees(), t))
            }
        }
    }

    fn inverse_lerp<F: Fn(&Self) -> S>(_a: Self, _b: Self, _v: Self, _f: F) -> S {
        todo!()
    }
}

impl UserData for AngleF {
    fn add_methods<'a, M: UserDataMethods<'a, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));

        methods.add_meta_method(MetaMethod::Add, |_, this, b: AnyUserData| {
            if let Ok(i) = b.borrow::<Number>() {
                let a = *this;
                return Ok(a + *i as f32);
            }

            if let Ok(i) = b.borrow::<f64>() {
                let a = *this;
                return Ok(a + i.to_f32());
            }

            if let Ok(a) = b.borrow::<Self>() {
                return Ok(*a + *this);
            }
            Ok(*this)
        });

        methods.add_meta_method(MetaMethod::Sub, |_, this, b: AnyUserData| {
            if let Ok(i) = b.borrow::<f32>() {
                let a = *this;
                return Ok(a - *i);
            }

            if let Ok(i) = b.borrow::<f64>() {
                let a = *this;
                return Ok(a - i.to_f32());
            }

            if let Ok(a) = b.borrow::<Self>() {
                return Ok(*a - *this);
            }
            Ok(*this)
        });

        methods.add_meta_method(MetaMethod::Mul, |_, this, b: AnyUserData| {
            if let Ok(i) = b.borrow::<f32>() {
                let a = *this;
                return Ok(a * *i);
            }

            if let Ok(i) = b.borrow::<f64>() {
                let a = *this;
                return Ok(a * i.to_f32());
            }

            if let Ok(a) = b.borrow::<Self>() {
                return Ok(*a * *this);
            }
            Ok(*this)
        });

        methods.add_meta_method(MetaMethod::Add, |_, this, b: AnyUserData| {
            if let Ok(i) = b.borrow::<f32>() {
                let a = *this;
                return Ok(a / *i);
            }

            if let Ok(i) = b.borrow::<f64>() {
                let a = *this;
                return Ok(a / i.to_f32());
            }

            if let Ok(a) = b.borrow::<Self>() {
                return Ok(*a / *this);
            }
            Ok(*this)
        });

        methods.add_method("to_rad", |_, this, ()| Ok(this.to_radians()));
        methods.add_method("to_deg", |_, this, ()| Ok(this.to_degrees()));
        methods.add_method("take", |_, this, ()| Ok(this.take().to_f64()));
    }
    /*fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {

    }*/
}

impl UserData for AngleD {
    fn add_methods<'a, M: UserDataMethods<'a, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{}", this)));

        methods.add_meta_method(MetaMethod::Add, |_, this, b: AnyUserData| {
            if let Ok(i) = b.borrow::<f32>() {
                let a = *this;
                return Ok(a + i.to_f64());
            }

            if let Ok(i) = b.borrow::<f64>() {
                let a = *this;
                return Ok(a + *i);
            }

            if let Ok(a) = b.borrow::<Self>() {
                return Ok(*a + *this);
            }
            Ok(*this)
        });

        methods.add_method("to_rad", |_, this, ()| Ok(this.to_radians()));
        methods.add_method("to_deg", |_, this, ()| Ok(this.to_degrees()));
        methods.add_method("take", |_, this, ()| Ok(this.take().to_f64()));
    }
}

impl AngleF {
    pub fn load_lua(lua: &Lua) -> rlua::Result<()> {
        lua.globals().set(
            "radf",
            lua.create_function(|_, args: Number| Ok(Self::Radians(args as f32)))?,
        )?;

        lua.globals().set(
            "degf",
            lua.create_function(|_, args: Number| Ok(Self::Degrees(args as f32)))?,
        )?;

        Ok(())
    }
}

impl Angle<Number> {
    pub fn load_lua(lua: &Lua) -> rlua::Result<()> {
        lua.globals().set(
            "rad",
            lua.create_function(|_, args: Number| Ok(Self::Radians(args)))?,
        )?;

        lua.globals().set(
            "deg",
            lua.create_function(|_, args: Number| Ok(Self::Degrees(args)))?,
        )?;

        Ok(())
    }

    pub fn to_anglef(self) -> AngleF {
        match self {
            Angle::Radians(r) => AngleF::Radians(r as f32),
            Angle::Degrees(d) => AngleF::Degrees(d as f32),
        }
    }
}

impl<S: Scalar> Angle<S> {
    pub const RAD_ZERO: Self = Self::Radians(S::ZERO);
    pub const DEG_ZERO: Self = Self::Degrees(S::ZERO);

    pub fn sin(self) -> S {
        self.take().sine()
    }

    pub fn cos(self) -> S {
        self.take().cosine()
    }

    pub fn tan(self) -> S {
        self.take().tangent()
    }

    pub fn sin_cos(self) -> (S, S) {
        (self.sin(), self.cos())
    }

    pub fn sin_cos_tan(self) -> (S, S, S) {
        (self.sin(), self.cos(), self.tan())
    }

    pub fn to_radians(self) -> Self {
        match self {
            Angle::Radians(_) => self,
            Angle::Degrees(d) => Self::Radians(d.rads()),
        }
    }

    pub fn to_degrees(self) -> Self {
        match self {
            Angle::Radians(r) => Self::Degrees(r.degs()),
            Angle::Degrees(_) => self,
        }
    }

    pub fn is_degrees(self) -> bool {
        match self {
            Angle::Degrees(_) => true,
            _ => false,
        }
    }

    pub fn is_radians(self) -> bool {
        match self {
            Angle::Radians(_) => true,
            _ => false,
        }
    }

    pub fn take(self) -> S {
        match self {
            Angle::Radians(r) => r,
            Angle::Degrees(d) => d,
        }
    }

    pub fn take_radians(self) -> S {
        self.to_radians().take()
    }

    pub fn take_degrees(self) -> S {
        self.to_degrees().take()
    }
}

impl<S: Scalar> Into<AngleOperatorValue<S>> for Angle<S> {
    fn into(self) -> AngleOperatorValue<S> {
        AngleOperatorValue::Angle(self)
    }
}

impl Into<AngleOperatorValue<f32>> for f32 {
    fn into(self) -> AngleOperatorValue<f32> {
        AngleOperatorValue::Scalar(self)
    }
}

impl Into<AngleOperatorValue<f64>> for f64 {
    fn into(self) -> AngleOperatorValue<f64> {
        AngleOperatorValue::Scalar(self)
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> Add<I> for Angle<S> {
    type Output = Self;

    fn add(self, rhs: I) -> Self::Output {
        match rhs.into() {
            AngleOperatorValue::Angle(angle) => match self {
                Angle::Radians(r) => Self::Radians(r + angle.to_radians().take()),
                Angle::Degrees(d) => Self::Degrees(d + angle.to_degrees().take()),
            },
            AngleOperatorValue::Scalar(s) => match self {
                Angle::Radians(r) => Self::Radians(r + s),
                Angle::Degrees(d) => Self::Degrees(d + s),
            },
        }
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> Sub<I> for Angle<S> {
    type Output = Self;

    fn sub(self, rhs: I) -> Self::Output {
        match rhs.into() {
            AngleOperatorValue::Angle(angle) => match self {
                Angle::Radians(r) => Self::Radians(r - angle.to_radians().take()),
                Angle::Degrees(d) => Self::Degrees(d - angle.to_degrees().take()),
            },
            AngleOperatorValue::Scalar(s) => match self {
                Angle::Radians(r) => Self::Radians(r - s),
                Angle::Degrees(d) => Self::Degrees(d - s),
            },
        }
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> Mul<I> for Angle<S> {
    type Output = Self;

    fn mul(self, rhs: I) -> Self::Output {
        match rhs.into() {
            AngleOperatorValue::Angle(angle) => match self {
                Angle::Radians(r) => Self::Radians(r * angle.to_radians().take()),
                Angle::Degrees(d) => Self::Degrees(d * angle.to_degrees().take()),
            },
            AngleOperatorValue::Scalar(s) => match self {
                Angle::Radians(r) => Self::Radians(r * s),
                Angle::Degrees(d) => Self::Degrees(d * s),
            },
        }
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> Div<I> for Angle<S> {
    type Output = Self;

    fn div(self, rhs: I) -> Self::Output {
        match rhs.into() {
            AngleOperatorValue::Angle(angle) => match self {
                Angle::Radians(r) => Self::Radians(r / angle.to_radians().take()),
                Angle::Degrees(d) => Self::Degrees(d / angle.to_degrees().take()),
            },
            AngleOperatorValue::Scalar(s) => match self {
                Angle::Radians(r) => Self::Radians(r / s),
                Angle::Degrees(d) => Self::Degrees(d / s),
            },
        }
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> AddAssign<I> for Angle<S> {
    fn add_assign(&mut self, rhs: I) {
        *self = *self + rhs;
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> SubAssign<I> for Angle<S> {
    fn sub_assign(&mut self, rhs: I) {
        *self = *self - rhs;
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> MulAssign<I> for Angle<S> {
    fn mul_assign(&mut self, rhs: I) {
        *self = *self * rhs;
    }
}

impl<S: Scalar, I: Into<AngleOperatorValue<S>>> DivAssign<I> for Angle<S> {
    fn div_assign(&mut self, rhs: I) {
        *self = *self / rhs;
    }
}

impl<S: Scalar> PartialEq for Angle<S> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Angle::Radians(r) => r.eq(&other.to_radians().take()),
            Angle::Degrees(d) => d.eq(&other.to_degrees().take()),
        }
    }
}

impl<S: Scalar> Display for Angle<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Radians(r) => f.write_fmt(format_args!("{r}")),
            Angle::Degrees(d) => f.write_fmt(format_args!("{d}°")),
        }
    }
}
