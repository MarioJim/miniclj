use std::collections::VecDeque;

use crate::{
    callables::{conditionals::IsTrue, Callable},
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{List, RuntimeError, RuntimeResult, VMState, Value},
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
            Value::Lambda(..) => Ok(maybe_fn),
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a function",
                maybe_fn.type_str(),
            )),
        }?;

        let mut lists = args_iter
            .map(|arg| {
                List::try_from(arg).map_err(|type_str| {
                    RuntimeError::WrongDataType(self.name(), "a collection", type_str)
                })
            })
            .collect::<RuntimeResult<Vec<List>>>()?;
        let mut result_vec = VecDeque::new();
        loop {
            let mut args_for_callable = Vec::new();
            let mut next_lists = Vec::new();
            for list in lists {
                match list {
                    List::Cons(val, next_list) => {
                        args_for_callable.push(*val);
                        next_lists.push(*next_list);
                    }
                    List::EmptyList => return Ok(Value::List(result_vec.into_iter().collect())),
                }
            }
            lists = next_lists;
            let current_result = match &fn_value {
                Value::Callable(callable) => callable.execute(state, args_for_callable),
                Value::Lambda(ins_ptr, arity) => {
                    state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                }
                _ => unreachable!(),
            }?;
            result_vec.push_front(current_result);
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
                    Err(RuntimeError::WrongArityN("User defined callable", arity, 1))
                }
            }
            _ => Err(RuntimeError::WrongDataType(
                self.name(),
                "a function",
                maybe_fn.type_str(),
            )),
        }?;

        let mut list = List::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        let mut result_vec = VecDeque::new();
        while let List::Cons(next, rest) = list {
            let args_for_callable = vec![*next.clone()];
            let current_result = match &fn_value {
                Value::Callable(callable) => callable.execute(state, args_for_callable),
                Value::Lambda(ins_ptr, arity) => {
                    state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                }
                _ => unreachable!(),
            }?;
            if IsTrue.inner_execute(&current_result) {
                result_vec.push_front(*next);
            }
            list = *rest;
        }
        Ok(Value::List(result_vec.into_iter().collect()))
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
                    Err(RuntimeError::WrongArityN(
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

        let coll = List::try_from(maybe_coll).map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a collection", type_str)
        })?;

        match coll {
            List::EmptyList => match fn_value {
                Value::Callable(callable) => callable.execute(state, Vec::new()),
                Value::Lambda(ins_ptr, arity) => state.execute_lambda(ins_ptr, arity, Vec::new()),
                _ => unreachable!(),
            },
            List::Cons(first, rest) => match *rest {
                List::EmptyList => Ok(*first),
                List::Cons(second, rest) => {
                    let args_for_callable = vec![*first, *second];
                    let mut reduce_result = match &fn_value {
                        Value::Callable(callable) => callable.execute(state, args_for_callable),
                        Value::Lambda(ins_ptr, arity) => {
                            state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                        }
                        _ => unreachable!(),
                    }?;

                    let mut list = *rest;
                    while let List::Cons(next, rest) = list {
                        let args_for_callable = vec![reduce_result, *next];
                        reduce_result = match &fn_value {
                            Value::Callable(callable) => callable.execute(state, args_for_callable),
                            Value::Lambda(ins_ptr, arity) => {
                                state.execute_lambda(*ins_ptr, *arity, args_for_callable)
                            }
                            _ => unreachable!(),
                        }?;
                        list = *rest;
                    }

                    Ok(reduce_result)
                }
            },
        }
    }
}

display_for_callable!(Reduce);
