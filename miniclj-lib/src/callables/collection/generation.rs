use crate::callables::prelude::*;

#[derive(Debug, Clone)]
pub struct Range;

impl Callable for Range {
    fn name(&self) -> &'static str {
        "range"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if (1..=3).contains(&num_args) {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<stop num>) or (range <start> <stop>) or (range <start> <stop> <step>",
            ))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.is_empty() {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "at least one number",
                args.len(),
            ));
        }

        let mut args_iter = args.into_iter();

        let start = if args_iter.len() == 1 {
            0
        } else {
            args_iter.next().unwrap().as_usize().map_err(|type_str| {
                RuntimeError::WrongDataType(self.name(), "a positive number", type_str)
            })?
        };
        let stop = args_iter.next().unwrap().as_usize().map_err(|type_str| {
            RuntimeError::WrongDataType(self.name(), "a positive number", type_str)
        })?;
        let step = match args_iter.next() {
            Some(value) => value.as_usize().map_err(|type_str| {
                RuntimeError::WrongDataType(self.name(), "a positive number", type_str)
            }),
            None => Ok(1),
        }?;

        let mut list = Vec::new();
        let mut next_val = start;
        while next_val < stop {
            list.push(next_val);
            next_val += step;
        }

        Ok(Value::List(
            list.into_iter()
                .rev()
                .map(|n| Value::from(n as i64))
                .collect(),
        ))
    }
}

display_for_callable!(Range);
