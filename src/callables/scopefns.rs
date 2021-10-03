use crate::{
    callables::{Callable, ExecutionResult},
    value::SExpr,
    Scope,
};

#[derive(Debug, Clone)]
pub struct Def;

impl Callable for Def {
    fn name(&self) -> &'static str {
        "def"
    }

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Def);

#[derive(Debug, Clone)]
pub struct Defn;

impl Callable for Defn {
    fn name(&self) -> &'static str {
        "defn"
    }

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Defn);

#[derive(Debug, Clone)]
pub struct Let;

impl Callable for Let {
    fn name(&self) -> &'static str {
        "let"
    }

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Let);

#[derive(Debug, Clone)]
pub struct Loop;

impl Callable for Loop {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
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

    fn call(&self, _: Vec<SExpr>, _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Recur);
