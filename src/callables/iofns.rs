use std::io::{self, Read as ioRead};

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    SExpr, Scope, Value,
};

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Scope) -> ExecutionResult {
        let mut it = args.into_iter().map(|v| v.eval(scope));
        if let Some(v) = it.next() {
            print!("{}", v?);
        }
        for v in it {
            print!(" {}", v?);
        }
        Ok(Value::Nil)
    }
}

display_for_callable!(Print);

#[derive(Debug, Clone)]
pub struct Read;

impl Callable for Read {
    fn name(&self) -> &'static str {
        "read"
    }

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| RuntimeError::Error(e.to_string()))?;
        Ok(Value::String(buffer))
    }
}

display_for_callable!(Read);
