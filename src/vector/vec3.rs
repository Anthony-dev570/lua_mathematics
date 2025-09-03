use crate::scalar::Scalar;
use crate::vector::Vector;
use crate::{lua_vector, property};

pub type Vec3<S> = Vector<3, S>;
pub type Vec3F = Vec3<f32>;
pub type Vec3D = Vec3<f64>;

impl<S: Scalar> Vec3<S> {
    property!(
        self(x) -> S { self.0[0] }
        self(y) -> S { self.0[1] }
        self(z) -> S { self.0[2] }
        self(x_y_z) -> (S, S, S) { (self.0[0], self.0[1], self.0[2]) }
    );

    pub fn cross(&self, v: &Self) -> Self {
        let (ax, ay, az) = self.x_y_z();
        let (bx, by, bz) = v.x_y_z();

        Self::from_array([
            ay * bz - az * by,
            az * bx - ax * bz,
            ax * by - ay * bx
        ])
    }
}

use crate::lua_vector_methods;

lua_vector!(Vec3F {
    Args = (Option<f32>, Option<f32>, Option<f32>),
    CONSTRUCTOR_NAME = "vec3f",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args| {
            Ok(Self::from_array([
                args.0.unwrap_or(0f32),
                args.1.unwrap_or(0f32),
                args.2.unwrap_or(0f32),
            ]))
        })
    },
    associated_functions = (lua) [
        fn vec3f_zero(_args: ()) {
            Ok(Self::ZERO)
        }
        fn vec3f_one(_args: ()) {
            Ok(Self::ONE)
        }
    ]
    methods = (methods) {
        fn cross_product(this, b: Self) {
            Ok(this.cross(&b))
        }
    }
});

lua_vector!(Vec3D {
    Args = (Option<f64>, Option<f64>, Option<f64>),
    CONSTRUCTOR_NAME = "vec3f",
    create_constructor = (lua) {
        lua.create_function(|_, args: Self::Args| {
            Ok(Self::from_array([
                args.0.unwrap_or(0f64),
                args.1.unwrap_or(0f64),
                args.2.unwrap_or(0f64),
            ]))
        })
    },
    associated_functions = (lua) [
        fn vec3d_zero(_args: ()) {
            Ok(Self::ZERO)
        }
        fn vec3d_one(_args: ()) {
            Ok(Self::ONE)
        }
    ]
    methods = (methods) {
        fn cross_product(this, b: Self) {
            Ok(this.cross(&b))
        }
    }
});
