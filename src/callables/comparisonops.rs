use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, SExpr, State},
    instruction::Instruction,
    memaddress::{DataType, MemAddress},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl Callable for ComparisonOp {
    fn name(&self) -> &'static str {
        match self {
            ComparisonOp::Eq => "=",
            ComparisonOp::Ne => "!=",
            ComparisonOp::Gt => ">",
            ComparisonOp::Lt => "<",
            ComparisonOp::Ge => ">=",
            ComparisonOp::Le => "<=",
        }
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.is_empty() {
            return Err(CompilationError::EmptyArgs(self.name()));
        }
        let arg_addrs = args
            .into_iter()
            .map(|expr| state.compile(expr))
            .collect::<Result<Vec<MemAddress>, CompilationError>>()?;

        let res_addr = state.new_tmp_address(DataType::Number);
        let instruction = Instruction::new_builtin_call(self.name(), arg_addrs, res_addr);

        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(ComparisonOp);
