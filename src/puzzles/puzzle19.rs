use crate::intcode::IntcodeMachine;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    answers.push(format!("{}", (0..50).cartesian_product(0..50).map(|(x, y)| {
        let mut outputs = Vec::new();
        IntcodeMachine::new(&mut program.clone(), &mut vec![x, y], &mut outputs, 0, 0).execute_until_halt();
        outputs[0]
    }).sum::<i64>()));

    let mut left_edge = 0;
    let mut right_edge = 0;
    let mut current = 0;

    // Get the distance to the tractor beam's edge at x=100 and y=100, to get the gradients of the two lines
    while current == 0 {
        left_edge += 1;
        let mut outputs = Vec::new();
        IntcodeMachine::new(&mut program.clone(), &mut vec![100, left_edge], &mut outputs, 0, 0).execute_until_halt();
        current = outputs[0];
    }
    current = 0;
    while current == 0 {
        right_edge += 1;
        let mut outputs = Vec::new();
        IntcodeMachine::new(&mut program.clone(), &mut vec![right_edge, 100], &mut outputs, 0, 0).execute_until_halt();
        current = outputs[0];
    }

    // Calculate the intersect of the two lines
    let start_x = (100 * left_edge * ((right_edge * 99) / 100 + 99)) / (10_000 - (right_edge * left_edge));
    let start_y = ((start_x + 99) * right_edge) / 100;

    // Search slightly around that point to compensate for rounding errors
    'outer: for y in (start_y - 2)..=(start_y + 2) {
        for x in (start_x - 2)..=(start_x + 2) {
            if {
                let mut outputs = Vec::new();
                IntcodeMachine::new(&mut program.clone(), &mut vec![y, x + 99], &mut outputs, 0, 0).execute_until_halt();
                outputs[0]
            } == 1 &&
            {
                let mut outputs = Vec::new();
                IntcodeMachine::new(&mut program.clone(), &mut vec![y, x + 100], &mut outputs, 0, 0).execute_until_halt();
                outputs[0]
            } == 0 &&
            {
                let mut outputs = Vec::new();
                IntcodeMachine::new(&mut program.clone(), &mut vec![y + 99, x], &mut outputs, 0, 0).execute_until_halt();
                outputs[0]
            } == 1 &&
            {
                let mut outputs = Vec::new();
                IntcodeMachine::new(&mut program.clone(), &mut vec![y + 100, x], &mut outputs, 0, 0).execute_until_halt();
                outputs[0]
            } == 0 {
                answers.push(format!("{}{:04}", x, y));
                break 'outer;
            }
        }
    }

    answers
}
