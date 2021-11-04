use std::{cell::RefCell, cmp::max};

use crate::vm::{RuntimeError, Value};

type ValuesTable = RefCell<Vec<Option<Value>>>;
type GetResult = Result<Value, RuntimeError>;

#[derive(Debug, Default)]
pub struct Scope {
    vars: ValuesTable,
    temps: ValuesTable,
}

impl Scope {
    pub fn get_var(&self, index: usize) -> GetResult {
        self.vars
            .borrow()
            .get(index)
            .cloned()
            .flatten()
            .ok_or_else(|| {
                RuntimeError::CompilerError(format!("Variable {} not found in scope", index))
            })
    }

    pub fn get_temp(&self, index: usize) -> GetResult {
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
        self.inner_store(&self.vars, index, value)
    }

    pub fn store_temp(&self, index: usize, value: Value) {
        self.inner_store(&self.temps, index, value)
    }

    fn inner_store(&self, table: &ValuesTable, index: usize, value: Value) {
        let table_len = self.vars.borrow().len();
        table.borrow_mut().resize(max(table_len, index + 1), None);
        let _ = table.borrow_mut().get_mut(index).unwrap().insert(value);
    }
}