use num::Zero;

use crate::{
    callables::{Callable, ExecutionResult},
    SExpr, Scope, Value,
};

#[derive(Debug, Clone)]
pub struct IsTrue;

impl IsTrue {
    pub fn inner_call(&self, val: &Value) -> bool {
        match val {
            Value::Symbol(_) => {
                unreachable!("IsTrue::inner_call called with a symbol")
            }
            Value::Number(n) => n.is_zero(),
            Value::Nil => false,
            _ => true,
        }
    }
}

impl Callable for IsTrue {
    fn name(&self) -> &'static str {
        "true?"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<value>");
        }
        Ok(Value::from(self.inner_call(
            &args.into_iter().next().unwrap().eval(scope)?,
        )))
    }
}

display_for_callable!(IsTrue);

#[derive(Debug, Clone)]
pub struct If;

impl Callable for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Scope) -> ExecutionResult {
        if args.len() != 3 {
            return self.arity_err("<condition> <true expression> <false expression>");
        }
        let mut args_iter = args.into_iter();
        let condition = args_iter.next().unwrap();
        let true_expr = args_iter.next().unwrap();
        let false_expr = args_iter.next().unwrap();
        if IsTrue.call(vec![condition], scope)? == Value::from(true) {
            true_expr.eval(scope)
        } else {
            false_expr.eval(scope)
        }
    }
}

display_for_callable!(If);

#[derive(Debug, Clone)]
pub struct And;

impl Callable for And {
    fn name(&self) -> &'static str {
        "and"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Scope) -> ExecutionResult {
        let false_val = Value::from(false);
        for arg in args.into_iter() {
            if IsTrue.call(vec![arg], scope)? == false_val {
                return Ok(false_val);
            }
        }

        Ok(Value::from(true))
    }
}

display_for_callable!(And);

#[derive(Debug, Clone)]
pub struct Or;

impl Callable for Or {
    fn name(&self) -> &'static str {
        "or"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Scope) -> ExecutionResult {
        let true_val = Value::from(true);
        for arg in args.into_iter() {
            if IsTrue.call(vec![arg], scope)? == true_val {
                return Ok(true_val);
            }
        }

        Ok(Value::from(false))
    }
}

display_for_callable!(Or);
