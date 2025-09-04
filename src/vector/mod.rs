use crate::scalar::Scalar;
use rlua::FromLua;

pub mod imp;
pub mod vec2;
pub mod vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromLua)]
#[repr(C)]
pub struct Vector<const L: usize, S: Scalar>([S; L]);

#[macro_export]
macro_rules! lua_vector_methods {
    ($methods:ident) => {
        $methods.add_meta_method(rlua::MetaMethod::ToString, |_, this, _: ()| Ok(this.to_string()));
        $methods.add_meta_method(rlua::MetaMethod::Len, |_, this, _: ()| Ok(this.magnitude()));

        $methods.add_meta_method(rlua::MetaMethod::Index, |_, this, index: rlua::AnyUserData| {
            //Ok(this[index])
            if let Ok(index) = index.borrow::<usize>() {
                return Ok(Some(this[*index]));
            } else {
                panic!("Index type not allowed: {index:?}");
            }
        });

        $methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, index: Self| Ok(*this == index));
        $methods.add_meta_method(rlua::MetaMethod::Add, |_, this, index: rlua::AnyUserData| {
            if let Ok(index) = index.borrow::<Self>() {
                return Ok(*this + *index);
            }
            if let Ok(index) = index.borrow::<rlua::Integer>() {
                return Ok(*this + (*index as f64));
            }
            if let Ok(index) = index.borrow::<rlua::Number>() {
                return Ok(*this + *index);
            }
            Ok(*this)
        });

        $methods.add_meta_method(rlua::MetaMethod::Sub, |_, this, index: rlua::AnyUserData| {
            if let Ok(index) = index.borrow::<Self>() {
                return Ok(*this - *index);
            }
            if let Ok(index) = index.borrow::<rlua::Integer>() {
                return Ok(*this - (*index as f64));
            }
            if let Ok(index) = index.borrow::<rlua::Number>() {
                return Ok(*this - *index);
            }
            Ok(*this)
        });
        $methods.add_meta_method(rlua::MetaMethod::Mul, |_, this, index: rlua::AnyUserData| {
            if let Ok(index) = index.borrow::<rlua::Integer>() {
                return Ok(*this * (*index as f64));
            }
            if let Ok(index) = index.borrow::<rlua::Number>() {
                return Ok(*this * *index);
            }
            Ok(*this)
        });
        $methods.add_meta_method(rlua::MetaMethod::Unm, |_, this, _: ()| Ok(-*this));
        $methods.add_method("magnitude", |_, this, _: ()| Ok(this.magnitude()));
        $methods.add_method("dot", |_, this, b: Self| Ok(this.dot_product(&b)));
        $methods.add_method("norm", |_, this, _: ()| Ok(this.normalized()));
        $methods.add_method("angle", |_, this, b: Self| Ok(this.angle_between(&b)));
    };
}

#[macro_export]
macro_rules! lua_vector {
    (
        $t:ty {
            Args = $args:ty,
            CONSTRUCTOR_NAME = $constructor_name:literal,
            create_constructor = ($lua:ident) $constructor_block:block,
            associated_functions = ($lua_functions:ident) [
                $(fn $func_name:ident($func_args_name:ident: $func_args_ty:ty) $func_block:block)*
            ]
            methods = ($methods:ident) {
                $(fn $method_name:ident($this:ident, $method_args:ident: $method_args_type:ty) $method_block:block)*
            }
        }
    ) => {
        impl rlua::UserData for $t {
            fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>($methods: &mut M) {
                lua_vector_methods!($methods);
                $(
                    $methods.add_method(stringify!($method_name), |_, $this, $method_args: $method_args_type| $method_block);
                )*
            }
        }

        impl crate::LuaObject for $t {
            type Args = $args;
            const CONSTRUCTOR_NAME: &'static str = $constructor_name;

            fn create_constructor($lua: &rlua::Lua) -> mlua::Result<rlua::Function> {
                $constructor_block
            }

            fn associated_functions($lua_functions: &rlua::Lua) -> mlua::Result<Vec<crate::LuaAssociatedFunction>> {
                paste::paste! {
                    Ok(vec![
                    $(
                        crate::LuaAssociatedFunction {
                            function: $lua_functions.create_function(|_, $func_args_name: $func_args_ty| $func_block)?,
                            name: stringify!($func_name)
                        }
                    ),*
                    ])
                }
            }
        }
    };
}