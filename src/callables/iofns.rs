use std::{
    fmt::{self, Display},
    io::{self, Read as ioRead},
};

use crate::{callables::Callable, value::Value};

#[derive(Debug, Clone)]
struct Print;

impl Display for Print {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "print")
    }
}

impl Callable for Print {
    fn call(&self, args: &[Value]) -> Value {
        let mut it = args.iter();
        if let Some(v) = it.next() {
            print!("{}", v);
        }
        for v in it {
            print!(" {}", v);
        }
        Value::Nil
    }
}

#[derive(Debug, Clone)]
struct Read;

impl Display for Read {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "read")
    }
}

impl Callable for Read {
    fn call(&self, _: &[Value]) -> Value {
        let mut buffer = String::new();
        let result = io::stdin().read_to_string(&mut buffer);
        if let Err(error) = result {
            Value::Error(format!("{}", error))
        } else {
            Value::String(buffer)
        }
    }
}
