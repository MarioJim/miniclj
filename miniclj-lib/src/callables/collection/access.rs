use crate::{callables::prelude::*, vm::List};

#[derive(Debug, Clone)]
pub struct First;

impl Callable for First {
    fn name(&self) -> &'static str {
        "first"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection",
                args.len(),
            ));
        }

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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection",
                args.len(),
            ));
        }

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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 2 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<value> <collection>",
            ))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a value and a collection",
                args.len(),
            ));
        }

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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args != 0 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<collection> <...values>",
            ))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.is_empty() {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection and any number of values",
                args.len(),
            ));
        }

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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 2 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<collection> <index>",
            ))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection and an index",
                args.len(),
            ));
        }

        let mut args_iter = args.into_iter();
        let maybe_coll = args_iter.next().unwrap();
        let maybe_coll_type = maybe_coll.type_str();
        let index = args_iter.next().unwrap().as_usize().map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a positive number", type_str)
        })?;

        match maybe_coll {
            Value::List(l) => l
                .nth(index)
                .ok_or(RuntimeError::IndexOutOfBounds(maybe_coll_type)),
            Value::Vector(v) => v
                .into_iter()
                .nth(index)
                .ok_or(RuntimeError::IndexOutOfBounds(maybe_coll_type)),
            Value::String(s) => s
                .chars()
                .nth(index)
                .ok_or(RuntimeError::IndexOutOfBounds(maybe_coll_type))
                .map(|c| Value::String(String::from(c))),
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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 2 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<collection> <key>",
            ))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection and a key",
                args.len(),
            ));
        }

        let mut args_iter = args.into_iter();
        let maybe_coll = args_iter.next().unwrap();
        let key = args_iter.next().unwrap();

        match maybe_coll {
            Value::List(_) => Ok(Value::Nil),
            Value::Vector(v) => {
                let index = key.as_usize().map_err(|type_str| {
                    RuntimeError::WrongDataType(self.name(), "a positive number", type_str)
                })?;
                Ok(v.into_iter().nth(index).unwrap_or(Value::Nil))
            }
            Value::Set(s) => Ok(s.get(&key).cloned().unwrap_or(Value::Nil)),
            Value::Map(m) => Ok(m.get(&key).cloned().unwrap_or(Value::Nil)),
            Value::String(s) => {
                let index = key.as_usize().map_err(|type_str| {
                    RuntimeError::WrongDataType(self.name(), "a positive number", type_str)
                })?;
                Ok(s.chars()
                    .nth(index)
                    .map_or(Value::Nil, |c| Value::String(String::from(c))))
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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection",
                args.len(),
            ));
        }

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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<collection>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection",
                args.len(),
            ));
        }

        let maybe_coll = args.into_iter().next().unwrap();
        match maybe_coll {
            Value::List(List::EmptyList) => Ok(true),
            Value::List(List::Cons(..)) => Ok(false),
            Value::Vector(v) => Ok(v.is_empty()),
            Value::Set(s) => Ok(s.is_empty()),
            Value::Map(m) => Ok(m.is_empty()),
            Value::String(s) => Ok(s.is_empty()),
            Value::Nil => Ok(true),
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a collection",
                maybe_coll.type_str(),
            )),
        }
        .map(Value::from)
    }
}

display_for_callable!(IsEmpty);
