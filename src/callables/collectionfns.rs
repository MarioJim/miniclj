use std::convert::{TryFrom, TryInto};

use num::{Signed, Zero};

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    value::{list::List, ValueIterator},
    Scope, Value,
};

#[derive(Debug, Clone)]
pub struct First;

impl Callable for First {
    fn name(&self) -> &'static str {
        "first"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<sequence>");
        }
        let coll = args.iter().next().unwrap();
        let get_args = &[coll.clone(), Value::from(0)];
        Get.call(get_args, scope)
    }
}

display_for_callable!(First);

#[derive(Debug, Clone)]
pub struct Rest;

impl Callable for Rest {
    fn name(&self) -> &'static str {
        "rest"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<sequence>");
        }
        match &args[0] {
            Value::List(l) => Ok(l.rest()),
            Value::Vector(v) => Ok(v.rest()),
            _ => Err(RuntimeError::WrongArgument(
                self.name(),
                "a sequence",
                args[0].type_str(),
            )),
        }
    }
}

display_for_callable!(Rest);

#[derive(Debug, Clone)]
pub struct Cons;

impl Callable for Cons {
    fn name(&self) -> &'static str {
        "cons"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<value> <collection>");
        }
        let coll_iter = ValueIterator::try_from(args[1].clone()).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", args[1].type_str())
        })?;
        let mut coll_as_list: List = coll_iter.collect();
        coll_as_list.push_front(args[0].clone());
        Ok(Value::List(coll_as_list))
    }
}

display_for_callable!(Cons);

#[derive(Debug, Clone)]
pub struct Conj;

impl Callable for Conj {
    fn name(&self) -> &'static str {
        "conj"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<value> <sequence>");
        }
        let val = args[0].clone();
        match &args[1] {
            Value::List(l) => {
                let mut cloned_list = l.clone();
                cloned_list.push_front(val);
                Ok(Value::List(cloned_list))
            }
            Value::Vector(v) => {
                let mut cloned_vector = v.clone();
                cloned_vector.push(val);
                Ok(Value::Vector(cloned_vector))
            }
            Value::Set(s) => {
                let mut cloned_set = s.clone();
                cloned_set.insert(val);
                Ok(Value::Set(cloned_set))
            }
            Value::Map(m) => match val {
                Value::Vector(v) if v.len() == 2 => {
                    let (key, value) = v.try_into().unwrap();
                    let mut cloned_map = m.clone();
                    cloned_map.insert(key, value);
                    Ok(Value::Map(cloned_map))
                }
                _ => Err(RuntimeError::Error(String::from(
                    "Only vectors with two elements (key-value pair) can be added to a map",
                ))),
            },
            _ => Err(RuntimeError::WrongArgument(
                self.name(),
                "a collection",
                args[1].type_str(),
            )),
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

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<collection> <key>");
        }
        match &args[1] {
            Value::List(l) => l.get(&args[0]),
            Value::Vector(v) => v.get(&args[0]),
            Value::Set(s) => s.get(&args[0]),
            Value::Map(m) => m.get(&args[0]),
            Value::String(s) => {
                if let Value::Number(n) = &args[0] {
                    if n.is_integer() && !n.is_negative() {
                        Ok(s.chars()
                            .nth(usize::try_from(*n.numer()).unwrap())
                            .map(|chr| Value::String(String::from(chr)))
                            .unwrap_or(Value::Nil))
                    } else {
                        Err(RuntimeError::Error(format!(
                            "String can only be indexed by positive integers, not by {}",
                            n
                        )))
                    }
                } else {
                    Err(RuntimeError::Error(format!(
                        "String can't be indexed by {}",
                        args[0]
                    )))
                }
            }
            _ => Err(RuntimeError::WrongArgument(
                self.name(),
                "a collection",
                args[1].type_str(),
            )),
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

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let len = match &args[0] {
            Value::List(l) => l.len(),
            Value::Vector(v) => v.len(),
            Value::Set(s) => s.len(),
            Value::Map(m) => m.len(),
            Value::String(s) => s.len(),
            _ => {
                return Err(RuntimeError::WrongArgument(
                    self.name(),
                    "a collection",
                    args[0].type_str(),
                ))
            }
        };
        Ok(Value::from(i64::try_from(len).unwrap()))
    }
}

display_for_callable!(Len);

#[derive(Debug, Clone)]
pub struct IsEmpty;

impl Callable for IsEmpty {
    fn name(&self) -> &'static str {
        "empty?"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let len = Len.call(args, scope);
        if let Ok(Value::Number(n)) = len {
            Ok(Value::from(n.is_zero()))
        } else {
            len
        }
    }
}

display_for_callable!(IsEmpty);
