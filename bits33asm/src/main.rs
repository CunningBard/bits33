mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use rmp_serde::Serializer;
use serde::Serialize;
use bits33core::program::Program;

fn main() {
    let file = "./bits33asm/test.bits33asm";

    let instructions = parser::parse_file(file).unwrap();

    for inst in &instructions {
        println!("{:?}", inst);
    }
    let program = Program {
        instructions,
        strings: Vec::new(),
    };

    program.serialize(&mut Serializer::new(std::fs::File::create("./test.bits33exe").unwrap())).unwrap();
}
