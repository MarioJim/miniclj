use std::{collections::HashMap, rc::Rc};

use crate::callables::{
    conditionals::{If, IsTrue},
    factorops::FactorOp,
    iofns::Print,
    Callable,
};

#[derive(Debug)]
pub struct CallablesTable {
    builtin: HashMap<String, Rc<dyn Callable>>,
    custom: HashMap<String, Rc<dyn Callable>>,
}

impl Default for CallablesTable {
    fn default() -> Self {
        let mut builtin: HashMap<String, Rc<dyn Callable>> = HashMap::new();
        for c in [FactorOp::Add, FactorOp::Sub, FactorOp::Mul, FactorOp::Div] {
            builtin.insert(c.name().to_string(), Rc::new(c));
        }
        builtin.insert(If.name().into(), Rc::new(If));
        builtin.insert(IsTrue.name().into(), Rc::new(IsTrue));
        builtin.insert(Print.name().into(), Rc::new(Print));

        CallablesTable {
            builtin,
            custom: HashMap::new(),
        }
    }
}

impl CallablesTable {
    pub fn get<T: AsRef<str>>(&self, symbol: &T) -> Option<Rc<dyn Callable>> {
        self.custom
            .get(symbol.as_ref())
            .or_else(|| self.builtin.get(symbol.as_ref()))
            .cloned()
    }

    pub fn insert(&mut self, symbol: String, value: Rc<dyn Callable>) {
        self.custom.insert(symbol, value);
    }
}
