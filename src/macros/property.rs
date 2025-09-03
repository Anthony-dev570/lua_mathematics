#[macro_export]
macro_rules! property {
    ($this:ident($name:ident) -> $t:ty $path:block) => {
        pub fn $name(&$this) -> $t {
            $path
        }
    };
    ($($this:ident($name:ident) -> $t:ty $path:block)*) => {
        $(
            pub fn $name(&$this) -> $t {
                $path
            }
        )*
    };
}