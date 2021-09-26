use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

type SymbolTable = RefCell<HashMap<String, Value>>;

pub enum Scope {
    RootScope(SymbolTable),
    LocalScope(SymbolTable, Rc<Scope>),
}

impl Scope {
    pub fn new(parent_scope: Option<Rc<Scope>>) -> Scope {
        if let Some(scope) = parent_scope {
            return Scope::LocalScope(Default::default(), scope);
        }
        let symbol_table = HashMap::new();
        // TODO: Fill out with callables
        Scope::RootScope(RefCell::new(symbol_table))
    }

    pub fn get(&self, identifier: &str) -> Option<Value> {
        match self {
            Scope::RootScope(table) => table.borrow().get(identifier).cloned(),
            Scope::LocalScope(table, top_scope) => table
                .borrow()
                .get(identifier)
                .cloned()
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

    pub fn insert_in_root(&self, identifier: String, value: Value) {
        match self {
            Scope::LocalScope(_, s) => s.insert_in_root(identifier, value),
            Scope::RootScope(_) => self.insert(identifier, value),
        };
    }
}
