use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::memaddress::MemAddress;

type Table = RefCell<HashMap<String, MemAddress>>;
type Counter = RefCell<usize>;

#[derive(Debug)]
pub enum SymbolTable {
    RootTable {
        symbols: Table,
        temp_counter: Counter,
    },
    LocalTable {
        symbols: Table,
        temp_counter: Counter,
        top_scope: Rc<SymbolTable>,
    },
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable::RootTable {
            symbols: RefCell::new(HashMap::new()),
            temp_counter: RefCell::new(0),
        }
    }
}

impl SymbolTable {
    pub fn new(parent_table: Option<Rc<SymbolTable>>) -> SymbolTable {
        match parent_table {
            Some(top_scope) => SymbolTable::LocalTable {
                symbols: RefCell::new(HashMap::new()),
                temp_counter: RefCell::new(0),
                top_scope,
            },
            None => SymbolTable::default(),
        }
    }

    pub fn get(&self, symbol: &str) -> Option<MemAddress> {
        match self {
            SymbolTable::RootTable { symbols, .. } => symbols.borrow().get(symbol).cloned(),
            SymbolTable::LocalTable {
                symbols, top_scope, ..
            } => symbols
                .borrow()
                .get(symbol)
                .cloned()
                .or_else(|| top_scope.get(symbol)),
        }
    }

    pub fn get_new_temp_addr_idx(&self) -> usize {
        let temp_counter = match self {
            SymbolTable::RootTable { temp_counter, .. } => temp_counter,
            SymbolTable::LocalTable { temp_counter, .. } => temp_counter,
        };
        *temp_counter.borrow_mut() += 1;
        *temp_counter.borrow() - 1
    }

    pub fn insert(&self, symbol: String, value: MemAddress) {
        match self {
            SymbolTable::RootTable { symbols, .. } => symbols,
            SymbolTable::LocalTable { symbols, .. } => symbols,
        }
        .borrow_mut()
        .insert(symbol, value);
    }

    pub fn insert_in_root(&self, symbol: String, value: MemAddress) {
        match self {
            SymbolTable::LocalTable { top_scope, .. } => top_scope.insert_in_root(symbol, value),
            SymbolTable::RootTable { .. } => self.insert(symbol, value),
        };
    }

    pub fn get_top_scope(&self) -> Option<Rc<SymbolTable>> {
        match self {
            SymbolTable::LocalTable { top_scope, .. } => Some(top_scope.clone()),
            SymbolTable::RootTable { .. } => None,
        }
    }
}
