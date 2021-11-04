use num::Signed;

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    lispparser::NumberLiteralParser,
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

#[derive(Debug, Clone)]
pub struct NumberCast;

impl Callable for NumberCast {
    fn name(&self) -> &'static str {
        "num"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(self.name(), "<string>"))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let maybe_string = args.into_iter().next().unwrap();
        if let Value::String(string) = maybe_string {
            NumberLiteralParser::new()
                .parse(&string)
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

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
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

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(self.name(), "<string>"))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
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

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(self.name(), "<number>"))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
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
