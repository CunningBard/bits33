use bits33core::instructions::{Instruction, Value};
use crate::virtual_machine::VirtualMachine;

mod virtual_machine;

fn main() {
    let mut instructions = vec![];
    instructions.push(Instruction::Move {
        dest: 0,
        value: Value::Immediate(32)
    });
    let vm = VirtualMachine::run_instruction(instructions);
    println!("{:?}", vm);
}
