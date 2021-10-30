use std::{borrow::Borrow, collections::HashMap, io::Write, rc::Rc};

use crate::{
    compiler::{CallablesTable, CompilationError, CompilationResult, Literal, SExpr, SymbolTable},
    constant::Constant,
    instruction::{Instruction, InstructionPtr},
    memaddress::{DataType, MemAddress},
};

#[derive(Debug, Default)]
pub struct State {
    constant_var_idx: usize,
    temporal_var_idx: usize,
    instructions: Vec<Instruction>,
    symbol_table: Rc<SymbolTable>,
    constants: HashMap<Constant, MemAddress>,
    callables_table: CallablesTable,
}

impl State {
    pub fn compile(&mut self, expr: SExpr) -> CompilationResult {
        match expr {
            SExpr::Expr(symbol, exprs) => self
                .callables_table
                .get(&symbol)
                .ok_or(CompilationError::CallableNotDefined(symbol))?
                .compile(self, exprs),
            SExpr::List(_) => todo!(),
            SExpr::Vector(_) => todo!(),
            SExpr::Set(_) => todo!(),
            SExpr::Map(_) => todo!(),
            SExpr::Literal(lit) => {
                if let Literal::Symbol(symbol) = lit {
                    self.symbol_table
                        .get(&symbol)
                        .ok_or(CompilationError::SymbolNotDefined(symbol))
                } else {
                    let constant: Constant = lit.into();
                    match self.constants.get(&constant) {
                        Some(addr) => Ok(*addr),
                        None => {
                            let addr = MemAddress::new_constant(
                                constant.data_type(),
                                self.constant_var_idx,
                            );
                            self.constants.insert(constant, addr);
                            self.constant_var_idx += 1;
                            Ok(addr)
                        }
                    }
                }
            }
        }
    }

    pub fn push_scope(&mut self) {
        self.symbol_table = Rc::new(SymbolTable::new(Some(self.symbol_table.clone())));
    }

    pub fn pop_scope(&mut self) {
        self.symbol_table = match self.symbol_table.borrow() {
            SymbolTable::RootScope(_) => {
                panic!("Called compiler::Scope::pop_scope with the root scope")
            }
            SymbolTable::LocalScope(_, parent_table) => parent_table.clone(),
        };
    }

    /// Returns the index of the next instruction to be inserted
    pub fn instruction_ptr(&self) -> usize {
        self.instructions.len()
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> InstructionPtr {
        self.instructions.push(instruction);
        self.instructions.len() - 1
    }

    pub fn fill_jump(&mut self, instruction_ptr: InstructionPtr, goto: InstructionPtr) {
        let instr = self.instructions.get_mut(instruction_ptr).unwrap();
        match instr {
            Instruction::Jump(ptr) => *ptr = goto,
            Instruction::JumpOnTrue(_, ptr) => *ptr = goto,
            Instruction::JumpOnFalse(_, ptr) => *ptr = goto,
            _ => panic!("Trying to fill a jump where a different instruction was found"),
        }
    }

    pub fn new_tmp_address(&mut self, datatype: DataType) -> MemAddress {
        let addr = MemAddress::new_temp(datatype, self.temporal_var_idx);
        self.temporal_var_idx += 1;
        addr
    }

    pub fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        for (constant, addr) in &self.constants {
            writer.write_fmt(format_args!("{} {}\n", addr, constant))?;
        }
        for instruction in &self.instructions {
            writer.write_fmt(format_args!("{}\n", instruction))?;
        }
        Ok(())
    }
}
