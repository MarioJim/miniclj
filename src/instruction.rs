use std::fmt::{self, Display};

use crate::memaddress::MemAddress;

pub type InstructionPtr = usize;

#[derive(Debug)]
pub enum Instruction {
    Call {
        callable: MemAddress,
        args: Vec<MemAddress>,
        result_addr: MemAddress,
    },
    Return(MemAddress),
    Assignment {
        src: MemAddress,
        dst: MemAddress,
    },
    Jump(InstructionPtr),
    JumpOnTrue(MemAddress, InstructionPtr),
    JumpOnFalse(MemAddress, InstructionPtr),
}

impl Instruction {
    pub fn new_call(
        callable: MemAddress,
        args: Vec<MemAddress>,
        result_addr: MemAddress,
    ) -> Instruction {
        Instruction::Call {
            callable,
            args,
            result_addr,
        }
    }

    pub fn new_return(return_addr: MemAddress) -> Instruction {
        Instruction::Return(return_addr)
    }

    pub fn new_assignment(src: MemAddress, dst: MemAddress) -> Instruction {
        Instruction::Assignment { src, dst }
    }

    pub fn new_jump(direction: Option<(bool, MemAddress)>) -> Instruction {
        match direction {
            Some((true, addr)) => Instruction::JumpOnTrue(addr, 0),
            Some((false, addr)) => Instruction::JumpOnFalse(addr, 0),
            None => Instruction::Jump(0),
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
            } => {
                write!(f, "call {}", callable,)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, " {}", result_addr)
            }
            Instruction::Return(addr) => write!(f, "ret {}", addr),
            Instruction::Assignment { src, dst } => write!(f, "mov {} {}", src, dst),
            Instruction::Jump(ins_ptr) => write!(f, "jmp {}", ins_ptr),
            Instruction::JumpOnTrue(addr, ins_ptr) => {
                write!(f, "jmpT {} {}", addr, ins_ptr)
            }
            Instruction::JumpOnFalse(addr, ins_ptr) => {
                write!(f, "jmpF {} {}", addr, ins_ptr)
            }
        }
    }
}
