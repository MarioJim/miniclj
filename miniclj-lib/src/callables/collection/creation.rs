use std::collections::HashMap as RustHashMap;

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

#[derive(Debug, Clone)]
pub struct List;

impl Callable for List {
    fn name(&self) -> &'static str {
        "list"
    }

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, mut args: Vec<Value>) -> RuntimeResult<Value> {
        args.reverse();
        Ok(Value::List(args.into_iter().collect()))
    }
}

display_for_callable!(List);

#[derive(Debug, Clone)]
pub struct Vector;

impl Callable for Vector {
    fn name(&self) -> &'static str {
        "vector"
    }

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
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

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
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

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args % 2 == 0 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<...pairs of values>",
            ))
        }
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
