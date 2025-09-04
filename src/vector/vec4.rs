use crate::{lua_vector, lua_vector_methods};
use crate::vector::Vector;

pub type Vec4<S> = Vector<4, S>;
pub type Vec4F = Vec4<f32>;
pub type Vec4D = Vec4<f64>;

lua_vector!(Vec4F[f32] {
    Args = (Option<f32>, Option<f32>, Option<f32>, Option<f32>),
    CONSTRUCTOR_NAME = "vec4f",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args| {
            Ok(Self::from_array([
                args.0.unwrap_or(0f32),
                args.1.unwrap_or(0f32),
                args.2.unwrap_or(0f32),
                args.3.unwrap_or(0f32),
            ]))
        })
    },
    associated_functions = (lua) [
        fn vec4f_zero(_args: ()) {
            Ok(Self::ZERO)
        }
        fn vec4f_one(_args: ()) {
            Ok(Self::ONE)
        }
    ]
    methods = (methods) {

    }
});