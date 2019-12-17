use crate::intcode::IntcodeMachine;
use std::collections::HashMap;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let mut program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    let mut outputs = Vec::new();
    IntcodeMachine::new(&mut program.clone(), &mut Vec::new(), &mut outputs, 0, 0).execute_until_halt();

    let width = outputs.iter().position(|i| *i == 10).unwrap();
    let height = outputs.len() / (width + 1);

    let mut scaffold: HashMap<(usize, usize), char> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            scaffold.insert((x, y), match outputs[0] {
                46 => '.',
                35 => '#',
                62 => '<',
                64 => '>',
                94 => '^',
                118 => 'v',
                88 => 'X',
                _ => '?'
            });
            outputs.remove(0);
        }
        outputs.remove(0);
    }

    let mut cum_sum = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if *scaffold.get(&(x, y)).unwrap() == '#' &&
                *scaffold.get(&(x + 1, y)).unwrap() == '#' &&
                *scaffold.get(&(x - 1, y)).unwrap() == '#' &&
                *scaffold.get(&(x, y + 1)).unwrap() == '#' &&
                *scaffold.get(&(x, y - 1)).unwrap() == '#' {
                cum_sum += x * y;
            }
        }
    }

    answers.push(format!("{}", cum_sum));

    // Today's I've just done by hand. The search space is very large, and I can't see a way to avoid brute forcing it, so I can't be bothered.
    // My solution:
    // A,A,B,C,B,A,C,B,C,A
    // L,6,R,12,L,6,L,8,L,8
    // L,6,R,12,R,8,L,8
    // L,4,L,4,L,6
    // n
    //
    // 10,
    // 110, 10,
    // 54, 44, 76, 44, 52, 44, 76, 44, 52, 44, 76, 10,
    // 56, 44, 76, 44, 56, 44, 82, 44, 50, 49, 44, 82, 44, 54, 44, 76, 10,
    // 56, 44, 76, 44, 56, 44, 76, 44, 54, 44, 76, 44, 50, 49, 44, 82, 44, 54, 44, 76, 10,
    // 65, 44, 67, 44, 66, 44, 67, 44, 65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 65

    outputs.clear();
    program[0] = 2;
    IntcodeMachine::new(&mut program.clone(), &mut vec![10, 110,
                                                        10, 54, 44, 76, 44, 52, 44, 76, 44, 52, 44, 76,
                                                        10, 56, 44, 76, 44, 56, 44, 82, 44, 50, 49, 44, 82, 44, 54, 44, 76,
                                                        10, 56, 44, 76, 44, 56, 44, 76, 44, 54, 44, 76, 44, 50, 49, 44, 82, 44, 54, 44, 76,
                                                        10, 65, 44, 67, 44, 66, 44, 67, 44, 65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 65
    ], &mut outputs, 0, 0).execute_until_halt();

    answers.push(format!("{:?}", outputs[outputs.len() - 1]));

    answers
}
