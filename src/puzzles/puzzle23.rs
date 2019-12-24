use crate::intcode::IntcodeMachine;
use std::collections::HashMap;
use arr_macro::arr;
use itertools::Itertools;

pub fn run(input: String) -> Vec<String> {
    let mut answers: Vec<String> = Vec::new();

    let program: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().expect(&format!("Not a number found in input `{}`", s))).collect();

    let mut packet_queue = HashMap::new();
    let mut states = arr![program.clone(); 50];
    let mut pos = [0; 50];
    let mut base = [0; 50];
    let mut is_halted = [false; 50];
    let mut is_idle = [false; 50];

    // Give them all their network addresses
    for current in 0..50 {
        let mut inputs = vec![current as i64];
        let mut outputs = Vec::new();
        let mut machine = IntcodeMachine::new(&mut states[current], &mut inputs, &mut outputs, pos[current], base[current]);
        machine.execute_until_next_op_is_input();
        pos[current] = machine.get_pos();
        base[current] = machine.get_base();
    }

    let mut current = 0;
    let mut sent_to_0 = Vec::new();
    while sent_to_0.len() < 2 || sent_to_0[sent_to_0.len() - 2] != sent_to_0[sent_to_0.len() - 1] {
        if !is_halted[current] {
            let mut inputs;
            let mut outputs = Vec::new();
            let two_inputs;
            let queue = packet_queue.entry(current as i64).or_insert(Vec::new());
            if queue.len() > 0 {
                let (packet_x, packet_y) = queue.remove(0);
                inputs = vec![packet_y, packet_x];
                two_inputs = true;
            } else {
                inputs = vec![-1];
                two_inputs = false;
            }
            {
                let mut machine = IntcodeMachine::new(&mut states[current], &mut inputs, &mut outputs, pos[current], base[current]);
                machine.execute_until_next_op_is_input();
                if two_inputs { machine.execute_until_next_op_is_input(); }
                pos[current] = machine.get_pos();
                base[current] = machine.get_base();
                is_halted[current] = machine.is_halted();
            }
            outputs.iter().tuples::<(_, _, _)>().for_each(|(d, x, y)| {
                packet_queue.entry(*d).or_insert(Vec::new()).push((*x, *y))
            });
            is_idle[current] = !two_inputs && outputs.len() == 0;
        }
        if current == 49 && is_idle.iter().all(|b| *b) {
            let packet_to_send = packet_queue.get(&255).unwrap().iter().last().unwrap().clone();
            packet_queue.entry(0).or_insert(Vec::new()).push(packet_to_send);
            sent_to_0.push(packet_to_send);
        }
        current += 1;
        current %= 50;
    }

    answers.push(format!("{}", packet_queue.get(&255).unwrap()[0].1));

    answers.push(format!("{}", packet_queue.get(&255).unwrap().iter().last().unwrap().1));

    answers
}
