use std::{io, rc::Rc};

use escape8259::unescape;

use crate::compiler::{
    callables::{Callable, ExecutionResult, RuntimeError},
    SExpr, Scope, Value,
};

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        let mut it = args.into_iter().map(|v| v.eval(scope));
        if let Some(v) = it.next() {
            if let Ok(Value::String(s)) = v {
                print!("{}", unescape(&s).unwrap());
            } else {
                print!("{}", v?);
            }
        }
        for v in it {
            if let Ok(Value::String(s)) = v {
                print!(" {}", unescape(&s).unwrap());
            } else {
                print!(" {}", v?);
            }
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

    fn call(&self, _: Vec<SExpr>, _: &Rc<Scope>) -> ExecutionResult {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| RuntimeError::Error(e.to_string()))?;
        Ok(Value::String(String::from(buffer.trim_end())))
    }
}

display_for_callable!(Read);
