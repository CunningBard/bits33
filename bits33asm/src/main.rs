mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use rmp_serde::Serializer;
use serde::Serialize;
use bits33core::program::Program;

fn main()  {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: bits33asm <program: .b33asm>");

        return;
    }

    println!("Assembling {:?}", args[1]);

    let file = args[1].clone();

    if file.is_empty(){
        println!("No file given");

        return;
    }

    let file_name = file.split(".").collect::<Vec<&str>>()[0];

    let instructions = parser::parse_file(&*file).unwrap();

    for inst in &instructions {
        println!("{:?}", inst);
    }
    let program = Program {
        instructions,
        strings: Vec::new(),
    };

    program.serialize(
        &mut Serializer::new(
            std::fs::File::create(
                format!("{}.b33exe", file_name)
            ).unwrap()
        )
    ).unwrap();
}
