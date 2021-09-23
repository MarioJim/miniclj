use std::{
    convert::TryInto,
    fmt::{self, Display},
};

use num::{Rational64, Signed, Zero};

use crate::{callables::Callable, value::Value};

#[derive(Debug, Clone)]
struct Cons;

impl Display for Cons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cons")
    }
}

impl Callable for Cons {
    fn call(&self, args: &[Value]) -> Value {
        if args.len() != 2 {
            return Value::Error(String::from(
                "cons called with wrong number of arguments, should be <new element> <collection>",
            ));
        }
        let elem = args[0].clone();
        match &args[1] {
            Value::List(l) => l.cons(elem),
            Value::Vector(v) => v.cons(elem),
            Value::Set(s) => s.cons(elem),
            Value::Map(m) => m.cons(elem),
            _ => Value::Error(String::from(
                "Second argument of call to cons isn't a collection",
            )),
        }
    }
}

#[derive(Debug, Clone)]
struct Get;

impl Display for Get {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "get")
    }
}

impl Callable for Get {
    fn call(&self, args: &[Value]) -> Value {
        if args.len() != 2 {
            return Value::Error(String::from(
                "get called with wrong number of arguments, should be <collection> <key>",
            ));
        }
        match &args[1] {
            Value::List(l) => l.get(&args[0]),
            Value::Vector(v) => v.get(&args[0]),
            Value::Set(s) => s.get(&args[0]),
            Value::Map(m) => m.get(&args[0]),
            Value::String(s) => {
                if let Value::Number(n) = &args[0] {
                    if n.is_integer() && !n.is_negative() {
                        s.chars()
                            .nth((*n.numer()).try_into().unwrap())
                            .map(|chr| Value::String(String::from(chr)))
                            .unwrap_or(Value::Nil)
                    } else {
                        Value::Error(format!(
                            "String can only be indexed by positive integers, not by {}",
                            n
                        ))
                    }
                } else {
                    Value::Error(format!("String can't be indexed by {}", args[0]))
                }
            }
            _ => Value::Error(String::from(
                "Second argument of call to get isn't a collection",
            )),
        }
    }
}

#[derive(Debug, Clone)]
struct Len;

impl Display for Len {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "len")
    }
}

impl Callable for Len {
    fn call(&self, args: &[Value]) -> Value {
        if args.len() != 1 {
            return Value::Error(String::from(
                "len called with wrong number of arguments, should be <collection>",
            ));
        }
        let len = match &args[0] {
            Value::List(l) => l.len(),
            Value::Vector(v) => v.len(),
            Value::Set(s) => s.len(),
            Value::Map(m) => m.len(),
            Value::String(s) => s.len(),
            _ => {
                return Value::Error(String::from(
                    "Second argument of call to len isn't a collection",
                ))
            }
        };
        Value::Number(Rational64::from_integer(len.try_into().unwrap()))
    }
}

#[derive(Debug, Clone)]
struct IsEmpty;

impl Display for IsEmpty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "empty?")
    }
}

impl Callable for IsEmpty {
    fn call(&self, args: &[Value]) -> Value {
        if args.len() != 1 {
            return Value::Error(String::from(
                "empty? called with wrong number of arguments, should be <collection>",
            ));
        }
        let len = Len.call(args);
        if let Value::Number(n) = len {
            if n.is_zero() {
                Value::Number(Rational64::from_integer(1))
            } else {
                Value::Number(Rational64::from_integer(0))
            }
        } else {
            len
        }
    }
}
