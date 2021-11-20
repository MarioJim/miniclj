use num::{Rational64, Zero};

use crate::callables::prelude::*;

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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        match (self, num_args) {
            (FactorOp::Sub | FactorOp::Div, 0) => Err(CompilationError::EmptyArgs(self.name())),
            _ => Ok(()),
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(*self)))
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
                0 => Err(RuntimeError::WrongArityS(
                    self.name(),
                    "at least one number",
                    0,
                )),
                1 => Ok(Value::Number(-nums.next().unwrap())),
                _ => {
                    let positive = nums.next().unwrap();
                    let negative = nums.fold(zero, |a, b| a + b);
                    Ok(Value::Number(positive - negative))
                }
            },
            FactorOp::Mul => Ok(Value::Number(nums.fold(one, |a, b| a * b))),
            FactorOp::Div => match nums.len() {
                0 => Err(RuntimeError::WrongArityS(
                    self.name(),
                    "at least one number",
                    0,
                )),
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn v(n: i64) -> Value {
        Value::from(n)
    }

    #[test]
    fn test_add() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert_eq!(FactorOp::Add.execute(&vm, vec![]).unwrap(), v(0));
        assert_eq!(FactorOp::Add.execute(&vm, vec![v(2)]).unwrap(), v(2));
        assert_eq!(
            FactorOp::Add
                .execute(&vm, vec![v(2), v(5), v(6), v(-3)])
                .unwrap(),
            v(10)
        );
    }

    #[test]
    fn test_sub() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert!(matches!(
            FactorOp::Sub.execute(&vm, vec![]),
            Err(RuntimeError::WrongArityS(..))
        ));
        assert_eq!(FactorOp::Sub.execute(&vm, vec![v(2)]).unwrap(), v(-2));
        assert_eq!(
            FactorOp::Sub
                .execute(&vm, vec![v(2), v(5), v(6), v(-3)])
                .unwrap(),
            v(-6)
        );
    }

    #[test]
    fn test_mul() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert_eq!(FactorOp::Mul.execute(&vm, vec![]).unwrap(), v(1));
        assert_eq!(FactorOp::Mul.execute(&vm, vec![v(2)]).unwrap(), v(2));
        assert_eq!(
            FactorOp::Mul
                .execute(&vm, vec![v(2), v(5), v(6), v(-3)])
                .unwrap(),
            v(-180)
        );
    }

    #[test]
    fn test_div() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        let f = |num, den| Value::Number(Rational64::new(num, den));
        assert!(matches!(
            FactorOp::Div.execute(&vm, vec![]),
            Err(RuntimeError::WrongArityS(..))
        ));
        assert_eq!(FactorOp::Div.execute(&vm, vec![v(2)]).unwrap(), f(1, 2));
        assert_eq!(
            FactorOp::Div
                .execute(&vm, vec![v(2), v(5), v(6), v(-3)])
                .unwrap(),
            f(-2, 90)
        );
        assert!(matches!(
            FactorOp::Div.execute(&vm, vec![v(2), v(3), v(0)]),
            Err(RuntimeError::DivisionByZero)
        ));
    }
}
