use crate::{
    callables::prelude::*,
    compiler::{CompilationResult, SExpr},
};

#[derive(Debug, Clone)]
pub struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 0 {
            Err(CompilationError::EmptyArgs(self.name()))
        } else {
            Ok(())
        }
    }

    fn inner_compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let mut args_iter = args.into_iter();
        let mut res_addr = state.compile(args_iter.next().unwrap())?;
        for arg in args_iter {
            res_addr = state.compile(arg)?;
        }

        Ok(res_addr)
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Do);
