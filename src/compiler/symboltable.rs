use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::compiler::MemAddress;

type Table = RefCell<HashMap<String, MemAddress>>;

#[derive(Debug)]
pub enum SymbolTable {
    RootScope(Table),
    LocalScope(Table, Rc<SymbolTable>),
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable::new(None)
    }
}

impl SymbolTable {
    pub fn new(parent_table: Option<Rc<SymbolTable>>) -> SymbolTable {
        if let Some(symbol_table) = parent_table {
            return SymbolTable::LocalScope(Default::default(), symbol_table);
        }
        let symbol_table = HashMap::new();

        SymbolTable::RootScope(RefCell::new(symbol_table))
    }

    pub fn get(&self, symbol: &str) -> Option<MemAddress> {
        match self {
            SymbolTable::RootScope(table) => table.borrow().get(symbol).cloned(),
            SymbolTable::LocalScope(table, top_scope) => table
                .borrow()
                .get(symbol)
                .cloned()
                .or_else(|| top_scope.get(symbol)),
        }
    }

    pub fn insert(&self, symbol: String, value: MemAddress) {
        match self {
            SymbolTable::RootScope(t) => t,
            SymbolTable::LocalScope(t, _) => t,
        }
        .borrow_mut()
        .insert(symbol, value);
    }

    pub fn insert_in_root(&self, symbol: String, value: MemAddress) {
        match self {
            SymbolTable::LocalScope(_, s) => s.insert_in_root(symbol, value),
            SymbolTable::RootScope(_) => self.insert(symbol, value),
        };
    }
}
