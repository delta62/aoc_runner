use crate::{error::Result, PuzzleError};
use std::error::Error;

/// A value which can be returned from an Advent of Code solver function
pub trait SolutionOutput {
    fn to_string(self) -> Result<String>;
}

/// Easily implement `SolutionOutput` for types which implement `ToString`
macro_rules! output_str {
    ($($type:ty),+) => {
        $(
            impl SolutionOutput for $type {
                fn to_string(self) -> Result<String> {
                    Ok(ToString::to_string(&self))
                }
            }
        )+
    };
}

output_str![&str, String, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize];

impl<T, E> SolutionOutput for std::result::Result<T, E>
where
    T: ToString,
    E: Error,
{
    fn to_string(self) -> Result<String> {
        self.map(|x| x.to_string())
            .map_err(|e| PuzzleError::SolutionError(format!("Error while running solution: {e}")))
    }
}
