use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};

use crate::{Scope, Value};

pub type SExprs = Vec<Box<SExpr>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr {
    Expr(SExprs),
    Lambda(SExprs),
    List(SExprs),
    Vector(SExprs),
    Set(HashSet<Box<Value>>),
    Map(HashMap<String, Box<SExpr>>),
    Value(Box<Value>),
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
            SExpr::Map(_) => todo!(),
            SExpr::Value(v) => write!(f, "{}", v),
        }
    }
}

impl SExpr {
    pub fn eval(&self, _: &Scope) -> Value {
        // TODO: Fill out logic
        Value::SExpr(self.clone())
    }
}
