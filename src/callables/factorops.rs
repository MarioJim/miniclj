use std::fmt::{self, Display};

use num::{Rational64, Zero};

use crate::{Callable, Scope, Value};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FactorOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for FactorOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FactorOp::Add => write!(f, "+"),
            FactorOp::Sub => write!(f, "-"),
            FactorOp::Mul => write!(f, "*"),
            FactorOp::Div => write!(f, "/"),
        }
    }
}

impl Callable for FactorOp {
    fn call(&self, args: &[Value], _: &Scope) -> Value {
        let one = Rational64::from_integer(1);
        let zero = Rational64::from_integer(0);
        let maybe_nums = args
            .iter()
            .map(|a| {
                if let Value::Number(n) = a {
                    Ok(n)
                } else {
                    Err(a)
                }
            })
            .collect::<Result<Vec<&Rational64>, &Value>>();
        match self {
            FactorOp::Add => match maybe_nums {
                Ok(v) => {
                    let result = v.into_iter().fold(zero, |a, b| a + b);
                    Value::Number(result)
                }
                Err(v) => Value::Error(format!("Addition can't be called with argument {}", v)),
            },
            FactorOp::Sub => match args.len() {
                0 => Value::Error(String::from("Substraction called with no arguments")),
                1 => {
                    if let Value::Number(n) = args[0] {
                        Value::Number(n * Rational64::from_integer(-1))
                    } else {
                        Value::Error(format!(
                            "Substraction can't be called with argument {}",
                            args[0]
                        ))
                    }
                }
                _ => {
                    let nums = match maybe_nums {
                        Ok(v) => v,
                        Err(v) => {
                            return Value::Error(format!(
                                "Substraction can't be called with argument {}",
                                v
                            ));
                        }
                    };

                    Value::Number(nums[0] - nums[1..].iter().fold(zero, |a, b| a + *b))
                }
            },
            FactorOp::Mul => match maybe_nums {
                Ok(v) => {
                    let result = v.into_iter().fold(one, |a, b| a * b);
                    Value::Number(result)
                }
                Err(v) => Value::Error(format!(
                    "Multiplication can't be called with argument {}",
                    v
                )),
            },
            FactorOp::Div => match args.len() {
                0 => Value::Error(String::from("Division called with no arguments")),
                1 => {
                    if let Value::Number(n) = args[0] {
                        if n.is_zero() {
                            Value::Error(String::from("Division by zero"))
                        } else {
                            Value::Number(n.recip())
                        }
                    } else {
                        Value::Error(format!(
                            "Division can't be called with argument {}",
                            args[0]
                        ))
                    }
                }
                _ => {
                    let nums = match maybe_nums {
                        Ok(v) => v,
                        Err(v) => {
                            return Value::Error(format!(
                                "Division can't be called with argument {}",
                                v
                            ));
                        }
                    };

                    let denominator = nums[1..].iter().fold(one, |a, b| a * *b);

                    if denominator.is_zero() {
                        Value::Error(String::from("Division by zero"))
                    } else {
                        Value::Number(nums[0] / denominator)
                    }
                }
            },
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
    fn test_add() {
        let scope = Scope::new(None);
        assert_eq!(FactorOp::Add.call(&[], &scope), n(0));
        assert_eq!(FactorOp::Add.call(&[n(2)], &scope), n(2));
        assert_eq!(
            FactorOp::Add.call(&[n(2), n(5), n(6), n(-3)], &scope),
            n(10)
        );
    }

    #[test]
    fn test_sub() {
        let scope = Scope::new(None);
        assert!(matches!(FactorOp::Sub.call(&[], &scope), Value::Error(_)));
        assert_eq!(FactorOp::Sub.call(&[n(2)], &scope), n(-2));
        assert_eq!(
            FactorOp::Sub.call(&[n(2), n(5), n(6), n(-3)], &scope),
            n(-6)
        );
    }

    #[test]
    fn test_mul() {
        let scope = Scope::new(None);
        assert_eq!(FactorOp::Mul.call(&[], &scope), n(1));
        assert_eq!(FactorOp::Mul.call(&[n(2)], &scope), n(2));
        assert_eq!(
            FactorOp::Mul.call(&[n(2), n(5), n(6), n(-3)], &scope),
            n(-180)
        );
    }

    #[test]
    fn test_div() {
        let scope = Scope::new(None);
        let f = |num, den| Value::Number(Rational64::new(num, den));
        assert!(matches!(FactorOp::Div.call(&[], &scope), Value::Error(_)));
        assert_eq!(FactorOp::Div.call(&[n(2)], &scope), f(1, 2));
        assert_eq!(
            FactorOp::Div.call(&[n(2), n(5), n(6), n(-3)], &scope),
            f(-2, 90)
        );
        assert!(matches!(
            FactorOp::Div.call(&[n(2), n(3), n(0)], &scope),
            Value::Error(_)
        ));
    }
}
