#[macro_export]
macro_rules! inequality {
    (
        match $n:ident[$o:ty] {
            $($lower_less_eq:expr; <= n < $upper_less:expr => $ret:expr),*
        }
    ) => {
        {
            let mut value = <$o>::default();
            $(
            if $n >= $lower_less_eq && $n < $upper_less {
                value = $ret;
            }
            )*
            value
        }
    };
}