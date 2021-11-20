use std::collections::HashMap as RustHashMap;

use crate::callables::prelude::*;

#[derive(Debug, Clone)]
pub struct List;

impl Callable for List {
    fn name(&self) -> &'static str {
        "list"
    }

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        Ok(Value::List(args.into_iter().rev().collect()))
    }
}

display_for_callable!(List);

#[derive(Debug, Clone)]
pub struct Vector;

impl Callable for Vector {
    fn name(&self) -> &'static str {
        "vector"
    }

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        Ok(Value::Vector(args))
    }
}

display_for_callable!(Vector);

#[derive(Debug, Clone)]
pub struct Set;

impl Callable for Set {
    fn name(&self) -> &'static str {
        "set"
    }

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        Ok(Value::Set(args.into_iter().collect()))
    }
}

display_for_callable!(Set);

#[derive(Debug, Clone)]
pub struct HashMap;

impl Callable for HashMap {
    fn name(&self) -> &'static str {
        "hash-map"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args % 2 == 0 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<...pairs of values>",
            ))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() % 2 == 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a pair number of values",
                args.len(),
            ));
        }

        let mut hashmap = RustHashMap::new();
        let mut args_iter = args.into_iter();
        while let Some(key) = args_iter.next() {
            let val = args_iter.next().unwrap();
            hashmap.insert(key, val);
        }

        Ok(Value::Map(hashmap))
    }
}

display_for_callable!(HashMap);
