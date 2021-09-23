use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
struct Def;

impl Callable for Def {
    fn name(&self) -> &'static str {
        "def"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Def);

#[derive(Debug, Clone)]
struct Defn;

impl Callable for Defn {
    fn name(&self) -> &'static str {
        "defn"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Defn);

#[derive(Debug, Clone)]
struct Let;

impl Callable for Let {
    fn name(&self) -> &'static str {
        "let"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Let);

#[derive(Debug, Clone)]
struct Loop;

impl Callable for Loop {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Loop);

#[derive(Debug, Clone)]
struct Recur;

impl Callable for Recur {
    fn name(&self) -> &'static str {
        "recur"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Recur);
