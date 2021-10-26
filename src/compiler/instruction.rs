use std::fmt::{self, Display};

use super::{InstructionPtr, MemAddress};

#[derive(Debug)]
pub enum Instruction {
    Call {
        instruction: MemAddress,
        parameters: Vec<MemAddress>,
        result_address: MemAddress,
    },
    ConditionalJump(MemAddress, InstructionPtr),
    InconditionalJump(InstructionPtr),
    GoSub(MemAddress),
    EndFunc,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Call {
                instruction,
                parameters,
                result_address,
            } => write!(
                f,
                "{} {} {}",
                usize::from(instruction),
                parameters
                    .iter()
                    .map(|a| usize::from(a).to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                usize::from(result_address)
            ),
            Instruction::ConditionalJump(addr, ins_ptr) => {
                write!(f, "jne {} {}", usize::from(addr), ins_ptr)
            }
            Instruction::InconditionalJump(ins_ptr) => write!(f, "jmp {}", ins_ptr),
            Instruction::GoSub(addr) => write!(f, "gosub {}", usize::from(addr)),
            Instruction::EndFunc => write!(f, "endfunc"),
        }
    }
}
