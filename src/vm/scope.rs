use std::cell::RefCell;

use crate::vm::{RuntimeError, RuntimeResult, Value};

type ValuesTable = RefCell<Vec<Option<Value>>>;

#[derive(Debug, Default)]
pub struct Scope {
    vars: ValuesTable,
    temps: ValuesTable,
}

impl Scope {
    pub fn get_var(&self, index: usize) -> RuntimeResult<Value> {
        self.vars
            .borrow()
            .get(index)
            .cloned()
            .flatten()
            .ok_or_else(|| {
                RuntimeError::CompilerError(format!("Variable {} not found in scope", index))
            })
    }

    pub fn get_temp(&self, index: usize) -> RuntimeResult<Value> {
        self.temps
            .borrow()
            .get(index)
            .cloned()
            .flatten()
            .ok_or_else(|| {
                RuntimeError::CompilerError(format!("Temp variable {} not found in scope", index))
            })
    }

    pub fn store_var(&self, index: usize, value: Value) {
        inner_store(&self.vars, index, value);
    }

    pub fn store_temp(&self, index: usize, value: Value) {
        inner_store(&self.temps, index, value);
    }
}

fn inner_store(table: &ValuesTable, index: usize, value: Value) {
    let table_len = table.borrow().len();
    if table_len < index + 1 {
        table.borrow_mut().resize(index + 4, None);
    }
    let _ = table.borrow_mut().get_mut(index).unwrap().insert(value);
}
