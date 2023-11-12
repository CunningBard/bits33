use bits33core::instructions::{Instruction, MathOp, Operation1, Operation2, OpType, Value};


#[derive(Debug)]
pub struct VirtualMachine {
    registers: [u32; 8],
    memory: [u8; 32],
    instruction_pointer: u32,
    instructions: Vec<Instruction>,
}

impl VirtualMachine {
    pub fn new(mut instructions: Vec<Instruction>) -> VirtualMachine {
        // add nop instruction to start of instructions

        let mut new_instructions = vec![Instruction::Nop];
        new_instructions.append(&mut instructions);

        VirtualMachine {
            registers: Default::default(),
            memory: Default::default(),
            instruction_pointer: 0,
            instructions: new_instructions
        }
    }

    fn evaluate_value(&self, value: Value) -> u32 {
        match value {
            Value::Register(r) => self.registers[r as usize],
            Value::Immediate(i) => i,
        }
    }
    
    fn handle_instruction(&mut self, instruction: Instruction){
        macro_rules! math_op {
            ($lhs:expr, $rhs:expr, $op_type:expr, $op:tt) => {
                match $op_type {
                    OpType::Float => (f32::from_bits($lhs) $op f32::from_bits($rhs)).to_bits(),
                    OpType::Int => ($lhs as i32 $op $rhs as i32) as u32,
                    OpType::UnsignedInt => $lhs $op $rhs,
                }
            }
        }

        match instruction {
            Instruction::Nop => {}
            Instruction::Add { op } |
            Instruction::Sub { op } |
            Instruction::Mul { op } |
            Instruction::Div { op } |
            Instruction::Mod { op }
            => {
                let lhs = self.evaluate_value(op.lhs);
                let rhs = self.evaluate_value(op.rhs);
                let res;

                match instruction {
                    Instruction::Add { op } => {
                        res = math_op!(lhs, rhs, op.op_type, +);
                    }
                    Instruction::Sub { op } => {
                        res = math_op!(lhs, rhs, op.op_type, -);
                    }
                    Instruction::Mul { op } => {
                        res = math_op!(lhs, rhs, op.op_type, *);
                    }
                    Instruction::Div { op } => {
                        res = math_op!(lhs, rhs, op.op_type, /);
                    }
                    Instruction::Mod { op } => {
                        res = math_op!(lhs, rhs, op.op_type, %);
                    }
                    _ => {
                        unreachable!()
                    }
                }

                self.registers[op.dest as usize] = res;
            }
            Instruction::And { op } |
            Instruction::Or { op } |
            Instruction::Xor { op } |
            Instruction::ShiftLeft { op } |
            Instruction::ShiftRight { op }
            => {
                let lhs = self.evaluate_value(op.lhs);
                let rhs = self.evaluate_value(op.rhs);

                let res = match instruction {
                    Instruction::And { .. } => {
                        lhs & rhs
                    }
                    Instruction::Or { .. } => {
                        lhs | rhs
                    }
                    Instruction::Xor { .. } => {
                        lhs ^ rhs
                    }
                    Instruction::ShiftLeft { .. } => {
                        lhs << rhs
                    }
                    Instruction::ShiftRight { .. } => {
                        lhs >> rhs
                    }
                    _ => {
                        unreachable!()
                    }
                };

                self.registers[op.dest as usize] = res;
            }
            Instruction::Not { op } => {
                let value = self.evaluate_value(op.value);

                self.registers[op.dest as usize] = !value;
            }
            Instruction::Jump {
                dest
            } => {
                self.instruction_pointer = dest;
            }
            Instruction::JumpIfZero {
                dest,
                condition
            } => {
                let condition = self.evaluate_value(condition);

                if condition == 0 {
                    self.instruction_pointer = dest;
                }
            }
            Instruction::JumpIfNotZero {
                dest,
                condition
            } => {
                let condition = self.evaluate_value(condition);

                if condition != 0 {
                    self.instruction_pointer = dest;
                }
            }
            Instruction::LoadMemory {
                dest,
                value
            } => {
                let value = self.evaluate_value(value);

                self.registers[dest as usize] = u32::from_le_bytes([
                    self.memory[value as usize],
                    self.memory[value as usize + 1],
                    self.memory[value as usize + 2],
                    self.memory[value as usize + 3],
                ]);
            }
            Instruction::Store {
                dest,
                value
            } => {
                let dest = self.evaluate_value(dest);
                let value = self.evaluate_value(value);

                let bytes = value.to_le_bytes();

                self.memory[dest as usize] = bytes[0];
                self.memory[dest as usize + 1] = bytes[1];
                self.memory[dest as usize + 2] = bytes[2];
                self.memory[dest as usize + 3] = bytes[3];
            }
            Instruction::Move {
                dest,
                value
            } => {
                let value = self.evaluate_value(value);

                self.registers[dest as usize] = value;
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.instructions[self.instruction_pointer as usize];

            self.handle_instruction(instruction);

            self.instruction_pointer += 1;

            if self.instruction_pointer >= self.instructions.len() as u32 {
                break;
            }
        }
    }

    pub fn run_instruction(instruction: Vec<Instruction>) -> Self {
        let mut vm = VirtualMachine::new(instruction);
        vm.run();
        vm
    }
}