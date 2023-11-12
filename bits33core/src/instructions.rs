
#[derive(Debug, Clone, Copy)]
pub enum Value {
    Register(u8),
    Immediate(u32),
}

#[derive(Debug, Clone, Copy)]
pub enum OpType {
    Float,
    Int,
    UnsignedInt,
}

#[derive(Debug, Clone, Copy)]
pub struct MathOp {
    pub lhs: Value,
    pub rhs: Value,
    pub dest: u8,
    pub op_type: OpType
}

#[derive(Debug, Clone, Copy)]
pub struct Operation2 {
    pub lhs: Value,
    pub rhs: Value,
    pub dest: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Operation1 {
    pub value: Value,
    pub dest: u8,
}

#[derive(Debug, Clone, Copy)]
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
    ShiftLeft { op: Operation2 },
    ShiftRight { op: Operation2 },
    Not { op: Operation1 },
    Jump { dest: u32 },
    JumpIfZero { dest: u32, condition: Value },
    JumpIfNotZero { dest: u32, condition: Value },
    LoadMemory { dest: u8, value: Value },
    Store { dest: Value, value: Value },
    Move { dest: u8, value: Value },
}