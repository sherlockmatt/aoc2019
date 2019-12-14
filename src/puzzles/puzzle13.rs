use crate::intcode::IntcodeMachine;
use std::collections::HashMap;
use failure::_core::cmp::Ordering;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    let mut outputs: Vec<i64> = Vec::new();
    IntcodeMachine::new(&mut program.clone(), &mut Vec::new(), &mut outputs, 0, 0).execute_until_halt();

    let mut blocks: HashMap<(i64, i64), i64> = HashMap::new();
    parse_outputs(&mut outputs, &mut blocks);

    let mut num_blocks = blocks_by_type(&blocks, 2).len();
    answers.push(format!("{:?}", num_blocks));

    blocks = HashMap::new(); // Reset the grid
    let mut state = program.clone();
    state[0] = 2; // Pay for a play
    let mut inputs: Vec<i64> = Vec::new();
    let mut pos = 0;
    let mut base = 0;
    let mut is_halted = false;

    while !is_halted && num_blocks > 0 {
        { // Do this inside a sub-scope so the references are dropped before we try to use them
            let mut machine = IntcodeMachine::new(&mut state, &mut inputs, &mut outputs, pos, base);
            machine.execute_until_next_op_is_input();
            pos = machine.get_pos();
            base = machine.get_base();
            is_halted = machine.is_halted();
        }
        parse_outputs(&mut outputs, &mut blocks);
        num_blocks = blocks_by_type(&blocks, 2).len();

        inputs.push(match (blocks_by_type(&blocks, 3)[0].0).0.cmp(&(blocks_by_type(&blocks, 4)[0].0).0) {
            Ordering::Less => 1, // If the bat's x is less than the ball's x, move right
            Ordering::Equal => 0, // If the bat is directly under the ball, don't move
            Ordering::Greater => -1 // If the bat's x is more then the ball's x, move left
        });
    }

    answers.push(format!("{}", blocks.get(&(-1, 0)).unwrap()));

    answers
}

fn parse_outputs(outputs: &mut Vec<i64>, blocks: &mut HashMap<(i64, i64), i64>) {
    outputs.reverse();
    while outputs.len() > 0 {
        let x = outputs.pop().unwrap();
        let y = outputs.pop().unwrap();
        let block_type = outputs.pop().unwrap();
        blocks.insert((x, y), block_type);
    }
    assert_eq!(0, outputs.len());
}

fn blocks_by_type(blocks: &HashMap<(i64, i64), i64>, block_type: i64) -> Vec<((i64, i64), i64)>{
    blocks.iter().filter(|((_, _), t)| **t == block_type).map(|(p, t)| (*p, *t)).collect()
}
