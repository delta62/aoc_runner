#![feature(iterator_try_collect)]

mod error;
mod input;
mod output;
mod solution;

pub use error::{PuzzleError, Result};
pub use input::PuzzleInput;
pub use inventory;
pub use output::SolutionOutput;
pub use solution::{PuzzleAnswer, PuzzleSolution, Solution};
