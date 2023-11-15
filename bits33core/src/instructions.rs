use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Value {
    Register(u8),
    Immediate(u32),
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum OpType {
    Float,
    Int,
    UnsignedInt,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Op2wTypes {
    pub lhs: Value,
    pub rhs: Value,
    pub dest: u8,
    pub op_type: OpType
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Op2 {
    pub lhs: Value,
    pub rhs: Value,
    pub dest: u8,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Op1 {
    pub value: Value,
    pub dest: u8,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Instruction {
    Nop,
    Add { op: Op2wTypes },
    Sub { op: Op2wTypes },
    Mul { op: Op2wTypes },
    Div { op: Op2wTypes },
    Mod { op: Op2wTypes },
    GreaterThanOrEqual { op: Op2wTypes },
    LessThanOrEqual { op: Op2wTypes },
    GreaterThan { op: Op2wTypes },
    LessThan { op: Op2wTypes },
    And { op: Op2 },
    Or { op: Op2 },
    Xor { op: Op2 },
    ShiftLeft { op: Op2 },
    ShiftRight { op: Op2 },
    Equal { op: Op2 },
    NotEqual { op: Op2 },
    Not { op: Op1 },
    Jump { dest: u32 },
    JumpIfZero { dest: u32, condition: Value },
    JumpIfNotZero { dest: u32, condition: Value },
    LoadMemory { dest: u8, value: Value },
    Store { dest: Value, value: Value },
    Move { dest: u8, value: Value },
}