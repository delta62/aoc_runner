use crate::error::Result;

/// A value which can be returned from an Advent of Code solver function
pub trait SolutionOutput {
    fn to_string(&self) -> Result<String>;
}

/// Easily implement `SolutionOutput` for types which implement `ToString`
macro_rules! output_str {
    ($($type:ty),+) => {
        $(
            impl SolutionOutput for $type {
                fn to_string(&self) -> Result<String> {
                    Ok(ToString::to_string(self))
                }
            }
        )+
    };
}

output_str![&str, String, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize];
