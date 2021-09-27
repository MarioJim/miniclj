use num::Rational64;

use crate::{
    callables::{Callable, ExecutionResult, RuntimeError},
    Scope, Value,
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

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.is_empty() {
            return self.arity_err("<...args>");
        }
        let evaled_args = args
            .iter()
            .map(|v| v.eval(scope))
            .collect::<Result<Vec<Value>, RuntimeError>>()?;
        let args_as_nums = |args: Vec<Value>| {
            args.into_iter()
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
                .collect::<Result<Vec<Rational64>, RuntimeError>>()
        };
        Ok(Value::from(match self {
            ComparisonOp::Eq => evaled_args.iter().all(|v| v == &evaled_args[0]),
            ComparisonOp::Ne => evaled_args.iter().any(|v| v != &evaled_args[0]),
            ComparisonOp::Gt => args_as_nums(evaled_args)?.windows(2).all(|w| w[0] > w[1]),
            ComparisonOp::Lt => args_as_nums(evaled_args)?.windows(2).all(|w| w[0] < w[1]),
            ComparisonOp::Ge => args_as_nums(evaled_args)?.windows(2).all(|w| w[0] >= w[1]),
            ComparisonOp::Le => args_as_nums(evaled_args)?.windows(2).all(|w| w[0] <= w[1]),
        }))
    }
}

display_for_callable!(ComparisonOp);

#[cfg(test)]
mod tests {
    use super::*;

    fn n(n: i64) -> Value {
        Value::from(n)
    }

    #[test]
    fn test_eq() {
        let scope = Scope::new(None);
        assert!(matches!(
            ComparisonOp::Eq.call(&[], &scope),
            Err(RuntimeError::ArityError(..))
        ));
        assert_eq!(ComparisonOp::Eq.call(&[n(2)], &scope).unwrap(), n(1));
        assert_eq!(ComparisonOp::Eq.call(&[n(2), n(2)], &scope).unwrap(), n(1));
        assert_eq!(
            ComparisonOp::Eq.call(&[n(2), n(2), n(3)], &scope).unwrap(),
            n(0)
        );
    }

    #[test]
    fn test_ne() {
        let scope = Scope::new(None);
        assert_eq!(ComparisonOp::Ne.call(&[n(2)], &scope).unwrap(), n(0));
        assert_eq!(ComparisonOp::Ne.call(&[n(2), n(2)], &scope).unwrap(), n(0));
        assert_eq!(
            ComparisonOp::Ne.call(&[n(2), n(2), n(3)], &scope).unwrap(),
            n(1)
        );
    }

    #[test]
    fn test_gt() {
        let scope = Scope::new(None);
        assert_eq!(ComparisonOp::Gt.call(&[n(5)], &scope).unwrap(), n(1));
        assert_eq!(
            ComparisonOp::Gt.call(&[n(2), n(2), n(1)], &scope).unwrap(),
            n(0)
        );
        assert_eq!(
            ComparisonOp::Gt.call(&[n(2), n(1), n(1)], &scope).unwrap(),
            n(0)
        );
        assert_eq!(
            ComparisonOp::Gt.call(&[n(3), n(2), n(1)], &scope).unwrap(),
            n(1)
        );
    }

    #[test]
    fn test_lt() {
        let scope = Scope::new(None);
        assert_eq!(ComparisonOp::Lt.call(&[n(5)], &scope).unwrap(), n(1));
        assert_eq!(
            ComparisonOp::Lt.call(&[n(1), n(1), n(2)], &scope).unwrap(),
            n(0)
        );
        assert_eq!(
            ComparisonOp::Lt.call(&[n(1), n(2), n(2)], &scope).unwrap(),
            n(0)
        );
        assert_eq!(
            ComparisonOp::Lt.call(&[n(1), n(2), n(3)], &scope).unwrap(),
            n(1)
        );
    }

    #[test]
    fn test_ge() {
        let scope = Scope::new(None);
        assert_eq!(ComparisonOp::Ge.call(&[n(5)], &scope).unwrap(), n(1));
        assert_eq!(
            ComparisonOp::Ge.call(&[n(2), n(2), n(1)], &scope).unwrap(),
            n(1)
        );
        assert_eq!(
            ComparisonOp::Ge.call(&[n(2), n(1), n(1)], &scope).unwrap(),
            n(1)
        );
        assert_eq!(
            ComparisonOp::Ge.call(&[n(3), n(2), n(1)], &scope).unwrap(),
            n(1)
        );
        assert_eq!(ComparisonOp::Ge.call(&[n(1), n(2)], &scope).unwrap(), n(0));
    }

    #[test]
    fn test_le() {
        let scope = Scope::new(None);
        assert_eq!(ComparisonOp::Le.call(&[n(5)], &scope).unwrap(), n(1));
        assert_eq!(
            ComparisonOp::Le.call(&[n(1), n(1), n(2)], &scope).unwrap(),
            n(1)
        );
        assert_eq!(
            ComparisonOp::Le.call(&[n(1), n(2), n(2)], &scope).unwrap(),
            n(1)
        );
        assert_eq!(
            ComparisonOp::Le.call(&[n(1), n(2), n(3)], &scope).unwrap(),
            n(1)
        );
        assert_eq!(ComparisonOp::Le.call(&[n(2), n(1)], &scope).unwrap(), n(0));
    }
}
