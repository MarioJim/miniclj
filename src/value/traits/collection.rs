use crate::value::Value;

pub trait Collection {
    fn cons(&mut self, val: Value);
    fn get(&self, key: &Value) -> Value;
    fn len(&self) -> usize;
    fn empty(&self) -> bool;
}
