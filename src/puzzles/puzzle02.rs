use failure::_core::convert::TryInto;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let inputs: Vec<u32> = input.trim().split(',').map(|s| s.parse::<u32>().expect(&format!("Not a number found in input `{}`", s))).collect();

    answers.push(format!("{}", calc(&mut inputs.clone(), 12, 2)[0]));

    let mut try_noun: u32 = 0;
    let mut try_verb: u32 = 0;
    let mut last_result: u32 = calc(&mut inputs.clone(), try_noun, try_verb)[0];
    while last_result != 19690720 {
        if try_verb != 99 {
            try_verb += 1;
        } else {
            try_verb = 0;
            try_noun += 1;
        }
        last_result = calc(&mut inputs.clone(), try_noun, try_verb)[0];
    }

    answers.push(format!("{:02}{:02}", try_noun, try_verb));

    answers
}

fn calc(state: &mut Vec<u32>, noun: u32, verb: u32) -> &mut Vec<u32> {
    state[1] = noun;
    state[2] = verb;

    let mut pos: usize = 0;
    while state[pos] != 99 {
        let pos1: usize = state[pos+1usize].try_into().unwrap();
        let pos2: usize = state[pos+2usize].try_into().unwrap();
        let pos3: usize = state[pos+3usize].try_into().unwrap();
        if state[pos] == 1 {
            state[pos3] = state[pos1] + state[pos2];
        } else if state[pos] == 2 {
            state[pos3] = state[pos1] * state[pos2];
        } else {
            println!("Unknown opcode found `{}`", state[pos]);
        }
        pos += 4;
    }

    state
}
