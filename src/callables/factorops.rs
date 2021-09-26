use num::{Rational64, Zero};

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    Scope, Value,
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

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        let one = Rational64::from_integer(1);
        let zero = Rational64::from_integer(0);
        let maybe_nums = args
            .iter()
            .map(|v| {
                if let Value::Number(n) = v {
                    Ok(n)
                } else {
                    Err(RuntimeError::WrongArgument(
                        self.name(),
                        "a number",
                        v.type_str(),
                    ))
                }
            })
            .collect::<Result<Vec<&Rational64>, RuntimeError>>();
        match self {
            FactorOp::Add => {
                maybe_nums.map(|nums| Value::Number(nums.into_iter().fold(zero, |a, b| a + b)))
            }
            FactorOp::Sub => match args.len() {
                0 => self.arity_err("<...args>"),
                1 => maybe_nums.map(|nums| Value::Number(-nums[0])),
                _ => maybe_nums.map(|nums| {
                    Value::Number(nums[0] - nums[1..].iter().fold(zero, |a, b| a + *b))
                }),
            },
            FactorOp::Mul => {
                maybe_nums.map(|nums| Value::Number(nums.into_iter().fold(one, |a, b| a * b)))
            }
            FactorOp::Div => match args.len() {
                0 => self.arity_err("<...args>"),
                1 => maybe_nums.map(|nums| Value::Number(nums[0].recip())),
                _ => maybe_nums.and_then(|nums| {
                    let denominator = nums[1..].iter().fold(one, |a, b| a * *b);
                    if denominator.is_zero() {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(nums[0] / denominator))
                    }
                }),
            },
        }
    }
}

display_for_callable!(FactorOp);

#[cfg(test)]
mod tests {
    use super::*;

    fn n(n: i64) -> Value {
        Value::from(n)
    }

    #[test]
    fn test_add() {
        let scope = Scope::new(None);
        assert_eq!(FactorOp::Add.call(&[], &scope).unwrap(), n(0));
        assert_eq!(FactorOp::Add.call(&[n(2)], &scope).unwrap(), n(2));
        assert_eq!(
            FactorOp::Add
                .call(&[n(2), n(5), n(6), n(-3)], &scope)
                .unwrap(),
            n(10)
        );
    }

    #[test]
    fn test_sub() {
        let scope = Scope::new(None);
        assert!(matches!(
            FactorOp::Sub.call(&[], &scope),
            Err(RuntimeError::ArityError(..))
        ));
        assert_eq!(FactorOp::Sub.call(&[n(2)], &scope).unwrap(), n(-2));
        assert_eq!(
            FactorOp::Sub
                .call(&[n(2), n(5), n(6), n(-3)], &scope)
                .unwrap(),
            n(-6)
        );
    }

    #[test]
    fn test_mul() {
        let scope = Scope::new(None);
        assert_eq!(FactorOp::Mul.call(&[], &scope).unwrap(), n(1));
        assert_eq!(FactorOp::Mul.call(&[n(2)], &scope).unwrap(), n(2));
        assert_eq!(
            FactorOp::Mul
                .call(&[n(2), n(5), n(6), n(-3)], &scope)
                .unwrap(),
            n(-180)
        );
    }

    #[test]
    fn test_div() {
        let scope = Scope::new(None);
        let f = |num, den| Value::Number(Rational64::new(num, den));
        assert!(matches!(
            FactorOp::Div.call(&[], &scope),
            Err(RuntimeError::ArityError(..))
        ));
        assert_eq!(FactorOp::Div.call(&[n(2)], &scope).unwrap(), f(1, 2));
        assert_eq!(
            FactorOp::Div
                .call(&[n(2), n(5), n(6), n(-3)], &scope)
                .unwrap(),
            f(-2, 90)
        );
        assert!(matches!(
            FactorOp::Div.call(&[n(2), n(3), n(0)], &scope),
            Err(RuntimeError::DivisionByZero)
        ));
    }
}
