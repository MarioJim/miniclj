use std::fmt::{Debug, Display};

use dyn_clone::DynClone;

use crate::{
    compiler::{CompilationError, CompilationResult, CompilerState, SExpr},
    instruction::Instruction,
    memaddress::{Lifetime, MemAddress},
    vm::{RuntimeResult, VMState, Value},
};

pub trait Callable: Display + Debug + DynClone {
    fn name(&self) -> &'static str;

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let callable_addr = self.find_callable_by_arity(state, args.len())?;

        let arg_addrs = args
            .into_iter()
            .map(|expr| state.compile(expr))
            .collect::<Result<Vec<MemAddress>, CompilationError>>()?;

        let res_addr = state.new_address(Lifetime::Temporal);
        let instruction = Instruction::new_call(callable_addr, arg_addrs, res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult;

    fn execute(&self, state: &VMState, args: Vec<Value>) -> RuntimeResult<Value>;
}

dyn_clone::clone_trait_object!(Callable);
