use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Call {
        register_idx: usize,
        instruction: String,
        registers: Vec<String>,
    },
    ConditionalJump(usize),
    InconditionalJump(usize),
    PushScope,
    PopScope,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Call {
                instruction,
                register_idx,
                registers,
            } => write!(
                f,
                "t{} {} {}",
                instruction,
                register_idx,
                registers.join(" ")
            ),
            Instruction::ConditionalJump(line) => write!(f, "jne {}", line),
            Instruction::InconditionalJump(line) => write!(f, "jmp {}", line),
            Instruction::PushScope => write!(f, "push"),
            Instruction::PopScope => write!(f, "pop"),
        }
    }
}
