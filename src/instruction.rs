use std::fmt::{self, Display};

use crate::memaddress::MemAddress;

pub type InstructionPtr = usize;

#[derive(Debug)]
pub enum Instruction {
    Call {
        callable: String,
        args: Vec<MemAddress>,
        result_addr: MemAddress,
    },
    Jump(InstructionPtr),
    JumpOnTrue(MemAddress, InstructionPtr),
    JumpOnFalse(MemAddress, InstructionPtr),
    GoSub(MemAddress),
    Return(MemAddress),
}

impl Instruction {
    pub fn new_builtin_call(
        callable: &str,
        args: Vec<MemAddress>,
        result_addr: MemAddress,
    ) -> Instruction {
        Instruction::Call {
            callable: callable.to_string(),
            args,
            result_addr,
        }
    }

    pub fn new_assignment(from: MemAddress, to: MemAddress) -> Instruction {
        Instruction::Call {
            callable: String::from("="),
            args: vec![from],
            result_addr: to,
        }
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
                write!(f, "{}", callable,)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, " {}", result_addr)
            }
            Instruction::Jump(ins_ptr) => write!(f, "jump {}", ins_ptr),
            Instruction::JumpOnTrue(addr, ins_ptr) => {
                write!(f, "jmpT {} {}", addr, ins_ptr)
            }
            Instruction::JumpOnFalse(addr, ins_ptr) => {
                write!(f, "jmpF {} {}", addr, ins_ptr)
            }
            Instruction::GoSub(addr) => write!(f, "gosub {}", addr),
            Instruction::Return(addr) => write!(f, "ret {}", addr),
        }
    }
}
