use std::rc::Rc;

use num::Signed;

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    parser::NumberLiteralParser,
    SExpr, Scope, Value,
};

#[derive(Debug, Clone)]
pub struct NumberCast;

impl Callable for NumberCast {
    fn name(&self) -> &'static str {
        "num"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        let maybe_string = args.into_iter().next().unwrap().eval(scope)?;
        if let Value::String(s) = maybe_string {
            match NumberLiteralParser::new().parse(&s) {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Err(RuntimeError::Error(format!(
                    "The string \"{}\" couldn't be parsed as a number",
                    s
                ))),
            }
        } else {
            Err(RuntimeError::WrongArgument(
                self.name(),
                "a string",
                maybe_string.type_str(),
            ))
        }
    }
}

display_for_callable!(NumberCast);

#[derive(Debug, Clone)]
pub struct StringCast;

impl Callable for StringCast {
    fn name(&self) -> &'static str {
        "str"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        args.into_iter()
            .map(|sexpr| {
                let evaled_value = sexpr.eval(scope)?;
                match evaled_value {
                    Value::String(s) => Ok(s),
                    Value::Number(n) => Ok(n.to_string()),
                    Value::Nil => Ok(String::new()),
                    _ => Err(RuntimeError::WrongArgument(
                        self.name(),
                        "a primitive value",
                        evaled_value.type_str(),
                    )),
                }
            })
            .collect::<Result<String, RuntimeError>>()
            .map(Value::String)
    }
}

display_for_callable!(StringCast);

#[derive(Debug, Clone)]
pub struct Ord;

impl Callable for Ord {
    fn name(&self) -> &'static str {
        "ord"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        let maybe_string = args.into_iter().next().unwrap().eval(scope)?;
        if let Value::String(s) = maybe_string {
            match s.chars().next() {
                Some(c) => Ok(Value::from(c as i64)),
                None => Err(RuntimeError::WrongArgument(
                    self.name(),
                    "a string with at least one character",
                    "an empty string",
                )),
            }
        } else {
            Err(RuntimeError::WrongArgument(
                self.name(),
                "a string",
                maybe_string.type_str(),
            ))
        }
    }
}

display_for_callable!(Ord);

#[derive(Debug, Clone)]
pub struct Chr;

impl Callable for Chr {
    fn name(&self) -> &'static str {
        "chr"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<number>");
        }
        let maybe_num = args.into_iter().next().unwrap().eval(scope)?;
        if let Value::Number(n) = maybe_num {
            if !n.is_integer() || n.is_negative() {
                Err(RuntimeError::WrongArgument(
                    self.name(),
                    "a positive integer",
                    "a decimal or negative integer",
                ))
            } else {
                match char::from_u32(*n.numer() as u32) {
                    Some(c) => Ok(Value::String(String::from(c))),
                    None => Err(RuntimeError::Error(format!(
                        "{} couldn't convert the number {} to a valid character",
                        self.name(),
                        n.numer()
                    ))),
                }
            }
        } else {
            Err(RuntimeError::WrongArgument(
                self.name(),
                "a number",
                maybe_num.type_str(),
            ))
        }
    }
}

display_for_callable!(Chr);

#[cfg(test)]
mod tests {
    use num::Rational64;

    use super::*;

    fn sv(s: &str) -> Value {
        Value::String(String::from(s))
    }

    fn ss(s: &str) -> SExpr {
        SExpr::Value(sv(s))
    }

    fn nv(n: i64, d: i64) -> Value {
        Value::Number(Rational64::new(n, d))
    }

    fn ns(n: i64, d: i64) -> SExpr {
        SExpr::Value(nv(n, d))
    }

    #[test]
    fn test_num() {
        let scope = Rc::new(Scope::new(None));
        assert_eq!(
            NumberCast.call(vec![ss("1234")], &scope).unwrap(),
            nv(1234, 1)
        );
        assert_eq!(
            NumberCast.call(vec![ss("-12.32")], &scope).unwrap(),
            nv(-1232, 100)
        );
        assert!(NumberCast.call(vec![ss("1.1.1")], &scope).is_err());
        assert!(NumberCast.call(vec![ss("testing")], &scope).is_err());
    }

    #[test]
    fn test_str() {
        let scope = Rc::new(Scope::new(None));
        assert_eq!(StringCast.call(vec![], &scope).unwrap(), sv(""));
        assert_eq!(
            StringCast
                .call(vec![ss("testA"), ss("testB")], &scope)
                .unwrap(),
            sv("testAtestB")
        );
        assert_eq!(
            StringCast
                .call(
                    vec![ns(12, 1), ss("str"), ns(1, 100), SExpr::Value(Value::Nil)],
                    &scope
                )
                .unwrap(),
            sv("12str1/100")
        );
    }

    #[test]
    fn test_chr_ord() {
        let scope = Rc::new(Scope::new(None));
        for chr in ["x", "0", "1", ",", "\""] {
            let val_str = Value::String(String::from(chr));
            let val_num = Ord
                .call(vec![SExpr::Value(val_str.clone())], &scope)
                .unwrap();
            assert_eq!(
                Chr.call(vec![SExpr::Value(val_num)], &scope).unwrap(),
                val_str
            );
        }
    }
}
