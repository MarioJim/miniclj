use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
    hash::Hash,
};

use num::BigRational;

pub type SExprs = Vec<Box<SExpr>>;

#[derive(Debug)]
pub enum SExpr {
    Expr(SExprs),
    Lambda(SExprs),
    List(SExprs),
    Vector(SExprs),
    Set(HashSet<Box<Atom>>),
    Map(HashMap<String, Box<SExpr>>),
    Atom(Box<Atom>),
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
            SExpr::Atom(a) => write!(f, "{}", a),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Atom {
    Identifier(String),
    String(String),
    Number(BigRational),
    Factor(FactorOp),
    Comparison(ComparisonOp),
    LambdaArg,
    Nil,
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Identifier(s) => write!(f, "{}", s),
            Atom::String(s) => write!(f, "{}", s),
            Atom::Number(n) => write!(f, "{}", n),
            Atom::Factor(o) => write!(f, "{}", o),
            Atom::Comparison(o) => write!(f, "{}", o),
            Atom::LambdaArg => write!(f, "%"),
            Atom::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FactorOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for FactorOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FactorOp::Add => write!(f, "+"),
            FactorOp::Sub => write!(f, "-"),
            FactorOp::Mul => write!(f, "*"),
            FactorOp::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl Display for ComparisonOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOp::Eq => write!(f, "="),
            ComparisonOp::Ne => write!(f, "!="),
            ComparisonOp::Gt => write!(f, ">"),
            ComparisonOp::Lt => write!(f, "<"),
            ComparisonOp::Ge => write!(f, ">="),
            ComparisonOp::Le => write!(f, "<="),
        }
    }
}
