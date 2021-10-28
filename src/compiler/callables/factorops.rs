use crate::compiler::{
    callables::{Callable, CompilationResult},
    CompilationError, DataType, Instruction, MemAddress, SExpr, State,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FactorOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Callable for FactorOp {
    fn name(&self) -> &'static str {
        match self {
            FactorOp::Add => "+",
            FactorOp::Sub => "-",
            FactorOp::Mul => "*",
            FactorOp::Div => "/",
        }
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.is_empty() && (matches!(self, FactorOp::Sub) || matches!(self, FactorOp::Div)) {
            return self.arity_err("at least one argument");
        }
        let arg_mem_addrs = args
            .into_iter()
            .map(|expr| state.compile(expr))
            .collect::<Result<Vec<MemAddress>, CompilationError>>()?;

        let res_addr = state.new_tmp_address(DataType::Number);
        let instruction =
            Instruction::new_call(self.name().to_string(), arg_mem_addrs, res_addr.clone());

        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(FactorOp);
