use std::{
    convert::{TryFrom, TryInto},
    rc::Rc,
};

use num::{Signed, Zero};

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    value::list::List,
    SExpr, Scope, Value,
};

#[derive(Debug, Clone)]
pub struct First;

impl Callable for First {
    fn name(&self) -> &'static str {
        "first"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let maybe_coll = args.into_iter().next().unwrap().eval(scope)?;
        let maybe_coll_type = maybe_coll.type_str();
        let mut coll_as_list = List::try_from(maybe_coll).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", maybe_coll_type)
        })?;
        Ok(coll_as_list.pop_front().unwrap_or(Value::Nil))
    }
}

display_for_callable!(First);

#[derive(Debug, Clone)]
pub struct Rest;

impl Callable for Rest {
    fn name(&self) -> &'static str {
        "rest"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let maybe_coll = args.into_iter().next().unwrap().eval(scope)?;
        let maybe_coll_type = maybe_coll.type_str();
        let mut coll_as_list = List::try_from(maybe_coll).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", maybe_coll_type)
        })?;
        coll_as_list.pop_front();
        Ok(Value::List(coll_as_list))
    }
}

display_for_callable!(Rest);

#[derive(Debug, Clone)]
pub struct Cons;

impl Callable for Cons {
    fn name(&self) -> &'static str {
        "cons"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<value> <collection>");
        }
        let mut args_iter = args.into_iter();
        let maybe_value = args_iter.next().unwrap();
        let maybe_coll = args_iter.next().unwrap().eval(scope)?;
        let maybe_coll_type = maybe_coll.type_str();
        let mut coll_as_list = List::try_from(maybe_coll).map_err(|_| {
            RuntimeError::WrongArgument(self.name(), "a collection", maybe_coll_type)
        })?;
        coll_as_list.push_front(maybe_value.eval(scope)?);
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

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<value> <collection>");
        }
        let mut args_iter = args.into_iter();
        let val = args_iter.next().unwrap().eval(scope)?;
        let maybe_collection = args_iter.next().unwrap().eval(scope)?;
        match maybe_collection {
            Value::List(mut list) => {
                list.push_front(val);
                Ok(Value::List(list))
            }
            Value::Vector(mut vector) => {
                vector.push(val);
                Ok(Value::Vector(vector))
            }
            Value::Set(mut set) => {
                set.insert(val);
                Ok(Value::Set(set))
            }
            Value::Map(mut map) => match val {
                Value::Vector(v) if v.len() == 2 => {
                    let (key, value) = v.try_into().unwrap();
                    map.insert(key, value);
                    Ok(Value::Map(map))
                }
                _ => Err(RuntimeError::Error(String::from(
                    "Only vectors with two elements (key-value pair) can be added to a map",
                ))),
            },
            _ => Err(RuntimeError::WrongArgument(
                self.name(),
                "a collection",
                maybe_collection.type_str(),
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

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<collection> <key>");
        }
        let mut args_iter = args.into_iter();
        let coll = args_iter.next().unwrap().eval(scope)?;
        let key = args_iter.next().unwrap().eval(scope)?;
        match coll {
            Value::List(l) => l.get(&key),
            Value::Vector(v) => v.get(&key),
            Value::Set(s) => s.get(&key),
            Value::Map(m) => m.get(&key),
            Value::String(s) => {
                if let Value::Number(n) = &key {
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
                        key
                    )))
                }
            }
            _ => Err(RuntimeError::WrongArgument(
                self.name(),
                "a collection",
                coll.type_str(),
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

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let coll = args.into_iter().next().unwrap().eval(scope)?;
        let len = match coll {
            Value::List(l) => l.len(),
            Value::Vector(v) => v.len(),
            Value::Set(s) => s.len(),
            Value::Map(m) => m.len(),
            Value::String(s) => s.len(),
            _ => {
                return Err(RuntimeError::WrongArgument(
                    self.name(),
                    "a collection",
                    coll.type_str(),
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

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        if let Value::Number(n) = Len.call(args, scope)? {
            Ok(Value::from(n.is_zero()))
        } else {
            unreachable!("Call to len returned something that isn't a number")
        }
    }
}

display_for_callable!(IsEmpty);
