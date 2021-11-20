use num::Rational64;

use crate::callables::prelude::*;

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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 0 {
            Err(CompilationError::EmptyArgs(self.name()))
        } else {
            Ok(())
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(*self)))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.is_empty() {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "at least one number",
                0,
            ));
        }

        let args_as_nums = |args: Vec<Value>| {
            args.into_iter()
                .map(|v| {
                    if let Value::Number(n) = v {
                        Ok(n)
                    } else {
                        Err(RuntimeError::WrongDataType(
                            self.name(),
                            "a number",
                            v.type_str(),
                        ))
                    }
                })
                .collect::<RuntimeResult<Vec<Rational64>>>()
        };

        Ok(Value::from(match self {
            ComparisonOp::Eq => args.iter().all(|v| v == &args[0]),
            ComparisonOp::Ne => args.iter().any(|v| v != &args[0]),
            ComparisonOp::Gt => args_as_nums(args)?.windows(2).all(|w| w[0] > w[1]),
            ComparisonOp::Lt => args_as_nums(args)?.windows(2).all(|w| w[0] < w[1]),
            ComparisonOp::Ge => args_as_nums(args)?.windows(2).all(|w| w[0] >= w[1]),
            ComparisonOp::Le => args_as_nums(args)?.windows(2).all(|w| w[0] <= w[1]),
        }))
    }
}

display_for_callable!(ComparisonOp);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn v(n: i64) -> Value {
        Value::from(n)
    }

    #[test]
    fn test_eq() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert!(matches!(
            ComparisonOp::Eq.execute(&vm, vec![]),
            Err(RuntimeError::WrongArityS(..))
        ));
        assert_eq!(
            ComparisonOp::Eq.execute(&vm, vec![v(2)]).unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Eq.execute(&vm, vec![v(2), v(2)]).unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Eq
                .execute(&vm, vec![v(2), v(2), v(3)])
                .unwrap(),
            false.into()
        );
    }

    #[test]
    fn test_ne() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert_eq!(
            ComparisonOp::Ne.execute(&vm, vec![v(2)]).unwrap(),
            false.into()
        );
        assert_eq!(
            ComparisonOp::Ne.execute(&vm, vec![v(2), v(2)]).unwrap(),
            false.into()
        );
        assert_eq!(
            ComparisonOp::Ne
                .execute(&vm, vec![v(2), v(2), v(3)])
                .unwrap(),
            true.into()
        );
    }

    #[test]
    fn test_gt() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert_eq!(
            ComparisonOp::Gt.execute(&vm, vec![v(5)]).unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Gt
                .execute(&vm, vec![v(2), v(2), v(1)])
                .unwrap(),
            false.into()
        );
        assert_eq!(
            ComparisonOp::Gt
                .execute(&vm, vec![v(2), v(1), v(1)])
                .unwrap(),
            false.into()
        );
        assert_eq!(
            ComparisonOp::Gt
                .execute(&vm, vec![v(3), v(2), v(1)])
                .unwrap(),
            true.into()
        );
    }

    #[test]
    fn test_lt() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert_eq!(
            ComparisonOp::Lt.execute(&vm, vec![v(5)]).unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Lt
                .execute(&vm, vec![v(1), v(1), v(2)])
                .unwrap(),
            false.into()
        );
        assert_eq!(
            ComparisonOp::Lt
                .execute(&vm, vec![v(1), v(2), v(2)])
                .unwrap(),
            false.into()
        );
        assert_eq!(
            ComparisonOp::Lt
                .execute(&vm, vec![v(1), v(2), v(3)])
                .unwrap(),
            true.into()
        );
    }

    #[test]
    fn test_ge() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert_eq!(
            ComparisonOp::Ge.execute(&vm, vec![v(5)]).unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Ge
                .execute(&vm, vec![v(2), v(2), v(1)])
                .unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Ge
                .execute(&vm, vec![v(2), v(1), v(1)])
                .unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Ge
                .execute(&vm, vec![v(3), v(2), v(1)])
                .unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Ge.execute(&vm, vec![v(1), v(2)]).unwrap(),
            false.into()
        );
    }

    #[test]
    fn test_le() {
        let vm = VMState::new(HashMap::new(), Vec::new());
        assert_eq!(
            ComparisonOp::Le.execute(&vm, vec![v(5)]).unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Le
                .execute(&vm, vec![v(1), v(1), v(2)])
                .unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Le
                .execute(&vm, vec![v(1), v(2), v(2)])
                .unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Le
                .execute(&vm, vec![v(1), v(2), v(3)])
                .unwrap(),
            true.into()
        );
        assert_eq!(
            ComparisonOp::Le.execute(&vm, vec![v(2), v(1)]).unwrap(),
            false.into()
        );
    }
}
