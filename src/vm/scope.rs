#![allow(dead_code)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{memaddress::MemAddress, vm::Value};

type SymbolTable = RefCell<HashMap<MemAddress, Value>>;

#[derive(Debug)]
pub enum Scope {
    RootScope(SymbolTable),
    LocalScope(SymbolTable, Rc<Scope>),
}

impl Scope {
    pub fn new(parent_scope: Option<Rc<Scope>>) -> Scope {
        match parent_scope {
            Some(scope) => Scope::LocalScope(Default::default(), scope),
            None => Scope::RootScope(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, address: &MemAddress) -> Option<Value> {
        match self {
            Scope::RootScope(table) => table.borrow().get(address).cloned(),
            Scope::LocalScope(table, top_scope) => table
                .borrow()
                .get(address)
                .cloned()
                .or_else(|| top_scope.get(address)),
        }
    }

    pub fn insert(&self, address: MemAddress, value: Value) {
        match self {
            Scope::RootScope(t) => t,
            Scope::LocalScope(t, _) => t,
        }
        .borrow_mut()
        .insert(address, value);
    }

    pub fn insert_in_root(&self, address: MemAddress, value: Value) {
        match self {
            Scope::LocalScope(_, s) => s.insert_in_root(address, value),
            Scope::RootScope(_) => self.insert(address, value),
        };
    }
}
