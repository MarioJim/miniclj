use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
};

use crate::value::Value;

#[derive(PartialEq, Eq)]
pub struct List(VecDeque<Value>);

impl List {
    fn cons(&mut self, val: Value) {
        self.0.push_front(val);
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = self
            .0
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "({})", string)
    }
}
