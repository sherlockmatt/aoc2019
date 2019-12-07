use crate::intcode::IntcodeMachine;
use std::thread;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i32> = input.trim().split(',').map(|s| s.parse::<i32>().expect(&format!("Not a number found in input `{}`", s))).collect();
    let mut max_value: i32 = 0;

    for a in 0..=4 {
        for b in 0..=4 {
            if b != a {
                for c in 0..=4 {
                    if c != a && c != b {
                        for d in 0..=4 {
                            if d != a && d != b && d != c {
                                for e in 0..=4 {
                                    if e != a && e != b && e != c && e != d {
                                        let test_input: Vec<i32> = vec![a, b, c, d, e];
                                        // First input is 0, which is taken from outputs, so put that here
                                        let outputs: &mut Vec<i32> = &mut vec![0];

                                        for i in test_input {
                                            let inputs: &mut Vec<i32> = &mut outputs.clone();
                                            inputs.push(i);
                                            outputs.clear();
                                            IntcodeMachine::new(&mut program.clone(), inputs, outputs, 0).execute_until_halt();
                                        }
                                        max_value = max_value.max(outputs[0]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    answers.push(format!("{}", max_value));

    let mut threads = Vec::new();
    for a in 5..=9 {
        for b in 5..=9 {
            if b != a {
                for c in 5..=9 {
                    if c != a && c != b {
                        for d in 5..=9 {
                            if d != a && d != b && d != c {
                                for e in 5..=9 {
                                    if e != a && e != b && e != c && e != d {
                                        let program: Vec<i32> = input.trim().split(',').map(|s| s.parse::<i32>().expect(&format!("Not a number found in input `{}`", s))).collect();
                                        threads.push(thread::spawn(move || -> i32 {
                                            let inputs: [&mut Vec<i32>; 5] = [&mut vec![0, a], &mut vec![b], &mut vec![c], &mut vec![d], &mut vec![e]];
                                            let outputs: [&mut Vec<i32>; 5] = [&mut vec![], &mut vec![], &mut vec![], &mut vec![], &mut vec![]];
                                            let states: [&mut Vec<i32>; 5] = [&mut program.clone(), &mut program.clone(), &mut program.clone(), &mut program.clone(), &mut program.clone()];
                                            let mut positions: [usize; 5] = [0, 0, 0, 0, 0];

                                            // Run all the machines once to consume their initial state
                                            for &current in &[0usize, 1usize, 2usize, 3usize, 4usize] {
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut values = Vec::new();
    for t in threads {
        values.push(t.join().unwrap());
    }

    answers.push(format!("{}", values.iter().max().unwrap()));

    answers
}
