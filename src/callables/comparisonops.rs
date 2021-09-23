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

    fn call(&self, args: &[Value], _: &Scope) -> ExecutionResult {
        if args.is_empty() {
            return Err(RuntimeError::ArityError(self.name(), "<...args>"));
        }
        fn args_as_nums<'a>(
            function_name: &'static str,
            args: &'a [Value],
        ) -> Result<Vec<&'a Rational64>, RuntimeError> {
            args.iter()
                .map(|v| {
                    if let Value::Number(n) = v {
                        Ok(n)
                    } else {
                        Err(RuntimeError::WrongArgument(
                            function_name,
                            "a number",
                            v.type_str(),
                        ))
                    }
                })
                .collect()
        }
        let nm = self.name();
        match self {
            ComparisonOp::Eq => Ok(args.iter().all(|v| v == &args[0])),
            ComparisonOp::Ne => Ok(args.iter().any(|v| v != &args[0])),
            ComparisonOp::Gt => args_as_nums(nm, args).map(|n| n.windows(2).all(|n| n[0] > n[1])),
            ComparisonOp::Lt => args_as_nums(nm, args).map(|n| n.windows(2).all(|n| n[0] < n[1])),
            ComparisonOp::Ge => args_as_nums(nm, args).map(|n| n.windows(2).all(|n| n[0] >= n[1])),
            ComparisonOp::Le => args_as_nums(nm, args).map(|n| n.windows(2).all(|n| n[0] <= n[1])),
        }
        .map(|boolean| Value::Number(Rational64::from_integer(if boolean { 1 } else { 0 })))
    }
}

display_for_callable!(ComparisonOp);

#[cfg(test)]
mod tests {
    use super::*;

    fn n(n: i64) -> Value {
        Value::Number(Rational64::from_integer(n))
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
