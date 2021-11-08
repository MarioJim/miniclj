use std::{cell::RefCell, collections::HashMap, rc::Rc};

use smol_str::SmolStr;

use crate::memaddress::{Lifetime, MemAddress};

type Table = RefCell<HashMap<SmolStr, MemAddress>>;
type Counter = RefCell<usize>;

#[derive(Debug)]
pub enum SymbolTable {
    RootTable {
        symbols: Table,
        temp_counter: Counter,
        var_counter: Counter,
    },
    LocalTable {
        top_scope: Rc<SymbolTable>,
        symbols: Table,
        temp_counter: Counter,
        var_counter: Counter,
    },
}

impl Default for SymbolTable {
    fn default() -> SymbolTable {
        SymbolTable::RootTable {
            symbols: RefCell::new(HashMap::new()),
            temp_counter: RefCell::new(0),
            var_counter: RefCell::new(0),
        }
    }
}

impl SymbolTable {
    pub fn new(parent_table: Option<Rc<SymbolTable>>) -> SymbolTable {
        match parent_table {
            Some(top_scope) => SymbolTable::LocalTable {
                top_scope,
                symbols: RefCell::new(HashMap::new()),
                temp_counter: RefCell::new(0),
                var_counter: RefCell::new(0),
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

    pub fn new_address(&self, lifetime: Lifetime) -> MemAddress {
        let counter = self.get_counter(lifetime);
        let addr_idx = *counter.borrow();
        *counter.borrow_mut() += 1;
        MemAddress::new(lifetime, addr_idx)
    }

    fn get_counter(&self, lifetime: Lifetime) -> &Counter {
        match (self, lifetime) {
            (SymbolTable::RootTable { var_counter, .. }, Lifetime::GlobalVar) => var_counter,
            (SymbolTable::RootTable { var_counter, .. }, Lifetime::LocalVar) => var_counter,
            (SymbolTable::RootTable { temp_counter, .. }, Lifetime::Temporal) => temp_counter,
            (SymbolTable::LocalTable { top_scope, .. }, Lifetime::GlobalVar) => {
                top_scope.get_counter(lifetime)
            }
            (SymbolTable::LocalTable { var_counter, .. }, Lifetime::LocalVar) => var_counter,
            (SymbolTable::LocalTable { temp_counter, .. }, Lifetime::Temporal) => temp_counter,
            (_, Lifetime::Constant) => panic!("The symbol table doesn't store constants"),
        }
    }

    pub fn insert(&self, symbol: SmolStr, address: MemAddress) {
        self.get_symbols_table(address.lifetime())
            .borrow_mut()
            .insert(symbol, address);
    }

    fn get_symbols_table(&self, lifetime: Lifetime) -> &Table {
        match (self, lifetime) {
            (SymbolTable::RootTable { symbols, .. }, Lifetime::GlobalVar) => symbols,
            (SymbolTable::RootTable { symbols, .. }, Lifetime::LocalVar) => symbols,
            (SymbolTable::LocalTable { top_scope, .. }, Lifetime::GlobalVar) => top_scope.get_symbols_table(lifetime),
            (SymbolTable::LocalTable { symbols, .. }, Lifetime::LocalVar) => symbols,
            _ => panic!("Can't insert addresses into the symbol table with lifetimes other than global or local"),
        }
    }

    pub fn remove_local(&self, symbol: &str) {
        let symbols = self.get_symbols_table(Lifetime::LocalVar);
        debug_assert_ne!(
            (*symbols.borrow()).get(symbol).unwrap().lifetime(),
            Lifetime::GlobalVar
        );
        symbols.borrow_mut().remove(symbol);
    }

    pub fn top_scope(&self) -> Option<Rc<SymbolTable>> {
        match self {
            SymbolTable::LocalTable { top_scope, .. } => Some(top_scope.clone()),
            SymbolTable::RootTable { .. } => None,
        }
    }
}
