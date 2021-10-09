use std::rc::Rc;

use crate::compiler::{
    callables::{Callable, ExecutionResult, RuntimeError},
    SExpr, Scope, Value,
};

#[derive(Debug, Clone)]
pub struct LambdaFn {
    symbols: Vec<String>,
    body: SExpr,
}

impl LambdaFn {
    pub fn new(symbols: Vec<String>, body: SExpr) -> LambdaFn {
        LambdaFn { symbols, body }
    }

    pub fn new_from_literal(body: SExpr) -> LambdaFn {
        LambdaFn::new(vec![String::from("%")], body)
    }
}

impl Callable for LambdaFn {
    fn name(&self) -> &'static str {
        "*fn"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != self.symbols.len() {
            return Err(RuntimeError::Error(format!(
                "User defined function called with {} arguments, expected {}",
                args.len(),
                self.symbols.len()
            )));
        }

        let inner_scope = Rc::new(Scope::new(Some(Rc::clone(scope))));
        for (sym, val) in self.symbols.iter().zip(args.into_iter()) {
            inner_scope.insert(sym.clone(), val.eval(scope)?);
        }

        self.body.clone().eval(&inner_scope)
    }

    fn is_user_defined(&self) -> bool {
        true
    }
}

display_for_callable!(LambdaFn);
#[derive(Debug, Clone)]
pub struct AnonymousFn;

impl Callable for AnonymousFn {
    fn name(&self) -> &'static str {
        "fn"
    }

    fn call(&self, args: Vec<SExpr>, _: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<arguments vector> <expression>");
        }
        let mut args_iter = args.into_iter();
        let maybe_fn_args = args_iter.next().unwrap();
        let fn_args = if let SExpr::Vector(v) = maybe_fn_args {
            Ok(v)
        } else {
            Err(RuntimeError::WrongArgument(
                self.name(),
                "a vector of symbols",
                maybe_fn_args.type_str(),
            ))
        }?;
        let mut symbols = vec![];
        for maybe_symbol in fn_args {
            if let SExpr::Value(Value::Symbol(string)) = *maybe_symbol {
                symbols.push(string)
            } else {
                return Err(RuntimeError::Error(format!(
                    "The arguments vector of a function only accepts identifiers, not {}",
                    maybe_symbol.type_str()
                )));
            }
        }

        let body = args_iter.next().unwrap();

        let lambdafn = LambdaFn::new(symbols, body);
        Ok(Value::Fn(Box::new(lambdafn)))
    }
}

display_for_callable!(AnonymousFn);
