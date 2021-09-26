use num::Signed;

use crate::{
    callables::{Callable, ExecutionResult},
    miniclj::NumberLiteralParser,
    Scope, Value,
};

use super::RuntimeError;

#[derive(Debug, Clone)]
struct NumberCast;

impl Callable for NumberCast {
    fn name(&self) -> &'static str {
        "num"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        if let Value::String(s) = &args[0] {
            match NumberLiteralParser::new().parse(s) {
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
                args[0].type_str(),
            ))
        }
    }
}

display_for_callable!(NumberCast);

#[derive(Debug, Clone)]
struct StringCast;

impl Callable for StringCast {
    fn name(&self) -> &'static str {
        "str"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        match args
            .iter()
            .map(|v| match v {
                Value::String(s) => Ok(String::from(s)),
                Value::Number(n) => Ok(format!("{}", n)),
                Value::Nil => Ok(String::from("nil")),
                _ => Err(v.type_str()),
            })
            .collect::<Result<String, &'static str>>()
        {
            Ok(s) => Ok(Value::String(s)),
            Err(v) => Err(RuntimeError::WrongArgument(
                self.name(),
                "a primitive value",
                v,
            )),
        }
    }
}

display_for_callable!(StringCast);

#[derive(Debug, Clone)]
struct Ord;

impl Callable for Ord {
    fn name(&self) -> &'static str {
        "ord"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        if let Value::String(s) = &args[0] {
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
                args[0].type_str(),
            ))
        }
    }
}

display_for_callable!(Ord);

#[derive(Debug, Clone)]
struct Chr;

impl Callable for Chr {
    fn name(&self) -> &'static str {
        "chr"
    }

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<number>");
        }
        if let Value::Number(n) = &args[0] {
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
                args[0].type_str(),
            ))
        }
    }
}

display_for_callable!(Chr);

#[cfg(test)]
mod tests {
    use num::Rational64;

    use super::*;

    fn s(s: &str) -> Value {
        Value::String(String::from(s))
    }

    fn n(n: i64, d: i64) -> Value {
        Value::Number(Rational64::new(n, d))
    }

    #[test]
    fn test_num() {
        let scope = Scope::new(None);
        assert_eq!(NumberCast.call(&[s("1234")], &scope).unwrap(), n(1234, 1));
        assert_eq!(
            NumberCast.call(&[s("-12.32")], &scope).unwrap(),
            n(-1232, 100)
        );
        assert!(NumberCast.call(&[s("1.1.1")], &scope).is_err());
        assert!(NumberCast.call(&[s("testing")], &scope).is_err());
    }

    #[test]
    fn test_str() {
        let scope = Scope::new(None);
        assert_eq!(StringCast.call(&[], &scope).unwrap(), s(""));
        assert_eq!(
            StringCast.call(&[s("testA"), s("testB")], &scope).unwrap(),
            s("testAtestB")
        );
        assert_eq!(
            StringCast
                .call(&[n(12, 1), s("str"), n(1, 100), Value::Nil], &scope)
                .unwrap(),
            s("12str1/100nil")
        );
    }

    #[test]
    fn test_chr_ord() {
        let scope = Scope::new(None);
        for chr in ["x", "0", "1", ",", "\""] {
            let val_str = Value::String(String::from(chr));
            let val_num = Ord.call(&[val_str.clone()], &scope).unwrap();
            assert_eq!(Chr.call(&[val_num], &scope).unwrap(), val_str);
        }
    }
}
