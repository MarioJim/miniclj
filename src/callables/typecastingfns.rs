use num::{Rational64, Signed};

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
            return Err(RuntimeError::ArityError(self.name(), "<string>"));
        }
        if let Value::String(s) = &args[0] {
            match NumberLiteralParser::new().parse(s) {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Err(RuntimeError::GenericError(format!(
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
        Ok(Value::String(
            args.iter().map(|v| format!("{}", v)).collect(),
        ))
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
            return Err(RuntimeError::ArityError(self.name(), "<string>"));
        }
        if let Value::String(s) = &args[0] {
            match s.chars().next() {
                Some(c) => Ok(Value::Number(Rational64::from_integer(c as i64))),
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
            return Err(RuntimeError::ArityError(self.name(), "<number>"));
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
                    None => Err(RuntimeError::GenericError(format!(
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
