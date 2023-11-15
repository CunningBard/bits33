use pest::iterators::Pair;
use pest::Parser;
use bits33core::instructions::{Instruction, Op2, Op2wTypes, OpType, Value};

#[derive(Parser)]
#[grammar = "bits33asm.pest"]
struct BareParser;


fn pair_to_op_type(op_type: Pair<Rule>) -> Result<OpType, String> {
    match op_type.as_str() {
        "f32" => Ok(OpType::Float),
        "i32" => Ok(OpType::Int),
        "u32" => Ok(OpType::UnsignedInt),
        _ => Err(format!("Invalid op_type: {}", op_type)),
    }
}

fn pair_to_register(register: Pair<Rule>) -> Result<u8, String> {
    match register.as_rule() {
        Rule::reg => Ok(register.as_str()[1..].parse::<u8>().unwrap()),
        _ => unreachable!("Invalid Pair Given")
    }
}

fn pair_to_value(value: Pair<Rule>) -> Result<Value, String> {
    match value.as_rule() {
        Rule::float => Ok(Value::Immediate(value.as_str().parse::<f32>().unwrap().to_bits())),
        Rule::int => Ok(Value::Immediate(value.as_str().parse::<i32>().unwrap() as u32)),
        Rule::uint => Ok(Value::Immediate(value.as_str().parse::<u32>().unwrap())),
        Rule::reg => Ok(Value::Register(value.as_str()[1..].parse::<u8>().unwrap())),
        _ => Err(format!("Invalid value: {}", value)),
    }
}

pub fn parse_file(file: &str) -> Result<Vec<Instruction>, String> {
    let code = std::fs::read_to_string(file).expect("Unable to read file");
    parse(&code)
}

fn parse(code: &String) -> Result<Vec<Instruction>, String> {
    let pairs = BareParser::parse(Rule::program, code)
        .map_err(|e| format!("{}", e))?;

    let mut instructions = Vec::new();

    macro_rules! jmp_types {
        ($jmp_type:ident, $dest:expr, $value:expr) => {
            match $dest.as_rule() {
                Rule::identifier => {
                        unimplemented!()
                }
                Rule::int => {
                    Instruction::$jmp_type { dest: $dest.as_str().parse::<u32>().unwrap(), condition: $value }
                }
                _ => unreachable!()
            }
        };
        ($jmp_type:ident, $dest:expr) => {
            match $dest.as_rule() {
                Rule::identifier => {
                        unimplemented!()
                }
                Rule::int => {
                    Instruction::$jmp_type { dest: $dest.as_str().parse::<u32>().unwrap() }
                }
                _ => unreachable!()
            }
        }
    }


    for pair in pairs {
        let pair_rule = pair.as_rule();
        let mut inner_rules = pair.into_inner();
        let res = match pair_rule {
            Rule::nop => Instruction::Nop,
            Rule::add |
            Rule::sub |
            Rule::mul |
            Rule::div |
            Rule::mod_ |
            Rule::gte |
            Rule::lte |
            Rule::gt_ |
            Rule::lt_
            => {
                let dest = pair_to_register(inner_rules.next().unwrap())?;
                let op_type = pair_to_op_type(inner_rules.next().unwrap())?;
                let value1 = pair_to_value(inner_rules.next().unwrap())?;
                let value2 = pair_to_value(inner_rules.next().unwrap())?;
                let op2 = Op2wTypes {
                    lhs: value1,
                    rhs: value2,
                    dest,
                    op_type,
                };

                match pair_rule {
                    Rule::add => Instruction::Add { op: op2 },
                    Rule::sub => Instruction::Sub { op: op2 },
                    Rule::mul => Instruction::Mul { op: op2 },
                    Rule::div => Instruction::Div { op: op2 },
                    Rule::mod_=> Instruction::Mod { op: op2 },
                    Rule::gte => Instruction::GreaterThanOrEqual { op: op2 },
                    Rule::lte => Instruction::LessThanOrEqual { op: op2 },
                    Rule::gt_ => Instruction::GreaterThan { op: op2 },
                    Rule::lt_ => Instruction::LessThan { op: op2 },
                    _ => unreachable!(),
                }
            },
            Rule::and |
            Rule::or  |
            Rule::xor |
            Rule::shl |
            Rule::shr |
            Rule::eq_ |
            Rule::neq
            => {
                let dest = pair_to_register(inner_rules.next().unwrap())?;
                let value1 = pair_to_value(inner_rules.next().unwrap())?;
                let value2 = pair_to_value(inner_rules.next().unwrap())?;
                let op2 = Op2 {
                    lhs: value1,
                    rhs: value2,
                    dest
                };

                match pair_rule {
                    Rule::and => Instruction::And { op: op2 },
                    Rule::or  => Instruction::Or { op: op2 },
                    Rule::xor => Instruction::Xor { op: op2 },
                    Rule::shl => Instruction::ShiftLeft { op: op2 },
                    Rule::shr => Instruction::ShiftRight { op: op2 },
                    Rule::eq_ => Instruction::Equal { op: op2 },
                    Rule::neq => Instruction::NotEqual { op: op2 },
                    _ => unreachable!(),
                }
            }

            Rule::jmp |
            Rule::jiz |
            Rule::jnz
            => {
                let dst = inner_rules.next().unwrap();

                match pair_rule {
                    Rule::jmp => {
                        jmp_types!(Jump, dst)
                    }
                    Rule::jiz => {
                        let value = pair_to_value(inner_rules.next().unwrap())?;
                        jmp_types!(JumpIfZero, dst, value)
                    }
                    Rule::jnz => {
                        let value = pair_to_value(inner_rules.next().unwrap())?;
                        jmp_types!(JumpIfNotZero, dst, value)
                    }
                    _ => unreachable!()
                }
            }
            Rule::load => {
                let dest = pair_to_register(inner_rules.next().unwrap())?;
                let value = pair_to_value(inner_rules.next().unwrap())?;

                Instruction::LoadMemory { dest, value }
            }
            Rule::store => {
                let dest = pair_to_value(inner_rules.next().unwrap())?;
                let value = pair_to_value(inner_rules.next().unwrap())?;

                Instruction::Store { dest, value }
            }
            Rule::mov => {
                let dest = pair_to_register(inner_rules.next().unwrap())?;
                let value = pair_to_value(inner_rules.next().unwrap())?;

                Instruction::Move {
                    dest,
                    value
                }

            }
            Rule::EOI => break,
            unknown => unreachable!("{:?}", unknown)
        };

        instructions.push(res);
    }

    Ok(instructions)
}