use crate::vector::Vector;
use crate::{lua_vector, lua_vector_methods};

pub type Vec2<S> = Vector<2, S>;
pub type Vec2F = Vec2<f32>;
pub type Vec2D = Vec2<f64>;

lua_vector!(Vec2F {
    Args = (Option<f32>, Option<f32>),
    CONSTRUCTOR_NAME = "vec2f",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args| {
            Ok(Self::from_array([
                args.0.unwrap_or(0f32),
                args.1.unwrap_or(0f32),
            ]))
        })
    },
    associated_functions = (lua) [
        fn vec2f_zero(_args: ()) {
            Ok(Self::ZERO)
        }
        fn vec2f_one(_args: ()) {
            Ok(Self::ONE)
        }
    ]
    methods = (methods) {

    }
});

lua_vector!(Vec2D {
    Args = (Option<f64>, Option<f64>),
    CONSTRUCTOR_NAME = "vec2d",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args| {
            Ok(Self::from_array([
                args.0.unwrap_or(0f64),
                args.1.unwrap_or(0f64),
            ]))
        })
    },
    associated_functions = (lua) [
        fn vec2d_zero(_args: ()) {
            Ok(Self::ZERO)
        }
        fn vec2d_one(_args: ()) {
            Ok(Self::ONE)
        }
    ]
    methods = (methods) {
        
    }
});