use crate::intcode::IntcodeMachine;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();
    let mut outputs: Vec<i64> = Vec::new();

    IntcodeMachine::new(&mut program.clone(), &mut vec![1], &mut outputs, 0, 0).execute_until_halt();

    answers.push(format!("{}", outputs[0]));

    outputs.clear();
    IntcodeMachine::new(&mut program.clone(), &mut vec![2], &mut outputs, 0, 0).execute_until_halt();

    answers.push(format!("{}", outputs[0]));

    answers
}
