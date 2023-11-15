use crate::virtual_machine::VirtualMachine;
use rmp_serde::{Deserializer};
use serde::Deserialize;
use bits33core::program::Program;

mod virtual_machine;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: bits33vm <program: .b33exe >");

        return;
    }

    let program = std::fs::read(args[1].clone()).unwrap();
    let program  = Program::deserialize(&mut Deserializer::new(&program[..])).unwrap();
    let mut vm: VirtualMachine = program.into();
    vm.run();

    println!("Registers: {:#?}", vm);
}
