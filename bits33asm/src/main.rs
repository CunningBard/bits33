extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;


#[derive(Parser)]
#[grammar = "bits33asm.pest"]
struct BareParser;

fn main() {
    let code = "r1 <- r2";
    let pairs = BareParser::parse(Rule::instruction, code).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", pairs);
}
