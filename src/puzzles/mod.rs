use failure::_core::ops::Index;
use failure::Error;

mod puzzle01;
mod puzzle02;

pub fn run(puzzle_number: usize, input: String) -> Result<Vec<String>, Error> {
    let puzzle_functions: [&dyn Fn(String) -> Vec<String>; 2] = [
        &puzzle01::run,
        &puzzle02::run,
    ];

    ensure!(puzzle_number <= puzzle_functions.len(), "Puzzle number {} not found", puzzle_number);

    return Ok(puzzle_functions.index(puzzle_number - 1usize)(input));
}
