use std::time::{Duration, Instant};

pub mod scalar;
pub mod vector;
pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::prelude::vec2f;

    #[test]
    fn it_works() {
        let v = vec2f(1.0, 2.0);
    }
}

pub fn benchmark<F: Fn()>(f: F, times: usize) -> Duration {
    let i = Instant::now();
    for _ in 0..times {
        f();
    }
    i.elapsed() / times as u32
}