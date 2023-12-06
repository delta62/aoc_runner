use crate::{error::Result, input::PuzzleInput, output::SolutionOutput};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct PuzzleAnswer {
    /// The amount of time it took to parse from the raw input string to a data type
    /// required by the solver
    pub parse_duration: Duration,

    /// The amount of time it took to run the solver on the pre-parsed input
    pub solve_duration: Duration,

    /// The result of running the solver
    pub result: Result<String>,
}

/// A generalized Advent of Code solver. You will generally want to
/// implement [`PuzzleSolution`] for your types, and the runtime will
/// automatically convert between those types and this one. The advantage
/// of `PuzzleSolution` over this more generalized trait are the dynamic
/// input and output types - the runtime will be able to automatically parse
/// inputs and handle errors gracefully.
pub trait Solution {
    /// The year this solution is for
    fn year(&self) -> u16;

    /// The day of advent (1-25, implicitly in December) that this solution is for
    fn day(&self) -> u8;

    /// The part of the day (1 or 2) that this solution is for
    fn part(&self) -> u8;

    /// Attempt to solve for the given input, returning the result along with
    /// timing metrics related to each phase of the solution
    fn solve(&self, input: &str) -> PuzzleAnswer;
}

inventory::collect!(&'static (dyn Solution + Sync));

impl<T> Solution for T
where
    T: PuzzleSolution,
{
    fn year(&self) -> u16 {
        self.year()
    }

    fn day(&self) -> u8 {
        self.day()
    }

    fn part(&self) -> u8 {
        self.part()
    }

    fn solve(&self, input: &str) -> PuzzleAnswer {
        let start_time = Instant::now();

        let input = <T as PuzzleSolution>::Input::parse(input);
        match input {
            Ok(input) => {
                let parse_duration = Instant::now().duration_since(start_time);
                let result = self.solve(input).to_string();
                let solve_duration = Instant::now().duration_since(start_time) - parse_duration;

                PuzzleAnswer {
                    parse_duration,
                    solve_duration,
                    result,
                }
            }
            Err(err) => PuzzleAnswer {
                parse_duration: Default::default(),
                solve_duration: Default::default(),
                result: Err(err),
            },
        }
    }
}

pub trait PuzzleSolution {
    /// The type of input required to run this solution
    type Input<'a>: PuzzleInput<'a>;

    /// The result of running this solution
    type Output: SolutionOutput;

    /// The year this solution is for
    fn year(&self) -> u16;

    /// The day of advent (1-25, implicitly in December) that this solution is for
    fn day(&self) -> u8;

    /// The part of the day (1 or 2) that this solution is for
    fn part(&self) -> u8;

    /// Attempt to solve for the given input
    fn solve<'i>(&self, input: Self::Input<'i>) -> Self::Output;
}
