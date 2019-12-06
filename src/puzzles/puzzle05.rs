use crate::intcode::IntcodeMachine;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let inputs: Vec<i32> = input.trim().split(',').map(|s| s.parse::<i32>().expect(&format!("Not a number found in input `{}`", s))).collect();
    let outputs: &mut Vec<i32> = &mut vec![];

    IntcodeMachine::solve(&mut inputs.clone(), &mut vec![1], outputs);

    answers.push(format!("{}", outputs.iter().last().unwrap()));

    outputs.clear();
    IntcodeMachine::solve(&mut inputs.clone(), &mut vec![5], outputs);

    answers.push(format!("{}", outputs.iter().last().unwrap()));

    answers
}
