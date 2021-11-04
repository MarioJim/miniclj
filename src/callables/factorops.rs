use num::{Rational64, Zero};

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FactorOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Callable for FactorOp {
    fn name(&self) -> &'static str {
        match self {
            FactorOp::Add => "+",
            FactorOp::Sub => "-",
            FactorOp::Mul => "*",
            FactorOp::Div => "/",
        }
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        match (self, num_args) {
            (FactorOp::Sub, 0) | (FactorOp::Div, 0) => {
                Err(CompilationError::EmptyArgs(self.name()))
            }
            _ => Ok(state.get_callable_addr(Box::new(*self))),
        }
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let one = Rational64::from_integer(1);
        let zero = Rational64::from_integer(0);

        let mut nums = args
            .into_iter()
            .map(|value| match value {
                Value::Number(n) => Ok(n),
                _ => Err(RuntimeError::WrongDataType(
                    self.name(),
                    "a number",
                    value.type_str(),
                )),
            })
            .collect::<RuntimeResult<Vec<Rational64>>>()?
            .into_iter();

        match self {
            FactorOp::Add => Ok(Value::Number(nums.fold(zero, |a, b| a + b))),
            FactorOp::Sub => match nums.len() {
                0 => unreachable!(),
                1 => Ok(Value::Number(-nums.next().unwrap())),
                _ => {
                    let positive = nums.next().unwrap();
                    let negative = nums.fold(zero, |a, b| a + b);
                    Ok(Value::Number(positive - negative))
                }
            },
            FactorOp::Mul => Ok(Value::Number(nums.fold(one, |a, b| a * b))),
            FactorOp::Div => match nums.len() {
                0 => unreachable!(),
                1 => Ok(Value::Number(nums.next().unwrap().recip())),
                _ => {
                    let numerator = nums.next().unwrap();
                    let denominator = nums.fold(one, |a, b| a * b);
                    if denominator.is_zero() {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(numerator / denominator))
                    }
                }
            },
        }
    }
}

display_for_callable!(FactorOp);
