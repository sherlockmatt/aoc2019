use crate::intcode::IntcodeMachine;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let inputs: Vec<i32> = input.trim().split(',').map(|s| s.parse::<i32>().expect(&format!("Not a number found in input `{}`", s))).collect();

    answers.push(format!("{}", calc(&mut inputs.clone(), 12, 2)));

    let mut try_noun: i32 = 0;
    let mut try_verb: i32 = 0;
    let mut last_result: i32 = calc(&mut inputs.clone(), try_noun, try_verb);
    while last_result != 19690720 {
        if try_verb != 99 {
            try_verb += 1;
        } else {
            try_verb = 0;
            try_noun += 1;
        }
        last_result = calc(&mut inputs.clone(), try_noun, try_verb);
    }

    answers.push(format!("{:02}{:02}", try_noun, try_verb));

    answers
}

fn calc(state: &mut Vec<i32>, noun: i32, verb: i32) -> i32 {
    state[1] = noun;
    state[2] = verb;

    IntcodeMachine::solve(state, &mut vec![], &mut vec![]);

    state[0]
}
