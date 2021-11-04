use std::collections::VecDeque;

use crate::{
    callables::{conditionals::IsTrue, Callable, CallableResult},
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{RuntimeError, Value},
};

#[derive(Debug, Clone)]
pub struct Map;

impl Callable for Map {
    fn name(&self) -> &'static str {
        "map"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args >= 2 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(
                self.name(),
                "<function> <...collections>",
            ))
        }
    }

    fn execute(&self, args: Vec<Value>) -> CallableResult {
        let mut args_iter = args.into_iter();
        let maybe_fn = args_iter.next().unwrap();
        let fn_value = match maybe_fn {
            Value::Callable(..) => Ok(maybe_fn),
            Value::Lambda(_, arity) => {
                if arity == args_iter.len() {
                    Ok(maybe_fn)
                } else {
                    Err(RuntimeError::WrongArity(arity, args_iter.len()))
                }
            }
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a function",
                maybe_fn.type_str(),
            )),
        }?;

        let mut lists = Vec::new();
        for arg in args_iter {
            lists.push(VecDeque::try_from(arg).map_err(|type_str| {
                RuntimeError::WrongDataType(self.name(), "a collection", type_str)
            })?);
        }
        let mut result = VecDeque::new();
        loop {
            let mut args_for_callable = Vec::new();
            for list in lists.iter_mut() {
                match list.pop_front() {
                    Some(val) => args_for_callable.push(val),
                    None => return Ok(Value::List(result)),
                }
            }
            let current_result = match &fn_value {
                Value::Callable(callable) => callable.execute(args_for_callable)?,
                Value::Lambda(_, _) => todo!(),
                _ => unreachable!(),
            };
            result.push_back(current_result);
        }
    }
}

display_for_callable!(Map);

#[derive(Debug, Clone)]
pub struct Filter;

impl Callable for Filter {
    fn name(&self) -> &'static str {
        "filter"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 2 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(
                self.name(),
                "<function> <collection>",
            ))
        }
    }

    fn execute(&self, args: Vec<Value>) -> CallableResult {
        let mut args_iter = args.into_iter();
        let maybe_fn = args_iter.next().unwrap();
        let maybe_coll = args_iter.next().unwrap();

        let fn_value = match maybe_fn {
            Value::Callable(..) => Ok(maybe_fn),
            Value::Lambda(_, arity) => {
                if arity == 1 {
                    Ok(maybe_fn)
                } else {
                    Err(RuntimeError::WrongArity(arity, 1))
                }
            }
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a function",
                maybe_fn.type_str(),
            )),
        }?;

        let coll = VecDeque::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        let mut result = VecDeque::new();
        for val in coll {
            let current_result = match &fn_value {
                Value::Callable(callable) => callable.execute(vec![val.clone()])?,
                Value::Lambda(_, _) => todo!(),
                _ => unreachable!(),
            };
            if IsTrue.inner_execute(&current_result) {
                result.push_back(val);
            }
        }
        Ok(Value::List(result))
    }
}

display_for_callable!(Filter);

#[derive(Debug, Clone)]
pub struct Reduce;

impl Callable for Reduce {
    fn name(&self) -> &'static str {
        "reduce"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 2 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(
                self.name(),
                "<function> <collection>",
            ))
        }
    }

    fn execute(&self, args: Vec<Value>) -> CallableResult {
        let mut args_iter = args.into_iter();
        let maybe_fn = args_iter.next().unwrap();
        let maybe_coll = args_iter.next().unwrap();

        let fn_value = match maybe_fn {
            Value::Callable(..) => Ok(maybe_fn),
            Value::Lambda(_, arity) => {
                if arity == 0 || arity == 2 {
                    Ok(maybe_fn)
                } else {
                    Err(RuntimeError::WrongArity(arity, args_iter.len()))
                }
            }
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a function",
                maybe_fn.type_str(),
            )),
        }?;

        let mut coll = VecDeque::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        let mut reduce_result = match coll.len() {
            0 => match fn_value {
                Value::Callable(callable) => return callable.execute(Vec::new()),
                Value::Lambda(_, arity) => {
                    return if arity == 0 {
                        todo!()
                    } else {
                        Err(RuntimeError::WrongArity(arity, args_iter.len()))
                    }
                }
                _ => unreachable!(),
            },
            1 => return Ok(coll.pop_front().unwrap()),
            _ => {
                let first_val = coll.pop_front().unwrap();
                let second_val = coll.pop_front().unwrap();
                match &fn_value {
                    Value::Callable(callable) => callable.execute(vec![first_val, second_val]),
                    Value::Lambda(_, arity) => {
                        if *arity == 2 {
                            todo!()
                        } else {
                            Err(RuntimeError::WrongArity(*arity, args_iter.len()))
                        }
                    }
                    _ => unreachable!(),
                }?
            }
        };

        for val in coll {
            reduce_result = match &fn_value {
                Value::Callable(callable) => callable.execute(vec![reduce_result, val]),
                Value::Lambda(_, _) => todo!(),
                _ => unreachable!(),
            }?;
        }

        Ok(reduce_result)
    }
}

display_for_callable!(Reduce);
