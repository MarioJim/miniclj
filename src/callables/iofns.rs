use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, SExpr, State},
    instruction::Instruction,
    memaddress::{DataType, MemAddress},
};

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.is_empty() {
            return Err(CompilationError::EmptyArgs(self.name()));
        }
        let arg_addrs = args
            .into_iter()
            .map(|expr| state.compile(expr))
            .collect::<Result<Vec<MemAddress>, CompilationError>>()?;

        let res_addr = state.new_tmp_address(DataType::Nil);
        let instruction = Instruction::new_builtin_call(self.name(), arg_addrs, res_addr);

        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Print);

#[derive(Debug, Clone)]
pub struct Println;

impl Callable for Println {
    fn name(&self) -> &'static str {
        "println"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.is_empty() {
            return Err(CompilationError::EmptyArgs(self.name()));
        }
        let arg_addrs = args
            .into_iter()
            .map(|expr| state.compile(expr))
            .collect::<Result<Vec<MemAddress>, CompilationError>>()?;

        let res_addr = state.new_tmp_address(DataType::Nil);
        let instruction = Instruction::new_builtin_call(self.name(), arg_addrs, res_addr);

        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Println);

#[derive(Debug, Clone)]
pub struct Read;

impl Callable for Read {
    fn name(&self) -> &'static str {
        "read"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if !args.is_empty() {
            return Err(CompilationError::Arity(self.name(), ""));
        }
        let res_addr = state.new_tmp_address(DataType::String);
        let instruction = Instruction::new_builtin_call(self.name(), vec![], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Read);
