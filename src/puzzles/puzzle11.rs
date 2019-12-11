use crate::intcode::IntcodeMachine;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    // All non-specified tiles default to black, so an empty map is completely black
    let mut tiles: HashMap<(i64, i64), i64> = HashMap::new();
    draw(&mut tiles, program.clone());
    answers.push(format!("{}", tiles.len()));

    // Reset the tiles, but make (0, 0) white this time
    tiles.clear();
    tiles.insert((0, 0), 1);
    draw(&mut tiles, program.clone());
    // Print the shape, but because the robot doesn't visit the entire rectangle perfectly we need to force the bounding box loops
    answers.push(format!("\n{}",
                         (tiles.iter().map(|((_, y), _)| *y).min().unwrap()..=tiles.iter().map(|((_, y), _)| *y).max().unwrap()).rev() // From largest to smallest y
                             .map(|y|
                                      (tiles.iter().map(|((x, _), _)| *x).min().unwrap()..=tiles.iter().map(|((x, _), _)| *x).max().unwrap()) // From smallest to largest x
                                          .map(|x|
                                              match tiles.get(&(x, y)).unwrap_or(&0) {
                                                  0 => '░', // "Black" pixel
                                                  1 => '█', // White pixel
                                                  _ => '?', // Other, should be unreachable
                                              }
                                          ).join("") // Concat the pixels
                             ).join("\n") // Concat the lines with line breaks
    ));

    answers
}

fn draw(tiles: &mut HashMap<(i64, i64), i64>, program: Vec<i64>) {
    println!("Starting draw with HashMap {:?}", tiles);
    let mut pos = 0;
    let mut base = 0;
    let mut state = program;

    let mut cur_pos = (0, 0);
    let mut cur_dir = (0, 1);
    let mut is_halted = false;
    while !is_halted {
        let current_tile = tiles.entry(cur_pos).or_insert(0);
        let mut inputs = vec![*current_tile];
        let mut outputs: Vec<i64> = vec![];
        let mut machine = IntcodeMachine::new(&mut state, &mut inputs, &mut outputs, pos, base);
        machine.execute_until_next_op_is_input();
        pos = machine.get_pos();
        base = machine.get_base();
        is_halted = machine.is_halted();

        // Take the outputs in reverse order, since pop takes from the end
        cur_dir = match outputs.pop().unwrap_or_else(|| panic!("Machine didn't provide any output!")) {
            0 => (cur_dir.1 * -1, cur_dir.0), // Turn left
            1 => (cur_dir.1, cur_dir.0 * -1), // Turn right
            n => panic!("Machine produced invalid output `{}`", n)
        };
        *current_tile = outputs.pop().unwrap_or_else(|| panic!("Machine provided only one output!"));

        cur_pos = (cur_pos.0 + cur_dir.0, cur_pos.1 + cur_dir.1);
    }
}
