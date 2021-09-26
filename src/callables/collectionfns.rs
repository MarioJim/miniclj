use std::convert::TryFrom;

use num::{Signed, Zero};

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    Scope, Value,
};

#[derive(Debug, Clone)]
struct First;

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
struct Rest;

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
struct Cons;

impl Callable for Cons {
    fn name(&self) -> &'static str {
        "cons"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<new element> <sequence>");
        }
        let elem = args[0].clone();
        match &args[1] {
            Value::List(l) => Ok(l.cons(elem)),
            Value::Vector(v) => Ok(v.cons(elem)),
            Value::Set(s) => Ok(s.cons(elem)),
            Value::Map(m) => m.cons(elem),
            _ => Err(RuntimeError::WrongArgument(
                self.name(),
                "a collection",
                args[1].type_str(),
            )),
        }
    }
}

display_for_callable!(Cons);

#[derive(Debug, Clone)]
struct Get;

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
struct Len;

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
struct IsEmpty;

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
