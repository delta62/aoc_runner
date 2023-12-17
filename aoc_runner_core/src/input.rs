use crate::error::Result;

/// A type which can be provided to an Advent of Code solver function
pub trait PuzzleInput<'a>: Sized {
    /// Attempt to parse an instance of this type from raw puzzle input
    fn parse(input: &'a str) -> Result<Self>;
}

impl<'a> PuzzleInput<'a> for &'a str {
    fn parse(input: &'a str) -> Result<Self> {
        Ok(input)
    }
}

impl<'a, T> PuzzleInput<'a> for Vec<T>
where
    T: PuzzleInput<'a>,
{
    fn parse(input: &'a str) -> Result<Self> {
        let mut ret = Vec::new();
        for line in input.lines() {
            ret.push(T::parse(line)?);
        }

        Ok(ret)
    }
}
