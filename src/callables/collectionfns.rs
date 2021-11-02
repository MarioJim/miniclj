use std::collections::VecDeque;

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    memaddress::MemAddress,
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

#[derive(Debug, Clone)]
pub struct First;

impl Callable for First {
    fn name(&self) -> &'static str {
        "first"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<collection>"))
        }
    }

    fn execute(
        &self,
        state: &mut VMState,
        args_addrs: Vec<MemAddress>,
        result_addr: MemAddress,
    ) -> RuntimeResult {
        let maybe_coll_addr = args_addrs.into_iter().next().unwrap();
        let maybe_coll = state.get(&maybe_coll_addr);
        let mut coll_as_list = VecDeque::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        let first = coll_as_list.pop_front().unwrap_or(Value::Nil);
        state.store(result_addr, first);

        Ok(())
    }
}

display_for_callable!(First);

#[derive(Debug, Clone)]
pub struct Rest;

impl Callable for Rest {
    fn name(&self) -> &'static str {
        "rest"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<collection>"))
        }
    }

    fn execute(
        &self,
        _state: &mut VMState,
        _args_addrs: Vec<MemAddress>,
        _result_addr: MemAddress,
    ) -> RuntimeResult {
        todo!()
    }
}

display_for_callable!(Rest);

#[derive(Debug, Clone)]
pub struct Cons;

impl Callable for Cons {
    fn name(&self) -> &'static str {
        "cons"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 2 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<value> <collection>"))
        }
    }
}

display_for_callable!(Cons);

#[derive(Debug, Clone)]
pub struct Conj;

impl Callable for Conj {
    fn name(&self) -> &'static str {
        "conj"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 2 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<collection> <value>"))
        }
    }
}

display_for_callable!(Conj);

#[derive(Debug, Clone)]
pub struct Get;

impl Callable for Get {
    fn name(&self) -> &'static str {
        "get"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 2 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<collection> <key>"))
        }
    }
}

display_for_callable!(Get);

#[derive(Debug, Clone)]
pub struct Len;

impl Callable for Len {
    fn name(&self) -> &'static str {
        "len"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<collection>"))
        }
    }
}

display_for_callable!(Len);

#[derive(Debug, Clone)]
pub struct IsEmpty;

impl Callable for IsEmpty {
    fn name(&self) -> &'static str {
        "empty?"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<collection>"))
        }
    }
}

display_for_callable!(IsEmpty);
