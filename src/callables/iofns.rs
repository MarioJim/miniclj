use std::io::{self, Read as ioRead};

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    Scope, Value,
};

#[derive(Debug, Clone)]
struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        let mut it = args.iter();
        if let Some(v) = it.next() {
            print!("{}", v);
        }
        for v in it {
            print!(" {}", v);
        }
        Ok(Value::Nil)
    }
}

display_for_callable!(Print);

#[derive(Debug, Clone)]
struct Read;

impl Callable for Read {
    fn name(&self) -> &'static str {
        "read"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| RuntimeError::GenericError(format!("{}", e)))?;
        Ok(Value::String(buffer))
    }
}

display_for_callable!(Read);
