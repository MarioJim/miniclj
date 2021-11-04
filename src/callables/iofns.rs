use std::io;

use escape8259::unescape;

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 0 {
            Err(CompilationError::EmptyArgs(self.name()))
        } else {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        if let Some(v) = args_iter.next() {
            if let Value::String(s) = v {
                print!("{}", unescape(&s).unwrap());
            } else {
                print!("{}", v);
            }
        }
        for v in args_iter {
            if let Value::String(s) = v {
                print!(" {}", unescape(&s).unwrap());
            } else {
                print!(" {}", v);
            }
        }
        Ok(Value::Nil)
    }
}

display_for_callable!(Print);

#[derive(Debug, Clone)]
pub struct Println;

impl Callable for Println {
    fn name(&self) -> &'static str {
        "println"
    }

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, state: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let result = Print.execute(state, args)?;
        println!();
        Ok(result)
    }
}

display_for_callable!(Println);

#[derive(Debug, Clone)]
pub struct Read;

impl Callable for Read {
    fn name(&self) -> &'static str {
        "read"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 0 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(self.name(), ""))
        }
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| RuntimeError::Error(e.to_string()))?;
        Ok(Value::String(String::from(buffer.trim_end())))
    }
}

display_for_callable!(Read);
