use serde::{Deserialize, Serialize};
use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub strings: Vec<String>,
}