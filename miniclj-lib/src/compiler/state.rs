use std::{collections::HashMap as RustHashMap, io::Write, rc::Rc};

use smol_str::SmolStr;

use crate::{
    callables::{Callable, CallablesTable, HashMap, List, Set, Vector},
    compiler::{CompilationError, CompilationResult, Literal, SExpr, SymbolTable},
    constant::Constant,
    instruction::{Instruction, InstructionPtr},
    memaddress::{Lifetime, MemAddress},
};

#[derive(Debug, Default)]
pub struct CompilerState {
    constants: RustHashMap<Constant, MemAddress>,
    instructions: Vec<Instruction>,
    symbol_table: Rc<SymbolTable>,
    loop_jumps_stack: Vec<(InstructionPtr, Vec<MemAddress>)>,
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

                let res_addr = self.new_address(Lifetime::Temporal);
                let instruction = Instruction::new_call(callable_addr, arg_addrs, res_addr);
                self.add_instruction(instruction);

                Ok(res_addr)
            }
            SExpr::ShortLambda(exprs) => {
                let jump_lambda_instr = Instruction::new_jump(None);
                let jump_lambda_instr_ptr = self.add_instruction(jump_lambda_instr);
                let lambda_start_ptr = self.instruction_ptr();
                let lambda_const = Constant::new_lambda(lambda_start_ptr, 1);
                let lambda_addr = self.insert_constant(lambda_const);

                self.compile_lambda(vec![SmolStr::from("%")], SExpr::Expr(exprs))?;
                self.fill_jump(jump_lambda_instr_ptr, self.instruction_ptr());
                Ok(lambda_addr)
            }
            SExpr::List(exprs) => List.compile(self, exprs),
            SExpr::Vector(exprs) => Vector.compile(self, exprs),
            SExpr::Set(exprs) => Set.compile(self, exprs),
            SExpr::Map(exprs) => HashMap.compile(self, exprs),
            SExpr::Literal(literal) => {
                if let Literal::Symbol(symbol) = literal {
                    self.symbol_table
                        .get(&symbol)
                        .or_else(|| {
                            self.callables_table
                                .get(&symbol)
                                .and_then(|callable| callable.get_as_address(self))
                        })
                        .ok_or(CompilationError::SymbolNotDefined(symbol))
                } else {
                    Ok(self.insert_constant(literal.into()))
                }
            }
        }
    }

    pub fn compile_lambda(
        &mut self,
        arg_names: Vec<SmolStr>,
        body: SExpr,
    ) -> Result<(), CompilationError> {
        self.symbol_table = Rc::new(SymbolTable::new_local(
            self.symbol_table.clone(),
            arg_names.len(),
        ));
        for (arg_idx, arg_name) in arg_names.into_iter().enumerate() {
            let addr = MemAddress::new_local_var(arg_idx);
            self.symbol_table.insert(arg_name, addr);
        }
        let res_addr = self.compile(body)?;
        self.symbol_table = self.symbol_table.parent_table().unwrap();

        let ret_instr = Instruction::new_return(res_addr);
        self.add_instruction(ret_instr);
        Ok(())
    }

    pub fn get_symbol(&self, symbol: &str) -> Option<MemAddress> {
        self.symbol_table.get(symbol)
    }

    pub fn new_address(&self, lifetime: Lifetime) -> MemAddress {
        self.symbol_table.new_address(lifetime)
    }

    pub fn insert_symbol(&self, symbol: SmolStr, address: MemAddress) {
        self.symbol_table.insert(symbol, address);
    }

    pub fn remove_symbol(&self, symbol: &str) {
        self.symbol_table.remove_local(symbol);
    }

    pub fn insert_constant(&mut self, constant: Constant) -> MemAddress {
        if let Some(addr) = self.constants.get(&constant) {
            *addr
        } else {
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
            Instruction::Jump(ptr)
            | Instruction::JumpOnTrue(_, ptr)
            | Instruction::JumpOnFalse(_, ptr) => *ptr = goto,
            _ => panic!("Trying to fill a jump where a different instruction was found"),
        };
    }

    pub fn push_loop_jump(&mut self, instruction_ptr: InstructionPtr, addresses: Vec<MemAddress>) {
        self.loop_jumps_stack.push((instruction_ptr, addresses));
    }

    pub fn pop_loop_jump(&mut self) -> Option<(InstructionPtr, Vec<MemAddress>)> {
        self.loop_jumps_stack.pop()
    }

    pub fn get_callable_addr(&mut self, callable: Box<dyn Callable>) -> MemAddress {
        self.insert_constant(callable.into())
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
