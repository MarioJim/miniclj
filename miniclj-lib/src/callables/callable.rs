use std::fmt::{Debug, Display};

use dyn_clone::DynClone;

use crate::{
    compiler::{CompilationError, CompilationResult, CompilerState, SExpr},
    instruction::Instruction,
    memaddress::{Lifetime, MemAddress},
    vm::{RuntimeResult, VMState, Value},
};

/// Base trait that all language callables must implement
pub trait Callable: Display + Debug + DynClone {
    fn name(&self) -> &'static str;

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        self.check_arity(args.len())?;
        self.inner_compile(state, args)
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError>;

    fn inner_compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let callable_addr = self
            .get_as_address(state)
            .expect("Callable didn't override either get_as_address or inner_compile");

        let arg_addrs = args
            .into_iter()
            .map(|expr| state.compile(expr))
            .collect::<Result<Vec<MemAddress>, CompilationError>>()?;

        let res_addr = state.new_address(Lifetime::Temporal);
        let instruction = Instruction::new_call(callable_addr, arg_addrs, res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }

    fn get_as_address(&self, _state: &mut CompilerState) -> Option<MemAddress> {
        None
    }

    fn execute(&self, state: &VMState, args: Vec<Value>) -> RuntimeResult<Value>;
}

dyn_clone::clone_trait_object!(Callable);
