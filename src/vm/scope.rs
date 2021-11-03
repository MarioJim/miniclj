use std::{cell::RefCell, cmp::max, rc::Rc};

use crate::{
    instruction::InstructionPtr,
    memaddress::{Lifetime, MemAddress},
    vm::Value,
};

type ValuesTable = RefCell<Vec<Option<Value>>>;

#[derive(Debug, Default)]
pub struct Scope {
    vars: ValuesTable,
    temps: ValuesTable,
    enclosing_state: Option<EnclosingState>,
}

#[derive(Debug)]
pub struct EnclosingState {
    return_address: MemAddress,
    instruction_ptr: InstructionPtr,
    scope: Rc<Scope>,
}

impl Scope {
    pub fn new(
        return_address: MemAddress,
        instruction_ptr: InstructionPtr,
        scope: Rc<Scope>,
    ) -> Scope {
        Scope {
            vars: Default::default(),
            temps: Default::default(),
            enclosing_state: Some(EnclosingState {
                return_address,
                instruction_ptr,
                scope,
            }),
        }
    }

    pub fn top_state(&self) -> (MemAddress, InstructionPtr, Rc<Scope>) {
        let state = self
            .enclosing_state
            .as_ref()
            .expect("Can't return from the top scope");
        (
            state.return_address,
            state.instruction_ptr,
            state.scope.clone(),
        )
    }

    pub fn get(&self, address: &MemAddress) -> Value {
        self.inner_get(address)
            .or_else(|| {
                self.enclosing_state
                    .as_ref()
                    .map(|state| state.scope.inner_get(address))
                    .flatten()
            })
            .unwrap()
    }

    fn inner_get(&self, address: &MemAddress) -> Option<Value> {
        self.get_table_by_lifetime(address.lifetime())
            .borrow()
            .get(address.idx())
            .cloned()
            .flatten()
    }

    pub fn insert(&self, address: MemAddress, value: Value) {
        let table = self.get_table_by_lifetime(address.lifetime());
        let table_len = table.borrow().len();
        let address_idx = address.idx();
        table
            .borrow_mut()
            .resize(max(table_len, address_idx + 1), None);

        let _ = table
            .borrow_mut()
            .get_mut(address_idx)
            .unwrap()
            .insert(value);
    }

    fn get_table_by_lifetime(&self, lifetime: Lifetime) -> &ValuesTable {
        match (self.enclosing_state.as_ref(), lifetime) {
            (None, Lifetime::GlobalVar) => &self.vars,
            (None, Lifetime::Temporal) => &self.temps,
            (Some(_), Lifetime::LocalVar) => &self.vars,
            (Some(_), Lifetime::Temporal) => &self.temps,
            (Some(top_scope), Lifetime::GlobalVar) => {
                top_scope.scope.get_table_by_lifetime(lifetime)
            }
            _ => panic!("Address not found"),
        }
    }
}
