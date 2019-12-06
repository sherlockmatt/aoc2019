use std::collections::HashMap;
use failure::_core::convert::{TryInto, TryFrom};

pub struct Operation {
    length: usize,
    arg_mode_overrides: Vec<Option<i32>>,
    function: fn(&mut Vec<i32>, Vec<i32>, &mut Vec<i32>, &mut Vec<i32>) -> (bool, Option<usize>),
}

pub struct IntcodeMachine<'a> {
    memory: &'a mut Vec<i32>,
    opcodes: HashMap<i32, Operation>,
    inputs: &'a mut Vec<i32>,
    outputs: &'a mut Vec<i32>
}

impl IntcodeMachine<'_> {
    pub fn new<'a>(memory: &'a mut Vec<i32>, inputs: &'a mut Vec<i32>, outputs: &'a mut Vec<i32>) -> IntcodeMachine<'a> {
        let mut opcodes: HashMap<i32, Operation> = HashMap::new();
        opcodes.insert(
            1,
            Operation { // ADD
                length: 4,
                arg_mode_overrides: vec![None, None, Some(1)],
                function: |state, args, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = args[0] + args[1];
                    (false, None)
                },
            });
        opcodes.insert(
            2,
            Operation { // SUBTRACT
                length: 4,
                arg_mode_overrides: vec![None, None, Some(1)],
                function: |state, args, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = args[0] * args[1];
                    (false, None)
                },
            });
        opcodes.insert(
            3,
            Operation { // INPUT
                length: 2,
                arg_mode_overrides: vec![Some(1)],
                function: |state, args, inputs, _| {
                    state[usize::try_from(args[0]).unwrap()] = inputs.pop().unwrap_or_else(|| panic!("Ran out of inputs."));
                    (false, None)
                }
            }
        );
        opcodes.insert(
            4,
            Operation { // OUTPUT
                length: 2,
                arg_mode_overrides: vec![Some(1)],
                function: |state, args, _, outputs| {
                    outputs.push(state[usize::try_from(args[0]).unwrap()]);
                    (false, None)
                }
            }
        );
        opcodes.insert(
            5,
            Operation { // JUMP IF TRUE
                length: 3,
                arg_mode_overrides: vec![None, None],
                function: |_, args, _, _| {
                    match args[0] > 0 {
                        true => (false, Some(args[1].try_into().unwrap())),
                        false => (false, None)
                    }
                }
            }
        );
        opcodes.insert(
            6,
            Operation { // JUMP IF FALSE
                length: 3,
                arg_mode_overrides: vec![None, None],
                function: |_, args, _, _| {
                    match args[0] == 0 {
                        true => (false, Some(args[1].try_into().unwrap())),
                        false => (false, None)
                    }
                }
            }
        );
        opcodes.insert(
            7,
            Operation { // LESS THAN
                length: 4,
                arg_mode_overrides: vec![None, None, Some(1)],
                function: |state, args, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = if args[0] < args[1] { 1 } else { 0 };
                    (false, None)
                }
            }
        );
        opcodes.insert(
            8,
            Operation { // EQUAL TO
                length: 4,
                arg_mode_overrides: vec![None, None, Some(1)],
                function: |state, args, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = if args[0] == args[1] { 1 } else { 0 };
                    (false, None)
                }
            }
        );
        opcodes.insert(
            99,
            Operation { // HALT
                length: 1,
                arg_mode_overrides: vec![],
                function: |_, _, _, _| (true, None),
            });

        IntcodeMachine {
            memory,
            opcodes,
            inputs,
            outputs
        }
    }

    pub fn execute(&mut self) {
        let mut pos: usize = 0;
        let mut running: bool = true;

        while running {
            // Opcode is the last two digits of the value, the rest are the argument modes
            let opcode: i32 = self.memory[pos] % 100;
            let mut arg_modes: i32 = self.memory[pos] / 100;

            let operation: &Operation = self.opcodes.get(&opcode).unwrap_or_else(|| panic!("Unknown opcode found `{}`", opcode));
            let mut args: Vec<i32> = vec![];

            for n in 1..operation.length {
                // Use the override mode if present, or get the last digit of arg_modes
                let arg_mode = operation.arg_mode_overrides[n - 1].unwrap_or(arg_modes % 10);
                let pos_to_get: usize;

                if arg_mode == 1 {
                    pos_to_get = pos + n;
                } else {
                    pos_to_get = self.memory[pos + n].try_into().unwrap();
                }

                args.push(self.memory[pos_to_get]);

                // Shift right
                arg_modes /= 10;
            }

            let (halt, jump) = (operation.function)(self.memory, args, self.inputs, self.outputs);
            running = !halt;

            pos = match jump {
                None => pos + operation.length,
                Some(location) => location
            };
        }
    }

    pub fn solve<'a>(memory: &'a mut Vec<i32>, inputs: &'a mut Vec<i32>, outputs: &'a mut Vec<i32>) {
        IntcodeMachine::new(memory, inputs, outputs).execute();
    }
}
