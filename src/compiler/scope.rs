use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::compiler::Literal;

type SymbolTable = RefCell<HashMap<String, Literal>>;

#[derive(Debug)]
pub enum Scope {
    RootScope(SymbolTable),
    LocalScope(SymbolTable, Rc<Scope>),
}

impl Scope {
    pub fn new(parent_scope: Option<Rc<Scope>>) -> Scope {
        if let Some(scope) = parent_scope {
            return Scope::LocalScope(Default::default(), scope);
        }
        let mut symbol_table = HashMap::new();

        Scope::RootScope(RefCell::new(symbol_table))
    }

    pub fn get(&self, symbol: &str) -> Option<Literal> {
        match self {
            Scope::RootScope(table) => table.borrow().get(symbol).cloned(),
            Scope::LocalScope(table, top_scope) => table
                .borrow()
                .get(symbol)
                .cloned()
                .or_else(|| top_scope.get(symbol)),
        }
    }

    pub fn insert(&self, symbol: String, value: Literal) {
        match self {
            Scope::RootScope(t) => t,
            Scope::LocalScope(t, _) => t,
        }
        .borrow_mut()
        .insert(symbol, value);
    }

    pub fn insert_in_root(&self, symbol: String, value: Literal) {
        match self {
            Scope::LocalScope(_, s) => s.insert_in_root(symbol, value),
            Scope::RootScope(_) => self.insert(symbol, value),
        };
    }
}
