use crate::intcode::IntcodeMachine;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    for springscript in &["NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
", "NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN
"] {
        let mut inputs = springscript.chars().rev().map(|c| {
            let mut b = [0; 1];
            c.encode_utf8(&mut b);
            b[0] as i64
        }).collect();
        let mut outputs = Vec::new();

        IntcodeMachine::new(&mut program.clone(), &mut inputs, &mut outputs, 0, 0).execute_until_halt();

        answers.push(format!("{}", outputs[outputs.len() - 1]));
    }

    answers
}
