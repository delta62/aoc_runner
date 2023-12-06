use crate::error::Result;

/// A value which can be returned from an Advent of Code solver function
pub trait SolutionOutput {
    fn to_string(&self) -> Result<String>;
}

impl SolutionOutput for String {
    fn to_string(&self) -> Result<String> {
        Ok(ToString::to_string(self))
    }
}

impl SolutionOutput for u32 {
    fn to_string(&self) -> Result<String> {
        Ok(ToString::to_string(self))
    }
}

impl SolutionOutput for usize {
    fn to_string(&self) -> Result<String> {
        Ok(ToString::to_string(self))
    }
}
