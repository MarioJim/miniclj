use std::{collections::HashMap as RustHashMap, io::Write, rc::Rc};

use crate::{
    callables::{Callable, CallablesTable, HashMap, List, Set, Vector},
    compiler::{CompilationError, CompilationResult, Literal, SExpr, SymbolTable},
    constant::Constant,
    instruction::{Instruction, InstructionPtr},
    memaddress::MemAddress,
};

#[derive(Debug, Default)]
pub struct CompilerState {
    constants: RustHashMap<Constant, MemAddress>,
    instructions: Vec<Instruction>,
    symbol_table: Rc<SymbolTable>,
    callables_table: CallablesTable,
}

impl CompilerState {
    pub fn compile(&mut self, expr: SExpr) -> CompilationResult {
        match expr {
            SExpr::Expr(exprs) => {
                let mut exprs_iter = exprs.into_iter();
                let first_expr = match exprs_iter.next() {
                    Some(first_expr) => first_expr,
                    None => return self.compile(SExpr::List(Vec::new())),
                };
                if let SExpr::Literal(Literal::Symbol(symbol)) = first_expr.clone() {
                    // Check that there isn't an override for the function
                    if self.symbol_table.get(&symbol).is_none() {
                        return match self.callables_table.get(&symbol) {
                            Some(callable) => callable.compile(self, exprs_iter.collect()),
                            None => Err(CompilationError::CallableNotDefined(symbol)),
                        };
                    }
                }

                let callable_addr = self.compile(first_expr)?;

                let arg_addrs = exprs_iter
                    .map(|expr| self.compile(expr))
                    .collect::<Result<Vec<MemAddress>, CompilationError>>()?;

                let res_addr = self.new_tmp_address();
                let instruction = Instruction::new_call(callable_addr, arg_addrs, res_addr);
                self.add_instruction(instruction);

                Ok(res_addr)
            }
            SExpr::ShortLambda(exprs) => {
                let jump_lambda_instr = Instruction::new_jump(None);
                let jump_lambda_instr_ptr = self.add_instruction(jump_lambda_instr);
                let lambda_start_ptr = self.instruction_ptr();
                let lambda_const = Constant::new_lambda(lambda_start_ptr, 1);
                let lambda_addr = self.insert_in_consttbl(lambda_const);

                self.compile_lambda(vec!["%".to_string()], SExpr::Expr(exprs))?;
                self.fill_jump(jump_lambda_instr_ptr, self.instruction_ptr());
                Ok(lambda_addr)
            }
            SExpr::List(exprs) => List.compile(self, exprs),
            SExpr::Vector(exprs) => Vector.compile(self, exprs),
            SExpr::Set(exprs) => Set.compile(self, exprs),
            SExpr::Map(exprs) => HashMap.compile(self, exprs),
            SExpr::Literal(lit) => {
                if let Literal::Symbol(symbol) = lit {
                    self.symbol_table
                        .get(&symbol)
                        .ok_or(CompilationError::SymbolNotDefined(symbol))
                } else {
                    Ok(self.insert_in_consttbl(lit.into()))
                }
            }
        }
    }

    pub fn compile_lambda(
        &mut self,
        arg_names: Vec<String>,
        body: SExpr,
    ) -> Result<(), CompilationError> {
        self.symbol_table = Rc::new(SymbolTable::new(Some(self.symbol_table.clone())));
        for (arg_idx, arg_name) in arg_names.into_iter().enumerate() {
            let addr = MemAddress::new_local_var(arg_idx);
            self.symbol_table.insert(arg_name, addr);
        }
        let res_addr = self.compile(body)?;
        self.symbol_table = self.symbol_table.get_top_scope().unwrap();

        let ret_instr = Instruction::new_return(res_addr);
        self.add_instruction(ret_instr);
        Ok(())
    }

    pub fn has_symbol_in_symtbl(&self, symbol: &str) -> bool {
        self.symbol_table.get(symbol).is_some()
    }

    pub fn insert_in_symtbl(&mut self, symbol: String, value: MemAddress) {
        self.symbol_table.insert(symbol, value);
    }

    pub fn insert_in_root_symtbl(&mut self, symbol: String, value: MemAddress) {
        self.symbol_table.insert_in_root(symbol, value);
    }

    pub fn insert_in_consttbl(&mut self, constant: Constant) -> MemAddress {
        match self.constants.get(&constant) {
            Some(addr) => *addr,
            None => {
                let next_idx = self
                    .constants
                    .iter()
                    .map(|(_, a)| a.idx() + 1)
                    .max()
                    .unwrap_or(0);
                let addr = MemAddress::new_const(next_idx);
                self.constants.insert(constant, addr);
                addr
            }
        }
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

    pub fn get_callable_addr(&mut self, callable: Box<dyn Callable>) -> MemAddress {
        self.insert_in_consttbl(callable.into())
    }

    pub fn new_tmp_address(&mut self) -> MemAddress {
        MemAddress::new_temp(self.symbol_table.get_new_temp_addr_idx())
    }

    pub fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        for (constant, addr) in &self.constants {
            writer.write_fmt(format_args!("{} {}\n", addr, constant))?;
        }
        writer.write_all(b"***\n")?;
        for instruction in &self.instructions {
            writer.write_fmt(format_args!("{}\n", instruction))?;
        }
        Ok(())
    }

    pub fn into_parts(self) -> (RustHashMap<Constant, MemAddress>, Vec<Instruction>) {
        (self.constants, self.instructions)
    }
}
