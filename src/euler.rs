use mlua::{AnyUserData, FromLua, Integer, Lua, MetaMethod, Number, UserData, UserDataMethods};
use crate::angle::{Angle, AngleD, AngleF};
use crate::quaternion::Quaternion;
use crate::scalar::Scalar;

pub type EulerF = Euler<f32>;

#[derive(Debug, Clone, Copy, PartialEq, FromLua)]
pub struct Euler<S: Scalar> {
    pub yaw: Angle<S>,
    pub pitch: Angle<S>,
    pub roll: Angle<S>,
    pub debug_flag: u8
}

impl<S: Scalar> Euler<S> {
    ///Returns the 3 components in (roll, pitch, yaw) form.
    pub fn take(self) -> (S, S, S) {
        (self.roll.take(), self.pitch.take(), self.yaw.take())
    }

    pub fn take_radians(self) -> (S, S, S) {
        self.to_radians().take()
    }

    pub fn take_degrees(self) -> (S, S, S) {
        self.to_degrees().take()
    }

    pub fn to_radians(self) -> Self {
        let mut out = self.clone();
        out.roll = out.roll.to_radians();
        out.yaw = out.yaw.to_radians();
        out.pitch = out.pitch.to_radians();
        out
    }

    pub fn to_degrees(self) -> Self {
        let mut out = self.clone();
        out.roll = out.roll.to_degrees();
        out.yaw = out.yaw.to_degrees();
        out.pitch = out.pitch.to_degrees();
        out
    }

    pub fn radify(&mut self) {
        *self = self.to_radians();
    }

    pub fn degify(&mut self) {
        *self = self.to_degrees();
    }

    pub fn to_quat(self) -> Quaternion<S> {
        Quaternion::from_euler(self)
    }
}

impl EulerF {
    pub const DEBUG_FLAG_NONE: u8 = 0;
    pub const DEBUG_FLAG_GREEK_A: u8 = 1;
    pub const DEBUG_FLAG_GREEK_B: u8 = 2;

    pub fn from_lua(lua: &Lua) -> rlua::Result<()> {
        let t = lua.create_function(|_, (yaw, pitch, roll): (Option<AnyUserData>, Option<AnyUserData>, Option<AnyUserData>)| {
            let yaw = yaw.map(|e| {
                if let Ok(e) = e.borrow::<AngleF>() {
                    return *e;
                }

                if let Ok(e) = e.borrow::<AngleD>() {
                    return e.to_anglef();
                }

                if let Ok(e) = e.borrow::<Number>() {
                    return Angle::Radians(e.to_f32());
                }

                if let Ok(e) = e.borrow::<Integer>() {
                    return Angle::Radians(*e as f32);
                }

                Angle::RAD_ZERO
            }).unwrap_or(Angle::RAD_ZERO);
            let pitch = pitch.map(|e| {
                if let Ok(e) = e.borrow::<AngleF>() {
                    return *e;
                }

                if let Ok(e) = e.borrow::<AngleD>() {
                    return e.to_anglef();
                }

                if let Ok(e) = e.borrow::<Number>() {
                    return Angle::Radians(e.to_f32());
                }

                if let Ok(e) = e.borrow::<Integer>() {
                    return Angle::Radians(*e as f32);
                }

                Angle::RAD_ZERO
            }).unwrap_or(Angle::RAD_ZERO);

            let roll = roll.map(|e| {
                if let Ok(e) = e.borrow::<AngleF>() {
                    return *e;
                }

                if let Ok(e) = e.borrow::<AngleD>() {
                    return e.to_anglef();
                }

                if let Ok(e) = e.borrow::<Number>() {
                    return Angle::Radians(e.to_f32());
                }

                if let Ok(e) = e.borrow::<Integer>() {
                    return Angle::Radians(*e as f32);
                }

                Angle::RAD_ZERO
            }).unwrap_or(Angle::RAD_ZERO);

            Ok(Self {
                yaw,
                pitch,
                roll,
                debug_flag: Self::DEBUG_FLAG_GREEK_A,
            })
        })?;
        lua.globals().set("eulerf", t)
    }
}

impl UserData for EulerF {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        _methods.add_meta_method(MetaMethod::ToString, |_lua, this, ()| {
            Ok(match this.debug_flag {
                Euler::DEBUG_FLAG_GREEK_A => format!("[α: {}, β: {}, γ: {}]", this.roll, this.pitch, this.yaw),
                Euler::DEBUG_FLAG_GREEK_B => format!("[ϕ: {}, θ: {}, ψ: {}]", this.roll, this.pitch, this.yaw),
                _ => format!("<{}, {}, {}>", this.roll, this.pitch, this.yaw),
            })
        });

        _methods.add_method_mut("debug_flag", |_lua, this, flag_value: Integer| {
            this.debug_flag = flag_value as u8;
            Ok(())
        });

        _methods.add_method("to_quat", |_lua, this, ()| Ok(this.to_quat()));
    }
}