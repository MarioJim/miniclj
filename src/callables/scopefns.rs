use std::rc::Rc;

use crate::{
    callables::{lambdafns::LambdaFn, Callable, ExecutionResult, RuntimeError},
    SExpr, Scope, Value,
};

#[derive(Debug, Clone)]
pub struct Def;

impl Callable for Def {
    fn name(&self) -> &'static str {
        "def"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<symbol> <value>");
        }
        let mut args_iter = args.into_iter();
        let maybe_symbol = args_iter.next().unwrap();
        let symbol = if let SExpr::Value(Value::Symbol(sym)) = maybe_symbol {
            sym
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a symbol",
                maybe_symbol.type_str(),
            ));
        };
        let val = args_iter.next().unwrap().eval(scope)?;

        scope.insert_in_root(symbol.clone(), val);
        Ok(Value::Symbol(symbol))
    }
}

display_for_callable!(Def);

#[derive(Debug, Clone)]
pub struct Defn;

impl Callable for Defn {
    fn name(&self) -> &'static str {
        "defn"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 3 {
            return self.arity_err("<symbol> <arguments vector> <expression>");
        }

        let mut args_iter = args.into_iter();
        let maybe_symbol = args_iter.next().unwrap();
        let symbol = if let SExpr::Value(Value::Symbol(sym)) = maybe_symbol {
            sym
        } else {
            return Err(RuntimeError::WrongArgument(
                self.name(),
                "a symbol",
                maybe_symbol.type_str(),
            ));
        };

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
        let val = Value::Fn(Box::new(lambdafn));

        scope.insert_in_root(symbol.clone(), val);
        Ok(Value::Symbol(symbol))
    }
}

display_for_callable!(Defn);

#[derive(Debug, Clone)]
pub struct Let;

impl Callable for Let {
    fn name(&self) -> &'static str {
        "let"
    }

    fn call(&self, args: Vec<SExpr>, scope: &Rc<Scope>) -> ExecutionResult {
        if args.len() != 2 {
            return self.arity_err("<vector of bindings> <body>");
        }

        let first_arg_error = Err(RuntimeError::Error(format!(
            "First argument of {} must be a vector with symbol - value pairs",
            self.name()
        )));

        let mut args_iter = args.into_iter();
        let bindings_vector = if let SExpr::Vector(v) = args_iter.next().unwrap() {
            v
        } else {
            return first_arg_error;
        };

        if bindings_vector.len() % 2 == 1 {
            return first_arg_error;
        }
        let inner_scope = Rc::new(Scope::new(Some(Rc::clone(scope))));
        let mut bindings_iter = bindings_vector.into_iter();
        while let Some(key_expr) = bindings_iter.next() {
            let symbol = if let SExpr::Value(Value::Symbol(sym)) = *key_expr {
                sym
            } else {
                return first_arg_error;
            };
            let val = bindings_iter.next().unwrap().eval(scope)?;
            inner_scope.insert(symbol, val);
        }

        args_iter.next().unwrap().eval(&inner_scope)
    }
}

display_for_callable!(Let);

#[derive(Debug, Clone)]
pub struct Loop;

impl Callable for Loop {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn call(&self, _: Vec<SExpr>, _: &Rc<Scope>) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Loop);

#[derive(Debug, Clone)]
pub struct Recur;

impl Callable for Recur {
    fn name(&self) -> &'static str {
        "recur"
    }

    fn call(&self, _: Vec<SExpr>, _: &Rc<Scope>) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Recur);
