use num::Rational64;

use crate::{
    ast::ComparisonOp,
    value::{Callable, Value},
};

impl Callable for ComparisonOp {
    fn call(&self, args: Vec<Value>) -> Value {
        if args.is_empty() {
            return Value::Error(String::from("Comparison function called with no arguments"));
        }
        let args_as_nums = |a: Vec<Value>| {
            a.into_iter()
                .map(|v| {
                    if let Value::Number(n) = v {
                        Ok(n)
                    } else {
                        Err(format!("Value {} can't be ordered", v))
                    }
                })
                .collect::<Result<Vec<Rational64>, String>>()
        };
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
        assert!(matches!(ComparisonOp::Eq.call(vec![]), Value::Error(_)));
        assert_eq!(ComparisonOp::Eq.call(vec![n(2)]), n(1));
        assert_eq!(ComparisonOp::Eq.call(vec![n(2), n(2)]), n(1));
        assert_eq!(ComparisonOp::Eq.call(vec![n(2), n(2), n(3)]), n(0));
    }

    #[test]
    fn test_ne() {
        assert_eq!(ComparisonOp::Ne.call(vec![n(2)]), n(0));
        assert_eq!(ComparisonOp::Ne.call(vec![n(2), n(2)]), n(0));
        assert_eq!(ComparisonOp::Ne.call(vec![n(2), n(2), n(3)]), n(1));
    }

    #[test]
    fn test_gt() {
        assert_eq!(ComparisonOp::Gt.call(vec![n(5)]), n(1));
        assert_eq!(ComparisonOp::Gt.call(vec![n(2), n(2), n(1)]), n(0));
        assert_eq!(ComparisonOp::Gt.call(vec![n(2), n(1), n(1)]), n(0));
        assert_eq!(ComparisonOp::Gt.call(vec![n(3), n(2), n(1)]), n(1));
    }

    #[test]
    fn test_lt() {
        assert_eq!(ComparisonOp::Lt.call(vec![n(5)]), n(1));
        assert_eq!(ComparisonOp::Lt.call(vec![n(1), n(1), n(2)]), n(0));
        assert_eq!(ComparisonOp::Lt.call(vec![n(1), n(2), n(2)]), n(0));
        assert_eq!(ComparisonOp::Lt.call(vec![n(1), n(2), n(3)]), n(1));
    }

    #[test]
    fn test_ge() {
        assert_eq!(ComparisonOp::Ge.call(vec![n(5)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(vec![n(2), n(2), n(1)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(vec![n(2), n(1), n(1)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(vec![n(3), n(2), n(1)]), n(1));
        assert_eq!(ComparisonOp::Ge.call(vec![n(1), n(2)]), n(0));
    }

    #[test]
    fn test_le() {
        assert_eq!(ComparisonOp::Le.call(vec![n(5)]), n(1));
        assert_eq!(ComparisonOp::Le.call(vec![n(1), n(1), n(2)]), n(1));
        assert_eq!(ComparisonOp::Le.call(vec![n(1), n(2), n(2)]), n(1));
        assert_eq!(ComparisonOp::Le.call(vec![n(1), n(2), n(3)]), n(1));
        assert_eq!(ComparisonOp::Le.call(vec![n(2), n(1)]), n(0));
    }
}
