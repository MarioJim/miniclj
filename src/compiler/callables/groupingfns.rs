use crate::compiler::{Callable, CompilationError, CompilationResult, SExpr, State};

#[derive(Debug, Clone)]
pub struct Do;

impl Callable for Do {
    fn name(&self) -> &'static str {
        "do"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
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
}

display_for_callable!(Do);
