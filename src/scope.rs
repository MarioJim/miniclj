use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

type SymbolTable = RefCell<HashMap<String, Value>>;

pub enum Scope {
    RootScope(SymbolTable),
    LocalScope(SymbolTable, Rc<Scope>),
}

impl Scope {
    pub fn new(maybe_scope: Option<Rc<Scope>>) -> Scope {
        if let Some(scope) = maybe_scope {
            return Scope::LocalScope(Default::default(), scope);
        }
        let symbol_table = HashMap::new();
        // TODO: Fill out with callables
        Scope::RootScope(RefCell::new(symbol_table))
    }

    pub fn get(&self, identifier: &str) -> Option<Value> {
        match self {
            Scope::RootScope(table) => table.borrow().get(identifier).map(|v| v.to_owned()),
            Scope::LocalScope(table, top_scope) => table
                .borrow()
                .get(identifier)
                .map(|v| v.to_owned())
                .or_else(|| top_scope.get(identifier)),
        }
    }

    pub fn insert(&self, identifier: String, value: Value) {
        match self {
            Scope::RootScope(t) => t,
            Scope::LocalScope(t, _) => t,
        }
        .borrow_mut()
        .insert(identifier, value);
    }
}
