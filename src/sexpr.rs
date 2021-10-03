use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::{
    callables::{lambdafns::LambdaFn, ExecutionResult, RuntimeError},
    value::{list, map, set, vector},
    Scope, Value,
};

pub type SExprs = Vec<Box<SExpr>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr {
    Expr(SExprs),
    Lambda(SExprs),
    List(SExprs),
    Vector(SExprs),
    Set(SExprs),
    Map(SExprs),
    Value(Value),
}

impl Display for SExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SExpr::Expr(exprs) => {
                write!(f, "(")?;
                let mut it = exprs.iter();
                if let Some(e) = it.next() {
                    write!(f, "{}", e)?;
                }
                for expr in it {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
            SExpr::Lambda(exprs) => {
                write!(f, "#(")?;
                let mut it = exprs.iter();
                if let Some(e) = it.next() {
                    write!(f, "{}", e)?;
                }
                for expr in it {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
            SExpr::List(exprs) => {
                write!(f, "'(")?;
                let mut it = exprs.iter();
                if let Some(e) = it.next() {
                    write!(f, "{}", e)?;
                }
                for expr in it {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
            SExpr::Vector(exprs) => {
                write!(f, "[")?;
                let mut it = exprs.iter();
                if let Some(e) = it.next() {
                    write!(f, "{}", e)?;
                }
                for expr in it {
                    write!(f, " {}", expr)?;
                }
                write!(f, "]")
            }
            SExpr::Set(exprs) => {
                write!(f, "#{{")?;
                let mut it = exprs.iter();
                if let Some(e) = it.next() {
                    write!(f, "{}", e)?;
                }
                for expr in it {
                    write!(f, " {}", expr)?;
                }
                write!(f, "}}")
            }
            SExpr::Map(exprs) => {
                write!(f, "{{")?;
                let mut it = exprs.iter();
                if let Some(e) = it.next() {
                    write!(f, "{}", e)?;
                }
                for expr in it {
                    write!(f, " {}", expr)?;
                }
                write!(f, "}}")
            }
            SExpr::Value(v) => write!(f, "{}", v),
        }
    }
}

impl SExpr {
    pub fn type_str(&self) -> &'static str {
        match self {
            SExpr::Expr(_) => "a s-expression",
            SExpr::Lambda(_) => "an anonymous function",
            SExpr::List(_) => "a list",
            SExpr::Vector(_) => "a vector",
            SExpr::Set(_) => "a set",
            SExpr::Map(_) => "a map",
            SExpr::Value(v) => v.type_str(),
        }
    }

    pub fn eval(self, scope: &Rc<Scope>) -> ExecutionResult {
        match self {
            SExpr::Expr(exprs) => {
                let mut exprs_iter = exprs.into_iter();
                let first_expr = match exprs_iter.next() {
                    Some(expr) => expr.eval(scope),
                    None => Ok(Value::List(list::List::default())),
                }?;
                if let Value::Fn(callable) = first_expr {
                    let args = exprs_iter.map(|e| *e).collect();
                    callable.call(args, scope)
                } else {
                    Err(RuntimeError::Error(format!(
                        "Value {} can't be called",
                        first_expr
                    )))
                }
            }
            SExpr::Lambda(exprs) => {
                let lambdafn = LambdaFn::new_from_literal(SExpr::Expr(exprs));
                Ok(Value::Fn(Box::new(lambdafn)))
            }
            SExpr::List(exprs) => {
                let mut result = list::List::default();
                for expr in exprs {
                    result.push_back(expr.eval_inside_list()?);
                }
                Ok(Value::List(result))
            }
            SExpr::Vector(exprs) => {
                let mut result = vector::Vector::default();
                for expr in exprs {
                    result.push(expr.eval(scope)?);
                }
                Ok(Value::Vector(result))
            }
            SExpr::Set(exprs) => {
                let mut result = set::Set::default();
                for expr in exprs {
                    result.insert(expr.eval(scope)?);
                }
                Ok(Value::Set(result))
            }
            SExpr::Map(exprs) => {
                let mut result = map::Map::default();
                let mut exprs_iter = exprs.into_iter();
                while let Some(key_expr) = exprs_iter.next() {
                    let key = key_expr.eval(scope)?;
                    let val = exprs_iter.next().unwrap().eval(scope)?;
                    result.insert(key, val);
                }
                Ok(Value::Map(result))
            }
            SExpr::Value(val) => val.eval(scope),
        }
    }

    pub fn eval_inside_list(self) -> ExecutionResult {
        match self {
            SExpr::Expr(exprs) | SExpr::Lambda(exprs) | SExpr::List(exprs) => {
                let mut result = list::List::default();
                for expr in exprs {
                    result.push_back(expr.eval_inside_list()?);
                }
                Ok(Value::List(result))
            }
            SExpr::Vector(exprs) => {
                let mut result = vector::Vector::default();
                for expr in exprs {
                    result.push(expr.eval_inside_list()?);
                }
                Ok(Value::Vector(result))
            }
            SExpr::Set(exprs) => {
                let mut result = set::Set::default();
                for expr in exprs {
                    result.insert(expr.eval_inside_list()?);
                }
                Ok(Value::Set(result))
            }
            SExpr::Map(exprs) => {
                if exprs.len() % 2 == 0 {
                    let mut result = map::Map::default();
                    let mut exprs_iter = exprs.into_iter();
                    while let Some(key_expr) = exprs_iter.next() {
                        let key = key_expr.eval_inside_list()?;
                        let val = exprs_iter.next().unwrap().eval_inside_list()?;
                        result.insert(key, val);
                    }
                    Ok(Value::Map(result))
                } else {
                    Err(RuntimeError::Error(String::from(
                        "Map must contain an even number of values",
                    )))
                }
            }
            SExpr::Value(val) => Ok(val),
        }
    }
}
