use crate::constants;


const GAMEBOY_INSTRUCTIONS: &str = constants::DAY_EIGHT;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    acc(i32),
    jmp(i32),
    nop(i32)
}

#[derive(Debug, Clone)]
struct CompiledInstruction {
    instruction: Instruction,
    accumulator_value: i32,
    index: usize
}

struct GameBoy {
    pub instructions: Vec<(Instruction, bool)>,
}

impl GameBoy {
    fn new(input: &str) -> Self {
        let instructions = input.lines().map(|x| {
            let inst = &x[0..3];
            let value: i32 = x[4..].to_string().parse().unwrap();
            match inst {
                "acc" => (Instruction::acc(value), false),
                "jmp" => (Instruction::jmp(value), false),
                "nop" => (Instruction::nop(value), false),
                _ => panic!("Invalid Instruction")
            }
        })
        // .inspect(|x| {println!("{:?}", x);})
        .collect();

        Self {
            instructions
        }
    }

    fn compile(&mut self, compiled_instruction: Option<&CompiledInstruction>) -> Vec<CompiledInstruction> {
        let mut i: usize = match compiled_instruction {
            Some(n) => n.index,
            None => 0
        };
        let mut acc: i32 = match compiled_instruction {
            Some(n) => n.accumulator_value,
            None => 0
        };
        let mut compiled_instructions = vec![];
        loop {
            // if has not been checked
            if !self.instructions[i].1 {
                self.instructions[i].1 = true;
                match self.instructions[i].0 {
                    Instruction::acc(val) => {
                        acc = acc + val;
                        compiled_instructions.push(CompiledInstruction {
                            instruction: Instruction::acc(val),
                            accumulator_value: acc,
                            index: i
                        });
                        i = i + 1;
                    },
                    Instruction::jmp(val) => {
                        compiled_instructions.push(CompiledInstruction {
                            instruction: Instruction::jmp(val),
                            accumulator_value: acc,
                            index: i
                        });

                        let index = i as i32 + val;
                        // println!("{:?}", index);
                        // println!("{:?}", val);
                        // println!("{:?}", self.instructions.len());
                        if index >= self.instructions.len() as i32 || index < 0 {
                            break;
                        } else {
                            i = index as usize;
                        }
                    },
                    Instruction::nop(val) => {
                        compiled_instructions.push(CompiledInstruction {
                            instruction: Instruction::nop(val),
                            accumulator_value: acc,
                            index: i
                        });
                        i = i + 1;
                    },
                }
            } else {
                // if it has been checked, then we exit the loop and return the compiled instructions
                break;
            }
        };

        compiled_instructions

    }

    fn run_instructions(mut self) -> i32 {
        let compiled_instructions = self.compile(None);
        compiled_instructions[compiled_instructions.iter().count() - 1].accumulator_value
    }

    fn update_instructions(&mut self) -> i32 {
        let compiled_instructions = self.compile(None);
        // 1 - go backwards so we only affect recent changes
        for i in (0..compiled_instructions.len()).rev() {
            let compiled_instruction = compiled_instructions.get(i).unwrap();
            match compiled_instruction.instruction {
                Instruction::jmp(val) | Instruction::nop(val) => {
                    // 1 - reset the instructions so that all checked bools are false
                    self.reset_instructions();
                    // 2 - replace a nop with a jmp and vice versa
                    match compiled_instruction.instruction {
                        Instruction::jmp(v) => {
                            self.instructions[compiled_instruction.index] = (Instruction::nop(val), false);
                        },
                        Instruction::nop(v) => {
                            // check that there wont be an obvious compilation error before compiling
                            let would_be_in_bounds: bool = {
                                let index: i32;
                                index = i as i32 + val;
                                (self.instructions.len() - 1) as i32 > index && index > 0
                            };
                            if would_be_in_bounds || val != 0 {
                                self.instructions[compiled_instruction.index] = (Instruction::jmp(val), false);
                            }
                        },
                        _ => {}
                    }
                    // 3 - try the instructions
                    let recompiled_instructions = self.compile(Some(compiled_instruction));
                    // 4 - if compiled_instructions[end].index == instructions.len() - 1, 
                    if recompiled_instructions[recompiled_instructions.iter().count() - 1].index + 1 == self.instructions.len() {
                        // 4.1 - we did it, return the accumulator there
                        return recompiled_instructions[recompiled_instructions.iter().count() - 1].accumulator_value;
                    } else {
                        // 4.2 - if we didnt make it, the revert the change and check the next candidate
                        match compiled_instruction.instruction {
                            Instruction::jmp(val) => {
                                self.instructions[compiled_instruction.index] = (Instruction::jmp(val), false);
                            },
                            Instruction::nop(val) => {
                                self.instructions[compiled_instruction.index] = (Instruction::nop(val), false);
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
        0
    }

    fn reset_instructions(&mut self) {
        for i in 0..self.instructions.len() {
            self.instructions[i] = (self.instructions[i].0, false)
        }
    }
}

pub fn find_accumulator_value_at_infinite_loop() -> i32 {
    let gameboy = GameBoy::new(GAMEBOY_INSTRUCTIONS);
    gameboy.run_instructions()
}

pub fn find_acc_after_fix() -> i32 {
    let mut gameboy = GameBoy::new(GAMEBOY_INSTRUCTIONS);
    gameboy.update_instructions()
}
