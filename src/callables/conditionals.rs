use std::slice;

use num::Zero;

use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
pub struct IsTrue;

impl IsTrue {
    pub fn inner_call(&self, val: &Value) -> bool {
        match val {
            Value::SExpr(_) => unreachable!(),
            Value::Identifier(_) => unreachable!(),
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

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 1 {
            return self.arity_err("<value>");
        }
        Ok(Value::from(self.inner_call(&args[0].eval(scope)?)))
    }
}

display_for_callable!(IsTrue);

#[derive(Debug, Clone)]
pub struct If;

impl Callable for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        if args.len() != 3 {
            return self.arity_err("<condition> <true expression> <false expression>");
        }
        if IsTrue.call(slice::from_ref(&args[0]), scope)? == Value::from(false) {
            args[2].eval(scope)
        } else {
            args[1].eval(scope)
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

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        let false_val = Value::from(false);
        for arg in args {
            if IsTrue.call(slice::from_ref(arg), scope)? == false_val {
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

    fn call(&self, args: &[Value], scope: &Scope) -> ExecutionResult {
        let true_val = Value::from(true);
        for arg in args {
            if IsTrue.call(slice::from_ref(arg), scope)? == true_val {
                return Ok(true_val);
            }
        }

        Ok(Value::from(false))
    }
}

display_for_callable!(Or);