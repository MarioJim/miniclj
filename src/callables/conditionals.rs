use crate::{
    callables::{Callable, ExecutionResult},
    Scope, Value,
};

#[derive(Debug, Clone)]
struct IsTrue;

impl Callable for IsTrue {
    fn name(&self) -> &'static str {
        "true?"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(IsTrue);

#[derive(Debug, Clone)]
struct If;

impl Callable for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(If);

#[derive(Debug, Clone)]
struct And;

impl Callable for And {
    fn name(&self) -> &'static str {
        "and"
    }

    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(And);

#[derive(Debug, Clone)]
struct Or;

impl Callable for Or {
    fn name(&self) -> &'static str {
        "or"
    }
    fn call(&self, _: &[Value], _: &Scope) -> ExecutionResult {
        todo!()
    }
}

display_for_callable!(Or);
