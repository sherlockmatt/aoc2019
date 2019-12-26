use crate::intcode::IntcodeMachine;
use std::io;
use std::io::Write;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let mut answer = String::new();
    let mut answered = false;
    while !answered {
        print!("Do you want to play manually? [y/n]: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut answer) {
            Err(_) => { // Default to auto if we can't read from stdin
                answer = "n".to_string();
                answered = true;
            }
            _ => answered = answer.trim() == "y" || answer.trim() == "n"
        };
        answer = answer.trim().to_string();
    }
    let mut program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    let mut outputs = Vec::new();
    let mut pos = 0;
    let mut base = 0;
    let mut is_halted;

    {
        let mut inputs = Vec::new();
        let mut machine = IntcodeMachine::new(&mut program, &mut inputs, &mut outputs, pos, base);
        machine.execute_until_next_op_is_input();
        pos = machine.get_pos();
        base = machine.get_base();
        is_halted = machine.is_halted();
    }
    if answer == "y".to_string() {
        print_output(&mut outputs);
        while !is_halted {
            let mut command = String::new();
            match io::stdin().read_line(&mut command) {
                Err(error) => panic!("Error reading from stdin: {}", error),
                _ => ()
            };
            let mut inputs: Vec<i64> = command.chars().rev().map(|c| {
                let mut b = [0; 1];
                c.encode_utf8(&mut b);
                b[0] as i64
            }).collect();
            {
                let mut machine = IntcodeMachine::new(&mut program, &mut inputs, &mut outputs, pos, base);
                machine.execute_until_input_is_consumed();
                machine.execute_until_next_op_is_input();
                pos = machine.get_pos();
                base = machine.get_base();
                is_halted = machine.is_halted();
            }
            print_output(&mut outputs);
        }
    } else {
        let mut inputs: Vec<i64> = "east
north
east
north
north
west
take asterisk
east
south
east
take sand
south
west
take prime number
east
north
east
south
take tambourine
west
north
west
".chars().rev().map(|c| {
            let mut b = [0; 1];
            c.encode_utf8(&mut b);
            b[0] as i64
        }).collect();
        IntcodeMachine::new(&mut program, &mut inputs, &mut outputs, pos, base).execute_until_halt();
        // The answer happens to be the first number in the output, so it's easy to find
        answers.push(format!("{}", outputs.iter().map(|c| char::from(*c as u8)).join("").split_whitespace().find(|s| (*s).parse::<u32>().is_ok()).unwrap()));
    }

    answers
}

fn print_output(outputs: &mut Vec<i64>) {
    println!("{}", outputs.iter().map(|c| char::from(*c as u8)).join(""));
    outputs.clear();
}
