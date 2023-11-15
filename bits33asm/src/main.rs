extern crate pest;
#[macro_use]
extern crate pest_derive;


#[derive(Parser)]
#[grammar = "bits33asm.pest"]
struct BareParser;

fn main() {
    println!("Hello, world!");
}
