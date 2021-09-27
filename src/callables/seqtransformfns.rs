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

        let maybe_fn = args[0].eval(scope)?;
        let function = if let Value::Fn(function) = maybe_fn {
            function
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a function",
                args[0].type_str(),
            ));
        };

        let maybe_coll = args[1].eval(scope)?;
        let maybe_coll_type = maybe_coll.type_str();
        let list = ValueIterator::try_from(maybe_coll)
            .map_err(|_| RuntimeError::WrongArgument(self.name(), "a collection", maybe_coll_type))?
            .map(|v| function.call(&[v], scope))
            .collect::<Result<_, RuntimeError>>()?;
        Ok(Value::List(list))
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

        let maybe_fn = args[0].eval(scope)?;
        let function = if let Value::Fn(function) = maybe_fn {
            function
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a function",
                maybe_fn.type_str(),
            ));
        };

        let maybe_coll = args[1].eval(scope)?;
        let maybe_coll_type = maybe_coll.type_str();
        let coll_iter = ValueIterator::try_from(maybe_coll).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", maybe_coll_type)
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
pub struct Reduce;

impl Callable for Reduce {
    fn name(&self) -> &'static str {
        "reduce"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<function> <collection>");
        }

        let maybe_fn = args[0].eval(scope)?;
        let function = if let Value::Fn(function) = maybe_fn {
            function
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a function",
                maybe_fn.type_str(),
            ));
        };

        let maybe_coll = args[1].eval(scope)?;
        let maybe_coll_type = maybe_coll.type_str();
        let mut coll_iter = ValueIterator::try_from(maybe_coll).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", maybe_coll_type)
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
