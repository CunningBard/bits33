use bits33core::instructions::{Instruction, MathOp, Operation1, Operation2, OpType, Value};
struct VirtualMachine {
    registers: [u32; 32],
    memory: [u8; 1024],
    instruction_pointer: u32,
    instructions: Vec<Instruction>,
}

impl VirtualMachine {
    pub fn new(mut instructions: Vec<Instruction>) -> VirtualMachine {
        // add nop instruction to start of instructions

        let mut new_instructions = vec![Instruction::Nop];
        new_instructions.append(&mut instructions);

        VirtualMachine {
            registers: [0; 32],
            memory: [0; 1024],
            instruction_pointer: 0,
            instructions: new_instructions
        }
    }

    pub fn evaluate_value(&self, value: Value) -> u32 {
        match value {
            Value::Register(r) => self.registers[r as usize],
            Value::Immediate(i) => i,
        }
    }
    
    pub fn handle_instruction(&mut self, instruction: Instruction){
        macro_rules! math_op {
            ($lhs:expr, $rhs:expr, $op_type:expr, $op:tt, $dest:expr) => {
                let lhs = self.evaluate_value($lhs);
                let rhs = self.evaluate_value($rhs);

                let result = match $op_type {
                    OpType::Float => (f32::from_bits(lhs) $op f32::from_bits(rhs)).to_bits(),
                    OpType::Int => (i32::from_bits(lhs) $op i32::from_bits(rhs)).to_bits(),
                    OpType::UnsignedInt => lhs $op rhs,
                };

                self.registers[$dest as usize] = result;
            }
        }


        macro_rules! op2 {
            ($lhs:expr, $rhs:expr, $op:tt, $dest:expr) => {
                let lhs = self.evaluate_value($lhs);
                let rhs = self.evaluate_value($rhs);

                self.registers[$dest as usize] = lhs $op rhs;
            }
        }

        match instruction {
            Instruction::Nop => {}
            Instruction::Add {
                op: MathOp {
                    lhs,
                    rhs,
                    dest,
                    op_type
                }
            } => {
                math_op!(lhs, rhs, op_type, +, dest)
            }
            Instruction::Sub {
                op: MathOp {
                    lhs,
                    rhs,
                    dest,
                    op_type
                }
            } => {
                math_op!(lhs, rhs, op_type, -, dest)
            }
            Instruction::Mul {
                op: MathOp {
                    lhs,
                    rhs,
                    dest,
                    op_type
                }
            } => {
                math_op!(lhs, rhs, op_type, *, dest)
            }
            Instruction::Div {
                op: MathOp {
                    lhs,
                    rhs,
                    dest,
                    op_type
                }
            } => {
                math_op!(lhs, rhs, op_type, /, dest)
            }
            Instruction::Mod {
                op: MathOp {
                    lhs,
                    rhs,
                    dest,
                    op_type
                }
            } => {
                math_op!(lhs, rhs, op_type, %, dest)
            }
            Instruction::And {
                op: Operation2 {
                    lhs,
                    rhs,
                    dest,
                }
            } => {
                op2!(lhs, rhs, &, dest)
            }
            Instruction::Or {
                op: Operation2 {
                    lhs,
                    rhs,
                    dest,
                }
            } => {
                op2!(lhs, rhs, |, dest)
            }
            Instruction::Xor {
                op: Operation2 {
                    lhs,
                    rhs,
                    dest,
                }
            } => {
                op2!(lhs, rhs, ^, dest)
            }
            Instruction::Not {
                op: Operation1 {
                    value,
                    dest,
                }
            } => {
                let value = self.evaluate_value(value);

                self.registers[dest as usize] = !value;
            }
            Instruction::ShiftLeft {
                op: Operation2 {
                    lhs,
                    rhs,
                    dest,
                }
            } => {
                op2!(lhs, rhs, <<, dest)
            }
            Instruction::ShiftRight {
                op: Operation2 {
                    lhs,
                    rhs,
                    dest,
                }
            } => {
                op2!(lhs, rhs, >>, dest)
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
}