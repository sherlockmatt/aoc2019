use crate::intcode::IntcodeMachine;
use std::thread;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i32> = input.trim().split(',').map(|s| s.parse::<i32>().expect(&format!("Not a number found in input `{}`", s))).collect();

    answers.push(format!("{}", (0..=4).permutations(5).map(
        |test_input| {
            // First input is 0, which is taken from outputs, so put that here
            let outputs: &mut Vec<i32> = &mut vec![0];

            for i in test_input {
                let inputs: &mut Vec<i32> = &mut outputs.clone();
                inputs.push(i);
                outputs.clear();
                IntcodeMachine::new(&mut program.clone(), inputs, outputs, 0).execute_until_halt();
            }

            outputs[0]
        }
    ).max().unwrap()));

    let mut threads = Vec::new();
    (5..=9).permutations(5).for_each(|test_input| {
        let program: Vec<i32> = input.trim().split(',').map(|s| s.parse::<i32>().expect(&format!("Not a number found in input `{}`", s))).collect();
        threads.push(thread::spawn(move || -> i32 {
            let inputs: [&mut Vec<i32>; 5] = [&mut vec![0, test_input[0]], &mut vec![test_input[1]], &mut vec![test_input[2]], &mut vec![test_input[3]], &mut vec![test_input[4]]];
            let outputs: [&mut Vec<i32>; 5] = [&mut vec![], &mut vec![], &mut vec![], &mut vec![], &mut vec![]];
            let states: [&mut Vec<i32>; 5] = [&mut program.clone(), &mut program.clone(), &mut program.clone(), &mut program.clone(), &mut program.clone()];
            let mut positions: [usize; 5] = [0, 0, 0, 0, 0];

            // Run all the machines once to consume their initial state
            for current in 0usize..=4usize {
                let mut machine = IntcodeMachine::new(states[current], inputs[current], outputs[current], positions[current]);
                machine.execute_until_next_op_is_input();
                positions[current] = machine.get_pos();
            }

            let mut current: usize = 0;
            loop {
                let mut machine = IntcodeMachine::new(states[current], inputs[current], outputs[current], positions[current]);
                machine.execute_until_next_op_is_input();

                // Save the position so that next loop we can start from where we left off
                positions[current] = machine.get_pos();

                // Stop processing once the last machine halts
                if current == 4 && machine.is_halted() { break; }

                // Feed the output to the next machine
                let next = (current + 1) % 5;
                inputs[next].push(outputs[current].pop().expect("No output found!"));

                current = next;
            }

            outputs[4][0]
        }));
    });

    let mut values = Vec::new();
    for t in threads {
        values.push(t.join().unwrap());
    }

    answers.push(format!("{}", values.iter().max().unwrap()));

    answers
}
