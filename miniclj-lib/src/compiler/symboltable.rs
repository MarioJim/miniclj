use std::{cell::RefCell, collections::HashMap, rc::Rc};

use smol_str::SmolStr;

use crate::memaddress::{Lifetime, MemAddress};

type Table = RefCell<HashMap<SmolStr, MemAddress>>;
type Counter = RefCell<usize>;

#[derive(Debug)]
pub enum SymbolTable {
    Global {
        symbols: Table,
        temp_counter: Counter,
        var_counter: Counter,
    },
    Local {
        parent_table: Rc<SymbolTable>,
        symbols: Table,
        temp_counter: Counter,
        var_counter: Counter,
    },
}

impl Default for SymbolTable {
    fn default() -> SymbolTable {
        SymbolTable::Global {
            symbols: RefCell::new(HashMap::new()),
            temp_counter: RefCell::new(0),
            var_counter: RefCell::new(0),
        }
    }
}

impl SymbolTable {
    pub fn new_local(parent_table: Rc<SymbolTable>, starting_var_count: usize) -> SymbolTable {
        SymbolTable::Local {
            parent_table,
            symbols: RefCell::new(HashMap::new()),
            temp_counter: RefCell::new(0),
            var_counter: RefCell::new(starting_var_count),
        }
    }

    pub fn get(&self, symbol: &str) -> Option<MemAddress> {
        match self {
            SymbolTable::Global { symbols, .. } => symbols.borrow().get(symbol).copied(),
            SymbolTable::Local {
                symbols,
                parent_table,
                ..
            } => symbols
                .borrow()
                .get(symbol)
                .copied()
                .or_else(|| parent_table.get(symbol)),
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
            (SymbolTable::Global { var_counter, .. }, Lifetime::GlobalVar | Lifetime::LocalVar)
            | (SymbolTable::Local { var_counter, .. }, Lifetime::LocalVar) => var_counter,
            (
                SymbolTable::Global { temp_counter, .. } | SymbolTable::Local { temp_counter, .. },
                Lifetime::Temporal,
            ) => temp_counter,
            (SymbolTable::Local { parent_table, .. }, Lifetime::GlobalVar) => {
                parent_table.get_counter(lifetime)
            }
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
            (SymbolTable::Global { symbols, .. }, Lifetime::GlobalVar)
            | (SymbolTable::Global { symbols, .. }, Lifetime::LocalVar)
            | (SymbolTable::Local { symbols, .. }, Lifetime::LocalVar) => symbols,
            (SymbolTable::Local { parent_table, .. }, Lifetime::GlobalVar) => parent_table.get_symbols_table(lifetime),
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

    pub fn parent_table(&self) -> Option<Rc<SymbolTable>> {
        match self {
            SymbolTable::Local { parent_table, .. } => Some(parent_table.clone()),
            SymbolTable::Global { .. } => None,
        }
    }
}
