use std::fmt::{self, Display};

use super::{InstructionPtr, MemAddress};

#[derive(Debug)]
pub enum Instruction {
    Call {
        callable: String,
        args: Vec<MemAddress>,
        result_addr: MemAddress,
    },
    ConditionalJump(MemAddress, InstructionPtr),
    InconditionalJump(InstructionPtr),
    GoSub(MemAddress),
    Return(MemAddress),
}

impl Instruction {
    pub fn new_call(
        callable: String,
        args: Vec<MemAddress>,
        result_addr: MemAddress,
    ) -> Instruction {
        Instruction::Call {
            callable,
            args,
            result_addr,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Call {
                callable,
                args,
                result_addr,
            } => write!(
                f,
                "{} {} {}",
                callable,
                args.iter()
                    .map(|a| usize::from(a).to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                usize::from(result_addr)
            ),
            Instruction::ConditionalJump(addr, ins_ptr) => {
                write!(f, "jne {} {}", usize::from(addr), ins_ptr)
            }
            Instruction::InconditionalJump(ins_ptr) => write!(f, "jmp {}", ins_ptr),
            Instruction::GoSub(addr) => write!(f, "gosub {}", usize::from(addr)),
            Instruction::Return(addr) => write!(f, "ret {}", usize::from(addr)),
        }
    }
}
