//r = v, g = t, b = p;

//r = v;
//g = t;
//b = p;

#[macro_export]
macro_rules! multi_assign {
    (
        $($a:ident = $b:ident),*;
    ) => {
        $($a = $b;)*
    };
}