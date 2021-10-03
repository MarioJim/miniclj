use num::{Rational64, Zero};

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    value::SExpr,
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

    fn call(&self, args: Vec<SExpr>, scope: &Scope) -> ExecutionResult {
        let one = Rational64::from_integer(1);
        let zero = Rational64::from_integer(0);
        let len_nums = args.len();
        let mut nums = args
            .into_iter()
            .map(|v| {
                let v_type = v.type_str();
                if let Value::Number(n) = v.eval(scope)? {
                    Ok(n)
                } else {
                    Err(RuntimeError::WrongArgument(self.name(), "a number", v_type))
                }
            })
            .collect::<Result<Vec<Rational64>, RuntimeError>>()?
            .into_iter();
        match self {
            FactorOp::Add => Ok(Value::Number(nums.fold(zero, |a, b| a + b))),
            FactorOp::Sub => match len_nums {
                0 => self.arity_err("<...args>"),
                1 => Ok(Value::Number(-nums.next().unwrap())),
                _ => {
                    let positive = nums.next().unwrap();
                    let negative = nums.fold(zero, |a, b| a + b);
                    Ok(Value::Number(positive - negative))
                }
            },
            FactorOp::Mul => Ok(Value::Number(nums.fold(one, |a, b| a * b))),
            FactorOp::Div => match len_nums {
                0 => self.arity_err("<...args>"),
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
    use super::*;

    fn v(n: i64) -> Value {
        Value::from(n)
    }

    fn s(n: i64) -> SExpr {
        SExpr::Value(v(n))
    }

    #[test]
    fn test_add() {
        let scope = Scope::new(None);
        assert_eq!(FactorOp::Add.call(vec![], &scope).unwrap(), v(0));
        assert_eq!(FactorOp::Add.call(vec![s(2)], &scope).unwrap(), v(2));
        assert_eq!(
            FactorOp::Add
                .call(vec![s(2), s(5), s(6), s(-3)], &scope)
                .unwrap(),
            v(10)
        );
    }

    #[test]
    fn test_sub() {
        let scope = Scope::new(None);
        assert!(matches!(
            FactorOp::Sub.call(vec![], &scope),
            Err(RuntimeError::ArityError(..))
        ));
        assert_eq!(FactorOp::Sub.call(vec![s(2)], &scope).unwrap(), v(-2));
        assert_eq!(
            FactorOp::Sub
                .call(vec![s(2), s(5), s(6), s(-3)], &scope)
                .unwrap(),
            v(-6)
        );
    }

    #[test]
    fn test_mul() {
        let scope = Scope::new(None);
        assert_eq!(FactorOp::Mul.call(vec![], &scope).unwrap(), v(1));
        assert_eq!(FactorOp::Mul.call(vec![s(2)], &scope).unwrap(), v(2));
        assert_eq!(
            FactorOp::Mul
                .call(vec![s(2), s(5), s(6), s(-3)], &scope)
                .unwrap(),
            v(-180)
        );
    }

    #[test]
    fn test_div() {
        let scope = Scope::new(None);
        let f = |num, den| Value::Number(Rational64::new(num, den));
        assert!(matches!(
            FactorOp::Div.call(vec![], &scope),
            Err(RuntimeError::ArityError(..))
        ));
        assert_eq!(FactorOp::Div.call(vec![s(2)], &scope).unwrap(), f(1, 2));
        assert_eq!(
            FactorOp::Div
                .call(vec![s(2), s(5), s(6), s(-3)], &scope)
                .unwrap(),
            f(-2, 90)
        );
        assert!(matches!(
            FactorOp::Div.call(vec![s(2), s(3), s(0)], &scope),
            Err(RuntimeError::DivisionByZero)
        ));
    }
}
