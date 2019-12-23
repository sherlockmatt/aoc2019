use failure::_core::ops::Index;
use failure::Error;

mod puzzle01;
mod puzzle02;
mod puzzle03;
mod puzzle04;
mod puzzle05;
mod puzzle06;
mod puzzle07;
mod puzzle08;
mod puzzle09;
mod puzzle10;
mod puzzle11;
mod puzzle12;
mod puzzle13;
mod puzzle14;
mod puzzle15;
mod puzzle16;
mod puzzle17;
mod puzzle18;
mod puzzle19;
mod puzzle20;
mod puzzle21;

pub fn run(puzzle_number: usize, input: String) -> Result<Vec<String>, Error> {
    let puzzle_functions: [&dyn Fn(String) -> Vec<String>; 21] = [
        &puzzle01::run,
        &puzzle02::run,
        &puzzle03::run,
        &puzzle04::run,
        &puzzle05::run,
        &puzzle06::run,
        &puzzle07::run,
        &puzzle08::run,
        &puzzle09::run,
        &puzzle10::run,
        &puzzle11::run,
        &puzzle12::run,
        &puzzle13::run,
        &puzzle14::run,
        &puzzle15::run,
        &puzzle16::run,
        &puzzle17::run,
        &puzzle18::run,
        &puzzle19::run,
        &puzzle20::run,
        &puzzle21::run,
    ];

    ensure!(puzzle_number <= puzzle_functions.len(), "Puzzle number {} not found", puzzle_number);

    return Ok(puzzle_functions.index(puzzle_number - 1usize)(input));
}
