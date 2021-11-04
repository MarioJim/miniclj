use crate::{
    callables::{Callable, CallableResult},
    compiler::{CompilationError, CompilationResult, CompilerState, SExpr},
    vm::{RuntimeError, VMState, Value},
};

#[derive(Debug, Clone)]
pub struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.is_empty() {
            return Err(CompilationError::EmptyArgs(self.name()));
        }

        let mut args_iter = args.into_iter();
        let mut res_addr = state.compile(args_iter.next().unwrap())?;
        for arg in args_iter {
            res_addr = state.compile(arg)?;
        }

        Ok(res_addr)
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> CallableResult {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Do);
