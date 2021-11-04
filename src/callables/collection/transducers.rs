use std::collections::VecDeque;

use crate::{
    callables::{conditionals::IsTrue, Callable},
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{RuntimeError, RuntimeResult, VMState, Value},
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
            Err(CompilationError::WrongArity(
                self.name(),
                "<function> <...collections>",
            ))
        }
    }

    fn execute(&self, state: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        let maybe_fn = args_iter.next().unwrap();
        let fn_value = match maybe_fn {
            Value::Callable(..) => Ok(maybe_fn),
            Value::Lambda(_, arity) => {
                if arity == args_iter.len() {
                    Ok(maybe_fn)
                } else {
                    Err(RuntimeError::WrongArity(
                        "User defined callable",
                        arity,
                        args_iter.len(),
                    ))
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
                Value::Callable(callable) => callable.execute(state, args_for_callable),
                Value::Lambda(ins_ptr, arity) => {
                    state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                }
                _ => unreachable!(),
            }?;
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
            Err(CompilationError::WrongArity(
                self.name(),
                "<function> <collection>",
            ))
        }
    }

    fn execute(&self, state: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        let maybe_fn = args_iter.next().unwrap();
        let maybe_coll = args_iter.next().unwrap();

        let fn_value = match maybe_fn {
            Value::Callable(..) => Ok(maybe_fn),
            Value::Lambda(_, arity) => {
                if arity == 1 {
                    Ok(maybe_fn)
                } else {
                    Err(RuntimeError::WrongArity("User defined callable", arity, 1))
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
            let args_for_callable = vec![val.clone()];
            let current_result = match &fn_value {
                Value::Callable(callable) => callable.execute(state, args_for_callable),
                Value::Lambda(ins_ptr, arity) => {
                    state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                }
                _ => unreachable!(),
            }?;
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
            Err(CompilationError::WrongArity(
                self.name(),
                "<function> <collection>",
            ))
        }
    }

    fn execute(&self, state: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut args_iter = args.into_iter();
        let maybe_fn = args_iter.next().unwrap();
        let maybe_coll = args_iter.next().unwrap();

        let fn_value = match maybe_fn {
            Value::Callable(..) => Ok(maybe_fn),
            Value::Lambda(_, arity) => {
                if arity == 0 || arity == 2 {
                    Ok(maybe_fn)
                } else {
                    Err(RuntimeError::WrongArity(
                        "User defined callable",
                        arity,
                        args_iter.len(),
                    ))
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
                Value::Callable(callable) => return callable.execute(state, Vec::new()),
                Value::Lambda(ins_ptr, arity) => {
                    return if arity == 0 {
                        state.execute_lambda(ins_ptr, arity, Vec::new())
                    } else {
                        Err(RuntimeError::WrongArity(
                            "User defined callable",
                            arity,
                            args_iter.len(),
                        ))
                    }
                }
                _ => unreachable!(),
            },
            1 => return Ok(coll.pop_front().unwrap()),
            _ => {
                let first_val = coll.pop_front().unwrap();
                let second_val = coll.pop_front().unwrap();
                let args_for_callable = vec![first_val, second_val];
                match &fn_value {
                    Value::Callable(callable) => callable.execute(state, args_for_callable),
                    Value::Lambda(ins_ptr, arity) => {
                        if *arity == 2 {
                            state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                        } else {
                            Err(RuntimeError::WrongArity(
                                "User defined callable",
                                *arity,
                                args_iter.len(),
                            ))
                        }
                    }
                    _ => unreachable!(),
                }?
            }
        };

        for val in coll {
            let args_for_callable = vec![reduce_result, val];
            reduce_result = match &fn_value {
                Value::Callable(callable) => callable.execute(state, args_for_callable),
                Value::Lambda(ins_ptr, arity) => {
                    state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                }
                _ => unreachable!(),
            }?;
        }

        Ok(reduce_result)
    }
}

display_for_callable!(Reduce);
