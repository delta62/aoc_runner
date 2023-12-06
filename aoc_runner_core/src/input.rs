use crate::error::{PuzzleError, Result};

/// A type which can be provided to an Advent of Code solver function
pub trait PuzzleInput<'a>: Sized {
    /// Attempt to parse an instance of this type from raw puzzle input
    fn parse(input: &'a [u8]) -> Result<Self>;
}

impl<'a> PuzzleInput<'a> for &'a str {
    fn parse(input: &'a [u8]) -> Result<Self> {
        std::str::from_utf8(input)
            .map_err(|_| PuzzleError::ParseError("Input is not valid UTF-8".to_owned()))
    }
}

impl<'a, T> PuzzleInput<'a> for Vec<T>
where
    T: PuzzleInput<'a>,
{
    fn parse(input: &'a [u8]) -> Result<Self> {
        let mut lines: Vec<_> = input.split(|x| *x == b'\n').collect();

        if let Some(&[]) = lines.last() {
            lines.pop(); // Empty line at end of file
        }

        lines.into_iter().map(|line| T::parse(line)).try_collect()
    }
}
