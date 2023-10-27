mod Hexgem;

use std::fmt::Display;

pub use Hexgem::*;

trait Unwrap<T> {
    fn get(self, message: &str) -> T;
}

impl<T, E: Display> Unwrap<T> for Result<T, E> {
    fn get(self, message: &str) -> T {
        match self {
            Ok(result) => result,
            Err(err) => {
                error!("{} {}", message, err);
                panic!("{}", message);
            }
        }
    }
}
