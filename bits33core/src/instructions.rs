

pub enum Value {
    Register(u8),
    Immediate(u32),
}

pub enum OpType {
    Float,
    Int,
    UnsignedInt,
}

pub struct MathOp {
    lhs: Value,
    rhs: Value,
    dest: u8,
    op_type: OpType
}

pub struct Operation2 {
    lhs: Value,
    rhs: Value,
    dest: u8,
}

pub struct Operation1 {
    value: Value,
    dest: u8,
}

pub enum Instruction {
    Nop,
    Add { op: MathOp },
    Sub { op: MathOp },
    Mul { op: MathOp },
    Div { op: MathOp },
    Mod { op: MathOp },
    And { op: Operation2 },
    Or { op: Operation2 },
    Xor { op: Operation2 },
    Not { op: Operation1 },
    ShiftLeft { op: Operation2 },
    ShiftRight { op: Operation2 },
    Jump { dest: u32 },
    JumpIfZero { dest: u32, condition: Value },
    JumpIfNotZero { dest: u32, condition: Value },
    LoadMemory { dest: u8, value: Value },
    Store { dest: Value, value: Value },
    Move { dest: u8, value: Value },
}