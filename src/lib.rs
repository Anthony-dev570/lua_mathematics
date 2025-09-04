use rlua::{Function, Lua, UserData};
use std::time::{Duration, Instant};

pub mod angle;
pub mod macros;
pub mod prelude;
pub mod scalar;
pub mod vector;

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
    use crate::vector::vec2::{Vec2D, Vec2F};
    use crate::vector::vec3::{Vec3D, Vec3F};
    use crate::LuaObject;
    use rlua::Lua;

    #[test]
    fn it_works() {
        let lua = Lua::new();
        Vec2F::load_lua(&lua).unwrap();
        Vec2D::load_lua(&lua).unwrap();

        Vec3F::load_lua(&lua).unwrap();
        Vec3D::load_lua(&lua).unwrap();

        Angle::load_lua(&lua).unwrap();

        let a = Angle::Radians(0f32);
        let b = a + 5f32;

        //println!("b: {}", b.to_degrees());

        let t = lua
            .load(
                r#"

        local a_x = 1.0
        local a_y = 1.0
        local a_z = 0.3

        local b_x = 0.0
        local b_y = 1.0
        local b_z = 0.0

        local a = vec3f(a_x, a_y, a_z)
        local b = vec3f(b_x, b_y, b_z)

        local angle = a:angle(b)
        print(tostring(angle:to_deg()))

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
