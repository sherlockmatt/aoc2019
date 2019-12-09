use crate::intcode::IntcodeMachine;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let inputs: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    answers.push(format!("{}", calc(&mut inputs.clone(), 12, 2)));

    let mut try_noun: i64 = 0;
    let mut try_verb: i64 = 0;
    let mut last_result: i64 = calc(&mut inputs.clone(), try_noun, try_verb);
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

fn calc(state: &mut Vec<i64>, noun: i64, verb: i64) -> i64 {
    state[1] = noun;
    state[2] = verb;

    IntcodeMachine::new(state, &mut vec![], &mut vec![], 0).execute_until_halt();

    state[0]
}
