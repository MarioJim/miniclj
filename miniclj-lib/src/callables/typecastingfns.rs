use num::Signed;

use crate::{callables::prelude::*, parsers::NumberLiteralParser};

#[derive(Debug, Clone)]
pub struct NumberCast;

impl Callable for NumberCast {
    fn name(&self) -> &'static str {
        "num"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<string>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let maybe_string = args.into_iter().next().unwrap();
        if let Value::String(string) = maybe_string {
            NumberLiteralParser::parse(&string)
                .map(Value::Number)
                .map_err(|_| RuntimeError::CouldntParse(format!("\"{}\"", string), "a number"))
        } else {
            Err(RuntimeError::WrongDataType(
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

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let formatted_args = args
            .into_iter()
            .map(|value| {
                if value == Value::Nil {
                    String::new()
                } else {
                    format!("{}", value)
                }
            })
            .collect::<String>();
        println!("from str {}", formatted_args);
        Ok(Value::String(formatted_args))
    }
}

display_for_callable!(StringCast);

#[derive(Debug, Clone)]
pub struct Ord;

impl Callable for Ord {
    fn name(&self) -> &'static str {
        "ord"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<string>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a string",
                args.len(),
            ));
        }

        let maybe_string = args.into_iter().next().unwrap();
        if let Value::String(s) = maybe_string {
            match s.chars().next() {
                Some(c) => Ok(Value::from(c as i64)),
                None => Err(RuntimeError::WrongDataType(
                    self.name(),
                    "a string with at least one character",
                    "an empty string",
                )),
            }
        } else {
            Err(RuntimeError::WrongDataType(
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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<number>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "a number",
                args.len(),
            ));
        }

        let maybe_num = args.into_iter().next().unwrap();
        if let Value::Number(n) = maybe_num {
            if !n.is_integer() || n.is_negative() {
                Err(RuntimeError::WrongDataType(
                    self.name(),
                    "a positive integer",
                    "a decimal or negative integer",
                ))
            } else {
                match char::from_u32(*n.numer() as u32) {
                    Some(c) => Ok(Value::String(String::from(c))),
                    None => Err(RuntimeError::CouldntParse(
                        format!("{}", n.numer()),
                        "a character",
                    )),
                }
            }
        } else {
            Err(RuntimeError::WrongDataType(
                self.name(),
                "a number",
                maybe_num.type_str(),
            ))
        }
    }
}

display_for_callable!(Chr);
