use num::{Rational64, Zero};

use crate::{
    ast::FactorOp,
    value::{Callable, Value},
};

impl Callable for FactorOp {
    fn call(&self, args: Vec<Value>) -> Value {
        let one = Rational64::from_integer(1);
        let zero = Rational64::from_integer(0);
        match self {
            FactorOp::Add => {
                let maybe_nums = args
                    .into_iter()
                    .map(|a| {
                        if let Value::Number(n) = a {
                            Ok(n)
                        } else {
                            Err(a)
                        }
                    })
                    .collect::<Result<Vec<Rational64>, Value>>();

                match maybe_nums {
                    Ok(v) => {
                        let result = v.into_iter().fold(zero, |a, b| a + b);
                        Value::Number(result)
                    }
                    Err(v) => Value::Error(format!("Addition can't be called with argument {}", v)),
                }
            }
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
                    let maybe_nums = args
                        .into_iter()
                        .map(|a| {
                            if let Value::Number(n) = a {
                                Ok(n)
                            } else {
                                Err(a)
                            }
                        })
                        .collect::<Result<Vec<Rational64>, Value>>();

                    let nums = match maybe_nums {
                        Ok(v) => v,
                        Err(v) => {
                            return Value::Error(format!(
                                "Substraction can't be called with argument {}",
                                v
                            ));
                        }
                    };

                    Value::Number(nums[0] - nums[1..].iter().fold(zero, |a, b| a + b))
                }
            },
            FactorOp::Mul => {
                let maybe_nums = args
                    .into_iter()
                    .map(|a| {
                        if let Value::Number(n) = a {
                            Ok(n)
                        } else {
                            Err(a)
                        }
                    })
                    .collect::<Result<Vec<Rational64>, Value>>();

                match maybe_nums {
                    Ok(v) => {
                        let result = v.into_iter().fold(one, |a, b| a * b);
                        Value::Number(result)
                    }
                    Err(v) => Value::Error(format!(
                        "Multiplication can't be called with argument {}",
                        v
                    )),
                }
            }
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
                    let maybe_nums = args
                        .into_iter()
                        .map(|a| {
                            if let Value::Number(n) = a {
                                Ok(n)
                            } else {
                                Err(a)
                            }
                        })
                        .collect::<Result<Vec<Rational64>, Value>>();

                    let nums = match maybe_nums {
                        Ok(v) => v,
                        Err(v) => {
                            return Value::Error(format!(
                                "Division can't be called with argument {}",
                                v
                            ));
                        }
                    };

                    let denominator = nums[1..].iter().fold(one, |a, b| a * b);

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
        let args = vec![2, 5, 6, -3].into_iter().map(n).collect::<Vec<Value>>();
        assert_eq!(FactorOp::Add.call(vec![]), n(0));
        assert_eq!(FactorOp::Add.call(vec![args[0].clone()]), n(2));
        assert_eq!(FactorOp::Add.call(args), n(10));
    }

    #[test]
    fn test_sub() {
        let args = vec![2, 5, 6, -3].into_iter().map(n).collect::<Vec<Value>>();
        assert!(matches!(FactorOp::Sub.call(vec![]), Value::Error(_)));
        assert_eq!(FactorOp::Sub.call(vec![args[0].clone()]), n(-2));
        assert_eq!(FactorOp::Sub.call(args), n(-6));
    }

    #[test]
    fn test_mul() {
        let args = vec![2, 5, 6, -3].into_iter().map(n).collect::<Vec<Value>>();
        assert_eq!(FactorOp::Mul.call(vec![]), n(1));
        assert_eq!(FactorOp::Mul.call(vec![args[0].clone()]), n(2));
        assert_eq!(FactorOp::Mul.call(args), n(-180));
    }

    #[test]
    fn test_div() {
        let f = |num, den| Value::Number(Rational64::new(num, den));
        let args = vec![2, 5, 6, -3].into_iter().map(n).collect::<Vec<Value>>();
        assert!(matches!(FactorOp::Div.call(vec![]), Value::Error(_)));
        assert_eq!(FactorOp::Div.call(vec![args[0].clone()]), f(1, 2));
        assert_eq!(FactorOp::Div.call(args), f(-2, 90));
    }
}
