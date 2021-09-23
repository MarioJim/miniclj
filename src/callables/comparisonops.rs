use std::fmt::{Display, Formatter};

use num::Rational64;

use crate::{callables::Callable, value::Value};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl Display for ComparisonOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOp::Eq => write!(f, "="),
            ComparisonOp::Ne => write!(f, "!="),
            ComparisonOp::Gt => write!(f, ">"),
            ComparisonOp::Lt => write!(f, "<"),
            ComparisonOp::Ge => write!(f, ">="),
            ComparisonOp::Le => write!(f, "<="),
        }
    }
}

impl Callable for ComparisonOp {
    fn call(&self, args: &[Value]) -> Value {
        if args.is_empty() {
            return Value::Error(String::from("Comparison function called with no arguments"));
        }
        fn args_as_nums(a: &[Value]) -> Result<Vec<&Rational64>, String> {
            a.iter()
                .map(|v| {
                    if let Value::Number(n) = v {
                        Ok(n)
                    } else {
                        Err(format!("Value {} can't be ordered", v))
                    }
                })
                .collect()
        }
        let result = match self {
            ComparisonOp::Eq => Ok(args.iter().all(|v| v == &args[0])),
            ComparisonOp::Ne => Ok(args.iter().any(|v| v != &args[0])),
            ComparisonOp::Gt => args_as_nums(args).map(|n| n.windows(2).all(|p| p[0] > p[1])),
            ComparisonOp::Lt => args_as_nums(args).map(|n| n.windows(2).all(|p| p[0] < p[1])),
            ComparisonOp::Ge => args_as_nums(args).map(|n| n.windows(2).all(|p| p[0] >= p[1])),
            ComparisonOp::Le => args_as_nums(args).map(|n| n.windows(2).all(|p| p[0] <= p[1])),
        };
        match result {
            Ok(boolean) => {
                if boolean {
                    Value::Number(Rational64::from_integer(1))
                } else {
                    Value::Number(Rational64::from_integer(0))
                }
            }
            Err(msg) => Value::Error(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(n: i64) -> Value {
        Value::Number(Rational64::from_integer(n))
    }

    #[test]
    fn test_eq() {
        assert!(matches!(ComparisonOp::Eq.call(&[]), Value::Error(_)));
        assert_eq!(ComparisonOp::Eq.call(&[n(2)]), n(1));
        assert_eq!(ComparisonOp::Eq.call(&[n(2), n(2)]), n(1));
        assert_eq!(ComparisonOp::Eq.call(&[n(2), n(2), n(3)]), n(0));
    }

    #[test]
    fn test_ne() {
        assert_eq!(ComparisonOp::Ne.call(&[n(2)]), n(0));
        assert_eq!(ComparisonOp::Ne.call(&[n(2), n(2)]), n(0));
        assert_eq!(ComparisonOp::Ne.call(&[n(2), n(2), n(3)]), n(1));
    }

    #[test]
    fn test_gt() {
        assert_eq!(ComparisonOp::Gt.call(&[n(5)]), n(1));
        assert_eq!(ComparisonOp::Gt.call(&[n(2), n(2), n(1)]), n(0));
        assert_eq!(ComparisonOp::Gt.call(&[n(2), n(1), n(1)]), n(0));
        assert_eq!(ComparisonOp::Gt.call(&[n(3), n(2), n(1)]), n(1));
    }

    #[test]
    fn test_lt() {
        assert_eq!(ComparisonOp::Lt.call(&[n(5)]), n(1));
        assert_eq!(ComparisonOp::Lt.call(&[n(1), n(1), n(2)]), n(0));
        assert_eq!(ComparisonOp::Lt.call(&[n(1), n(2), n(2)]), n(0));
        assert_eq!(ComparisonOp::Lt.call(&[n(1), n(2), n(3)]), n(1));
    }

    #[test]
    fn test_ge() {
        assert_eq!(ComparisonOp::Ge.call(&[n(5)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(&[n(2), n(2), n(1)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(&[n(2), n(1), n(1)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(&[n(3), n(2), n(1)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(&[n(1), n(2)]), n(0));
    }

    #[test]
    fn test_le() {
        assert_eq!(ComparisonOp::Le.call(&[n(5)]), n(1));
        assert_eq!(ComparisonOp::Le.call(&[n(1), n(1), n(2)]), n(1));
        assert_eq!(ComparisonOp::Le.call(&[n(1), n(2), n(2)]), n(1));
        assert_eq!(ComparisonOp::Le.call(&[n(1), n(2), n(3)]), n(1));
        assert_eq!(ComparisonOp::Le.call(&[n(2), n(1)]), n(0));
    }
}
