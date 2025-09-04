//condition ?
#[macro_export]
macro_rules! ternary {
    ($c:expr => $a:expr ; $b:expr) => {
        match $c {
            true => $a,
            false => $b
        }
    };
}