use std::collections::HashMap;
use failure::_core::convert::{TryInto, TryFrom};

pub struct Operation {
    name: String,
    length: usize,
    function: fn(&mut Vec<i64>, Vec<usize>, &mut Vec<i64>, &mut Vec<i64>, &mut i64) -> (bool, Option<usize>),
}

pub struct IntcodeMachine<'a> {
    memory: &'a mut Vec<i64>,
    opcodes: HashMap<i64, Operation>,
    inputs: &'a mut Vec<i64>,
    outputs: &'a mut Vec<i64>,
    pos: usize,
    is_halted: bool,
    base: i64,
}

impl IntcodeMachine<'_> {
    pub fn new<'a>(memory: &'a mut Vec<i64>, inputs: &'a mut Vec<i64>, outputs: &'a mut Vec<i64>, pos: usize, base: i64) -> IntcodeMachine<'a> {
        let mut opcodes: HashMap<i64, Operation> = HashMap::new();
        opcodes.insert(
            1,
            Operation {
                name: String::from("ADD"),
                length: 4,
                function: |state, args, _, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = state[args[0]] + state[args[1]];
                    (false, None)
                },
            });
        opcodes.insert(
            2,
            Operation {
                name: String::from("MULTIPLY"),
                length: 4,
                function: |state, args, _, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = state[args[0]] * state[args[1]];
                    (false, None)
                },
            });
        opcodes.insert(
            3,
            Operation {
                name: String::from("INPUT"),
                length: 2,
                function: |state, args, inputs, _, _| {
                    state[usize::try_from(args[0]).unwrap()] = inputs.pop().unwrap_or_else(|| panic!("Ran out of inputs."));
                    (false, None)
                },
            },
        );
        opcodes.insert(
            4,
            Operation {
                name: String::from("OUTPUT"),
                length: 2,
                function: |state, args, _, outputs, _| {
                    outputs.push(state[usize::try_from(args[0]).unwrap()]);
                    (false, None)
                },
            },
        );
        opcodes.insert(
            5,
            Operation {
                name: String::from("JUMP IF TRUE"),
                length: 3,
                function: |state, args, _, _, _| {
                    match state[args[0]] > 0 {
                        true => (false, Some(state[args[1]].try_into().unwrap())),
                        false => (false, None)
                    }
                },
            },
        );
        opcodes.insert(
            6,
            Operation {
                name: String::from("JUMP IF FALSE"),
                length: 3,
                function: |state, args, _, _, _| {
                    match state[args[0]] == 0 {
                        true => (false, Some(state[args[1]].try_into().unwrap())),
                        false => (false, None)
                    }
                },
            },
        );
        opcodes.insert(
            7,
            Operation {
                name: String::from("LESS THAN"),
                length: 4,
                function: |state, args, _, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = if state[args[0]] < state[args[1]] { 1 } else { 0 };
                    (false, None)
                },
            },
        );
        opcodes.insert(
            8,
            Operation {
                name: String::from("EQUAL TO"),
                length: 4,
                function: |state, args, _, _, _| {
                    state[usize::try_from(args[2]).unwrap()] = if state[args[0]] == state[args[1]] { 1 } else { 0 };
                    (false, None)
                },
            },
        );
        opcodes.insert(
            9,
            Operation {
                name: String::from("SET BASE"),
                length: 2,
                function: |state, args, _, _, base| {
                    *base += state[args[0]];
                    (false, None)
                },
            },
        );
        opcodes.insert(
            99,
            Operation {
                name: String::from("HALT"),
                length: 1,
                function: |_, _, _, _, _| (true, None),
            },
        );

        IntcodeMachine {
            memory,
            opcodes,
            inputs,
            outputs,
            pos,
            is_halted: false,
            base,
        }
    }

    pub fn execute(&mut self, until_fn: fn(&IntcodeMachine) -> bool) {
        let mut running: bool = true;

        while running {
            // Opcode is the last two digits of the value, the rest are the argument modes
            let opcode: i64 = self.memory[self.pos] % 100;
            let mut arg_modes: i64 = self.memory[self.pos] / 100;

            let operation: &Operation = self.opcodes.get(&opcode).unwrap_or_else(|| panic!("Unknown opcode found `{}` at pos `{}` of memory `{:?}`", opcode, self.pos, self.memory));
            let mut args: Vec<usize> = vec![];

            for n in 1..operation.length {
                let arg_mode = arg_modes % 10;

                args.push(match arg_mode {
                    0 => self.memory[self.pos + n].try_into().unwrap(),
                    1 => self.pos + n,
                    2 => usize::try_from(self.base + self.memory[self.pos + n]).unwrap_or_else(|_| panic!("Relative base caused an out of bounds to address `{}`", self.base + self.memory[self.pos + n])),
                    other => panic!("Unknown argument mode `{}`", other)
                }
                );

                if args[n - 1] >= self.memory.len().try_into().unwrap() {
                    self.memory.resize(usize::try_from(args[n - 1]).unwrap() + 100, 0)
                }

                // Shift right
                arg_modes /= 10;
            }

            let (halt, jump) = (operation.function)(self.memory, args, self.inputs, self.outputs, &mut self.base);
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

    pub fn get_base(&self) -> i64 {
        self.base
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
