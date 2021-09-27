use std::{convert::TryFrom, slice};

use crate::{
    callables::{conditionals::IsTrue, Callable, ExecutionResult, RuntimeError},
    value::{list::List, ValueIterator},
    Scope, Value,
};

#[derive(Debug, Clone)]
pub struct Map;

impl Callable for Map {
    fn name(&self) -> &'static str {
        "map"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<function> <collection>");
        }

        let function = if let Value::Fn(function) = &args[0] {
            function
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a function",
                args[0].type_str(),
            ));
        };

        Ok(Value::List(
            ValueIterator::try_from(args[1].clone())
                .map_err(|_| {
                    RuntimeError::WrongArgument(self.name(), "a collection", args[1].type_str())
                })?
                .map(|v| function.call(&[v], scope))
                .collect::<Result<_, RuntimeError>>()?,
        ))
    }
}

display_for_callable!(Map);

#[derive(Debug, Clone)]
pub struct Filter;

impl Callable for Filter {
    fn name(&self) -> &'static str {
        "filter"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<function> <collection>");
        }

        let function = if let Value::Fn(function) = &args[0] {
            function
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a function",
                args[0].type_str(),
            ));
        };

        let coll_iter = ValueIterator::try_from(args[1].clone()).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", args[1].type_str())
        })?;

        let mut filtered_list = List::default();
        for val in coll_iter {
            let keep = function.call(slice::from_ref(&val), scope)?;
            if IsTrue.inner_call(&keep) {
                filtered_list.push_front(val.clone());
            }
        }
        Ok(Value::List(filtered_list))
    }
}

display_for_callable!(Filter);

#[derive(Debug, Clone)]
struct Reduce;

impl Callable for Reduce {
    fn name(&self) -> &'static str {
        "reduce"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<function> <collection>");
        }

        let function = if let Value::Fn(function) = &args[0] {
            function
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a function",
                args[0].type_str(),
            ));
        };

        let mut coll_iter = ValueIterator::try_from(args[1].clone()).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", args[1].type_str())
        })?;

        let mut reduce_result = match coll_iter.next() {
            Some(v) => v,
            None => return function.call(&[], scope),
        };

        for value in coll_iter {
            reduce_result = function.call(&[reduce_result, value], scope)?;
        }
        Ok(reduce_result)
    }
}

display_for_callable!(Reduce);
