use crate::compiler::Literal;

pub type SExprs = Vec<SExpr>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr {
    Expr(SExprs),
    ShortLambda(SExprs),
    List(SExprs),
    Vector(SExprs),
    Set(SExprs),
    Map(SExprs),
    Literal(Literal),
}

impl SExpr {
    pub fn type_str(&self) -> &'static str {
        match self {
            SExpr::Expr(_) => "a s-expression",
            SExpr::ShortLambda(_) => "a lambda function",
            SExpr::List(_) => "a list",
            SExpr::Vector(_) => "a vector",
            SExpr::Set(_) => "a set",
            SExpr::Map(_) => "a map",
            SExpr::Literal(v) => v.type_str(),
        }
    }
}
