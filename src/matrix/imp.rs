use crate::matrix::Matrix;
use crate::scalar::Scalar;
use crate::vector::Vector;
use std::ffi::c_void;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Debug, Clone, Copy)]
pub enum MatrixInitializer<const R: usize, const C: usize, S: Scalar> {
    Vectors([Option<Vector<C, S>>; R]),
    Identity,
}

impl<const R: usize, const C: usize, S: Scalar> Matrix<R, C, S> {
    pub const ZERO: Self = Self([Vector::ZERO; R]);
    pub const ONE: Self = Self([Vector::ONE; R]);

    pub const fn from_array(array: [Vector<C, S>; R]) -> Self {
        Self(array)
    }

    pub fn as_ptr(&self) -> *const S {
        self.0[0].as_ptr()
    }

    pub fn as_c_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }

    pub fn longest_char(&self) -> (usize, usize, usize) {
        let mut length = 0;
        let mut length_pos = (0, 0);

        for row in 0..R {
            for col in 0..C {
                let current_length = self[row][col].to_string().len();
                if current_length >= length {
                    length_pos = (row, col);
                    length = current_length;
                }
            }
        }

        (length_pos.0, length_pos.1, length)
    }

    pub fn transpose(&self) -> Matrix<C, R, S> {
        let mut out = Matrix::ZERO;

        for row in 0..R {
            for col in 0..C {
                out[col][row] = self[row][col];
            }
        }

        out
    }
}

impl<const R: usize, const C: usize, S: Scalar> Index<usize> for Matrix<R, C, S> {
    type Output = Vector<C, S>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const R: usize, const C: usize, S: Scalar> IndexMut<usize> for Matrix<R, C, S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const R: usize, const C: usize, S: Scalar> Add<Self> for Matrix<R, C, S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Matrix::ZERO;
        for row in 0..R {
            for col in 0..C {
                out[col][row] = self[row][col] + rhs[row][col];
            }
        }
        out
    }
}

impl<const R: usize, const C: usize, S: Scalar> Sub<Self> for Matrix<R, C, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = Matrix::ZERO;
        for row in 0..R {
            for col in 0..C {
                out[col][row] = self[row][col] - rhs[row][col];
            }
        }
        out
    }
}

#[macro_export]
macro_rules! lua_matrix_methods {
    ($methods:ident) => {};
}

#[macro_export]
macro_rules! lua_matrix {
    (
        $t:ty => $f:ty {
            Args = $args:ty,
            CONSTRUCTOR_NAME = $constructor_name:literal,
            create_constructor = ($lua:ident) $constructor_block:block
            associated_functions = ($lua_functions:ident) [
                $(fn $func_name:ident($func_args_name:ident: $func_args_ty:ty) $func_block:block)*
            ]
            methods = {
                $(fn $method_name:ident($this:ident, $method_args:ident: $method_args_type:ty) $method_block:block)*
            }
            meta_method = {
                $($meta_method:path[$meta_this:ident] => $arg_name:ident: $arg_ty:ty $meta_method_body:block)*
            }
        }
    ) => {
        impl rlua::UserData for $t {
            fn add_methods<M: rlua::UserDataMethods<Self>>(methods: &mut M) {
                methods.add_meta_method(rlua::MetaMethod::ToString, |_, this, _: ()| Ok(this.to_string()));
                methods.add_meta_method(rlua::MetaMethod::Index, |_, this, index: usize| Ok(this[index]));
                methods.add_method_mut("set", |_, this, (c, r, v): (usize, usize, $f)| {
                    this[r][c] = v;
                    Ok(())
                });

                methods.add_method("transpose", |_, this, _: ()| {
                    Ok(this.transpose())
                });

                $(
                    methods.add_method(stringify!($method_name), |_, $this, $method_args: $method_args_type| $method_block);
                )*

                $(
                    methods.add_meta_method($meta_method, |_, $meta_this, $arg_name: $arg_ty| {
                        $meta_method_body
                    });
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

impl<const R: usize, const C: usize, S: Scalar> Display for Matrix<R, C, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (_a, _b, longest_char) = self.longest_char();
        let center = (0..R)
            .into_iter()
            .map(|r| {
                let center = (0..C)
                    .into_iter()
                    .map(|c| {
                        let a = self[r][c].to_string();
                        let d = longest_char - a.len() + 1;
                        format!("{:>d$}", a)
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                let first = crate::ternary!(
                    r == 0 => "⎡" ; crate::ternary!(r == R - 1 => "⎣" ; "⎥")
                );
                let last = crate::ternary!(
                    r == 0 => "⎤" ; crate::ternary!(r == R - 1 => "⎦" ; "⎥")
                );
                format!("{first}{center}{last}")
            })
            .collect::<Vec<String>>()
            .join("\n");
        f.write_fmt(format_args!("{center}"))
    }
}
