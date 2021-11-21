use crate::{callables::prelude::*, vm::List};

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
pub struct Del;

impl Callable for Del {
    fn name(&self) -> &'static str {
        "del"
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
        if args.len() != 2 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a collection and any number of values",
                args.len(),
            ));
        }

        let mut args_iter = args.into_iter();
        let maybe_coll = args_iter.next().unwrap();

        match maybe_coll {
            Value::Set(mut set) => {
                for value in args_iter {
                    set.remove(&value);
                }
                Ok(Value::Set(set))
            }
            Value::Map(mut map) => {
                for value in args_iter {
                    map.remove(&value);
                }
                Ok(Value::Map(map))
            }
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "an unordered collection",
                maybe_coll.type_str(),
            )),
        }
    }
}

display_for_callable!(Del);
