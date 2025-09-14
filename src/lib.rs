#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use rlua::{Function, Lua, UserData};
use std::time::{Duration, Instant};

pub mod angle;
pub mod euler;
pub mod macros;
pub mod matrix;
pub mod prelude;
pub mod quaternion;
pub mod scalar;
pub mod vector;
pub mod interpolation;

pub struct LuaAssociatedFunction<'a, 'lua> {
    pub function: Function<'lua>,
    pub name: &'a str,
}

pub trait LuaObject: UserData {
    type Args;
    const CONSTRUCTOR_NAME: &'static str;
    fn create_constructor(lua: &Lua) -> rlua::Result<Function>;

    fn associated_functions(lua: &Lua) -> rlua::Result<Vec<LuaAssociatedFunction>>;

    fn load_lua(lua: &Lua) -> rlua::Result<()> {
        let constructor = Self::create_constructor(lua)?;
        lua.globals().set(Self::CONSTRUCTOR_NAME, constructor)?;

        for associated_function in Self::associated_functions(lua)? {
            lua.globals()
                .set(associated_function.name, associated_function.function)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::Angle;
    use crate::euler::EulerF;
    use crate::matrix::matrix2x2::Matrix2x2F;
    use crate::matrix::matrix3x3::Matrix3x3F;
    use crate::quaternion::QuatF;
    use crate::vector::vec2::{Vec2D, Vec2F};
    use crate::vector::vec3::{Vec3D, Vec3F};
    use crate::vector::vec4::Vec4F;
    use crate::LuaObject;
    use rlua::Lua;
    use crate::matrix::matrix4x4::Matrix4x4F;

    #[test]
    fn it_works() {
        let lua = Lua::new();
        Vec2F::load_lua(&lua).unwrap();
        Vec2D::load_lua(&lua).unwrap();

        Vec3F::load_lua(&lua).unwrap();
        Vec3D::load_lua(&lua).unwrap();

        Vec4F::load_lua(&lua).unwrap();

        //Angle::load_lua(&lua).unwrap();

        Matrix2x2F::load_lua(&lua).unwrap();
        Matrix3x3F::load_lua(&lua).unwrap();
        Matrix4x4F::load_lua(&lua).unwrap();

        EulerF::from_lua(&lua).unwrap();

        QuatF::load_lua(&lua).unwrap();

        let t = lua
            .load(
                r#"
        local left = 0
        local right = 500
        local bottom = 0
        local top = 500
        local near = -1
        local far = 1

        local ms = mat4x4f_ortho(left, right, bottom, top, near, far)

        print(tostring(ms))

        "#,
            )
            .exec();
        match t {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
            }
        }
    }
}

pub fn benchmark<F: Fn()>(f: F, times: usize) -> Duration {
    let i = Instant::now();
    for _ in 0..times {
        f();
    }
    i.elapsed() / times as u32
}
