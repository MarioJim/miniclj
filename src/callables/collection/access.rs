use num::{Signed, ToPrimitive};

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{List, RuntimeError, RuntimeResult, VMState, Value},
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
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let maybe_coll = args.into_iter().next().unwrap();
        let coll_as_list = List::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        let first = match coll_as_list {
            List::Cons(first, _) => *first,
            List::EmptyList => Value::Nil,
        };
        Ok(first)
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
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let maybe_coll = args.into_iter().next().unwrap();
        let coll_as_list = List::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        let rest = match coll_as_list {
            List::Cons(_, rest) => *rest,
            List::EmptyList => List::EmptyList,
        };
        Ok(Value::List(rest))
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
            Err(CompilationError::WrongArity(
                self.name(),
                "<value> <collection>",
            ))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        let value = args_iter.next().unwrap();
        let maybe_coll = args_iter.next().unwrap();

        let coll_as_list = List::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        let list = List::Cons(Box::new(value), Box::new(coll_as_list));
        Ok(Value::List(list))
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
        if num_args == 0 {
            Err(CompilationError::WrongArity(
                self.name(),
                "<collection> <...values>",
            ))
        } else {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        let maybe_coll = args_iter.next().unwrap();

        match maybe_coll {
            Value::List(mut list) => {
                for value in args_iter {
                    list = List::Cons(Box::new(value), Box::new(list));
                }
                Ok(Value::List(list))
            }
            Value::Vector(mut vector) => {
                for value in args_iter {
                    vector.push(value);
                }
                Ok(Value::Vector(vector))
            }
            Value::Set(mut set) => {
                for value in args_iter {
                    set.insert(value);
                }
                Ok(Value::Set(set))
            }
            Value::Map(mut map) => {
                for value in args_iter {
                    let (key, val) = value.into_map_entry()?;
                    map.insert(key, val);
                }
                Ok(Value::Map(map))
            }
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a collection",
                maybe_coll.type_str(),
            )),
        }
    }
}

display_for_callable!(Conj);

#[derive(Debug, Clone)]
pub struct Nth;

impl Callable for Nth {
    fn name(&self) -> &'static str {
        "nth"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 2 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<collection> <index>",
            ))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        let maybe_coll = args_iter.next().unwrap();
        let maybe_coll_type = maybe_coll.type_str();
        let key = args_iter.next().unwrap();

        match (maybe_coll, key) {
            (Value::List(l), Value::Number(n)) => {
                if n.is_integer() && n.is_positive() {
                    let idx = n.to_usize().unwrap();
                    l.nth(idx)
                        .ok_or(RuntimeError::IndexOutOfBounds(maybe_coll_type))
                } else {
                    Err(RuntimeError::IndexOutOfBounds(maybe_coll_type))
                }
            }
            (Value::Vector(v), Value::Number(n)) => {
                if n.is_integer() && n.is_positive() {
                    let idx = n.to_usize().unwrap();
                    v.into_iter()
                        .nth(idx)
                        .ok_or(RuntimeError::IndexOutOfBounds(maybe_coll_type))
                } else {
                    Err(RuntimeError::IndexOutOfBounds(maybe_coll_type))
                }
            }
            (Value::String(s), Value::Number(n)) => {
                if n.is_integer() && n.is_positive() {
                    let idx = n.to_usize().unwrap();
                    s.chars()
                        .nth(idx)
                        .map(|c| Value::String(String::from(c)))
                        .ok_or(RuntimeError::IndexOutOfBounds(maybe_coll_type))
                } else {
                    Err(RuntimeError::IndexOutOfBounds(maybe_coll_type))
                }
            }
            (Value::List(_) | Value::Vector(_) | Value::String(_), key) => Err(
                RuntimeError::WrongDataType(self.name(), "a number", key.type_str()),
            ),
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a collection",
                maybe_coll_type,
            )),
        }
    }
}

display_for_callable!(Nth);

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
            Err(CompilationError::WrongArity(
                self.name(),
                "<collection> <key>",
            ))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        let maybe_coll = args_iter.next().unwrap();
        let key = args_iter.next().unwrap();

        match maybe_coll {
            Value::List(_) => Ok(Value::Nil),
            Value::Vector(v) => {
                if let Value::Number(n) = key {
                    if n.is_integer() && n.is_positive() {
                        let idx = n.to_usize().unwrap();
                        Ok(v.into_iter().nth(idx).unwrap_or(Value::Nil))
                    } else {
                        Ok(Value::Nil)
                    }
                } else {
                    Ok(Value::Nil)
                }
            }
            Value::Set(s) => Ok(s.get(&key).cloned().unwrap_or(Value::Nil)),
            Value::Map(m) => Ok(m.get(&key).cloned().unwrap_or(Value::Nil)),
            Value::String(s) => {
                if let Value::Number(n) = key {
                    if n.is_integer() && n.is_positive() {
                        let idx = n.to_usize().unwrap();
                        Ok(s.chars()
                            .nth(idx)
                            .map_or(Value::Nil, |c| Value::String(String::from(c))))
                    } else {
                        Ok(Value::Nil)
                    }
                } else {
                    Ok(Value::Nil)
                }
            }
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a collection",
                maybe_coll.type_str(),
            )),
        }
    }
}

display_for_callable!(Get);

#[derive(Debug, Clone)]
pub struct Count;

impl Callable for Count {
    fn name(&self) -> &'static str {
        "count"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let maybe_coll = args.into_iter().next().unwrap();
        match maybe_coll {
            Value::List(l) => Ok(l.len()),
            Value::Vector(v) => Ok(v.len()),
            Value::Set(s) => Ok(s.len()),
            Value::Map(m) => Ok(m.len()),
            Value::String(s) => Ok(s.chars().count()),
            Value::Nil => Ok(0),
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a collection",
                maybe_coll.type_str(),
            )),
        }
        .map(|count| Value::from(count as i64))
    }
}

display_for_callable!(Count);

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
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn execute(&self, state: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        Count
            .execute(state, args)
            .map(|count| Value::from(count.as_int().unwrap() == 0))
            .map_err(|_| RuntimeError::WrongDataType(self.name(), "a collection", "another value"))
    }
}

display_for_callable!(IsEmpty);
