use std::collections::HashMap;
use failure::_core::convert::{TryInto, TryFrom};

pub struct Operation {
    name: String,
    length: usize,
    arg_mode_overrides: Vec<Option<i32>>,
    function: fn(&mut Vec<i32>, Vec<i32>, &mut Vec<i32>, &mut Vec<i32>) -> (bool, Option<usize>),
}

pub struct IntcodeMachine<'a> {
    memory: &'a mut Vec<i32>,
    opcodes: HashMap<i32, Operation>,
    inputs: &'a mut Vec<i32>,
    outputs: &'a mut Vec<i32>,
    pos: usize,
    is_halted: bool
}

impl IntcodeMachine<'_> {
    pub fn new<'a>(memory: &'a mut Vec<i32>, inputs: &'a mut Vec<i32>, outputs: &'a mut Vec<i32>, pos: usize) -> IntcodeMachine<'a> {
        let mut opcodes: HashMap<i32, Operation> = HashMap::new();
        opcodes.insert(
            1,
            Operation {
                name: String::from("ADD"),
                length: 4,
                arg_mode_overrides: vec![None, None, Some(1)],
                function: |state, args, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = args[0] + args[1];
                    (false, None)
                },
            });
        opcodes.insert(
            2,
            Operation {
                name: String::from("SUBTRACT"),
                length: 4,
                arg_mode_overrides: vec![None, None, Some(1)],
                function: |state, args, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = args[0] * args[1];
                    (false, None)
                },
            });
        opcodes.insert(
            3,
            Operation {
                name: String::from("INPUT"),
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
            Operation {
                name: String::from("OUTPUT"),
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
            Operation {
                name: String::from("JUMP IF TRUE"),
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
            Operation {
                name: String::from("JUMP IF FALSE"),
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
            Operation {
                name: String::from("LESS THAN"),
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
            Operation {
                name: String::from("EQUAL TO"),
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
            Operation {
                name: String::from("HALT"),
                length: 1,
                arg_mode_overrides: vec![],
                function: |_, _, _, _| (true, None),
            });

        IntcodeMachine {
            memory,
            opcodes,
            inputs,
            outputs,
            pos,
            is_halted: false
        }
    }

    pub fn execute(&mut self, until_fn: fn(&IntcodeMachine) -> bool) {
        let mut running: bool = true;

        while running {
            // Opcode is the last two digits of the value, the rest are the argument modes
            let opcode: i32 = self.memory[self.pos] % 100;
            let mut arg_modes: i32 = self.memory[self.pos] / 100;

            let operation: &Operation = self.opcodes.get(&opcode).unwrap_or_else(|| panic!("Unknown opcode found `{}` at pos `{}` of memory `{:?}`", opcode, self.pos, self.memory));
            let mut args: Vec<i32> = vec![];

            for n in 1..operation.length {
                // Use the override mode if present, or get the last digit of arg_modes
                let arg_mode = operation.arg_mode_overrides[n - 1].unwrap_or(arg_modes % 10);
                let pos_to_get: usize;

                if arg_mode == 1 {
                    pos_to_get = self.pos + n;
                } else {
                    pos_to_get = self.memory[self.pos + n].try_into().unwrap();
                }

                args.push(self.memory[pos_to_get]);

                // Shift right
                arg_modes /= 10;
            }

            let (halt, jump) = (operation.function)(self.memory, args, self.inputs, self.outputs);
            self.is_halted = halt;

            self.pos = match jump {
                None => self.pos + operation.length,
                Some(location) => location
            };

            running = !self.is_halted && !(until_fn)(self);
        }
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn execute_until_halt(&mut self) {
        self.execute(|_| false);
    }

    pub fn execute_until_next_op_is_input(&mut self) {
        self.execute(|m|
            match m.opcodes.get(&(m.memory[m.pos] % 100)) {
                Some(o) if o.name == String::from("INPUT") => true,
                _ => false
            }
        );
    }
}
