#[macro_export]
macro_rules! operators {
    (
        $t:ty {
            consts {
                $($generic_const:tt: $generic_const_ty:ty),*
            }
            generics {
                $($generic_param2:ident: $($generic_req:ident),*),*
            }
            +($plus_self:ident) {
                $($add_type:ty => $add_output:ty $add_block:block)*
            }
            - {

            }
        }
    ) => {
        $(
            impl <$(const $generic_const: $generic_const_ty,),* $($generic_param2: $($generic_req),*),*> std::ops::Add<$add_type> for $t {
                type Output = $add_output

                fn add(self, rhs: $plus_self) -> $add_type {
                    $add_block
                }
            }
        )*
    };
}