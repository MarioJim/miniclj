use num::Rational64;

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl Callable for ComparisonOp {
    fn name(&self) -> &'static str {
        match self {
            ComparisonOp::Eq => "=",
            ComparisonOp::Ne => "!=",
            ComparisonOp::Gt => ">",
            ComparisonOp::Lt => "<",
            ComparisonOp::Ge => ">=",
            ComparisonOp::Le => "<=",
        }
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 0 {
            Err(CompilationError::EmptyArgs(self.name()))
        } else {
            Ok(state.get_callable_addr(Box::new(*self)))
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let args_as_nums = |args: Vec<Value>| {
            args.into_iter()
                .map(|v| {
                    if let Value::Number(n) = v {
                        Ok(n)
                    } else {
                        Err(RuntimeError::WrongDataType(
                            self.name(),
                            "a number",
                            v.type_str(),
                        ))
                    }
                })
                .collect::<RuntimeResult<Vec<Rational64>>>()
        };

        Ok(Value::from(match self {
            ComparisonOp::Eq => args.iter().all(|v| v == &args[0]),
            ComparisonOp::Ne => args.iter().any(|v| v != &args[0]),
            ComparisonOp::Gt => args_as_nums(args)?.windows(2).all(|w| w[0] > w[1]),
            ComparisonOp::Lt => args_as_nums(args)?.windows(2).all(|w| w[0] < w[1]),
            ComparisonOp::Ge => args_as_nums(args)?.windows(2).all(|w| w[0] >= w[1]),
            ComparisonOp::Le => args_as_nums(args)?.windows(2).all(|w| w[0] <= w[1]),
        }))
    }
}

display_for_callable!(ComparisonOp);
