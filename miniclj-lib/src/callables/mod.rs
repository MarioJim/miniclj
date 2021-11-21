macro_rules! display_for_callable {
    ($callable:ty) => {
        impl std::fmt::Display for $callable {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}

/// Exposes the base `Callable` trait
mod callable;
/// Exposes the callables related to collections
mod collection;
/// Exposes the callables used to compare and order values
mod comparisonops;
/// Exposes the callables related to checking if a value is truthy or falsy
mod conditionals;
/// Exposes the `loop` and `recur` callables
mod cycles;
/// Exposes the callables related to mathematical operations
mod factorops;
/// Exposes the `do` callable
mod groupingfns;
/// Exposes the callables related to input and output
mod iofns;
/// Exposes the `fn` callable
mod lambda;
/// Exposes callables related to adding variables to the local and global scope
mod scopefns;
/// Exposes callables used to cast values of some types to others
mod typecastingfns;

use std::collections::HashMap as RustHashMap;

pub use callable::Callable;
pub use collection::creation::{HashMap, List, Set, Vector};
pub use comparisonops::ComparisonOp;
pub use factorops::FactorOp;

macro_rules! add_fn {
    ($table: expr, $callable: path) => {
        $table.insert(String::from($callable.name()), Box::new($callable));
    };
}

/// The map of symbols to callables exposed by the language
pub struct CallablesTable(RustHashMap<String, Box<dyn Callable>>);

impl Default for CallablesTable {
    fn default() -> CallablesTable {
        let mut table: RustHashMap<String, Box<dyn Callable>> = RustHashMap::new();
        add_fn!(table, collection::access::First);
        add_fn!(table, collection::access::Rest);
        add_fn!(table, collection::access::Nth);
        add_fn!(table, collection::access::Get);
        add_fn!(table, collection::access::Count);
        add_fn!(table, collection::access::IsEmpty);

        add_fn!(table, collection::creation::List);
        add_fn!(table, collection::creation::Vector);
        add_fn!(table, collection::creation::Set);
        add_fn!(table, collection::creation::HashMap);

        add_fn!(table, collection::generation::Range);

        add_fn!(table, collection::modification::Cons);
        add_fn!(table, collection::modification::Conj);
        add_fn!(table, collection::modification::Del);

        add_fn!(table, collection::transducers::Map);
        add_fn!(table, collection::transducers::Filter);
        add_fn!(table, collection::transducers::Reduce);

        add_fn!(table, comparisonops::ComparisonOp::Eq);
        add_fn!(table, comparisonops::ComparisonOp::Ne);
        add_fn!(table, comparisonops::ComparisonOp::Gt);
        add_fn!(table, comparisonops::ComparisonOp::Lt);
        add_fn!(table, comparisonops::ComparisonOp::Ge);
        add_fn!(table, comparisonops::ComparisonOp::Le);

        add_fn!(table, conditionals::IsTrue);
        add_fn!(table, conditionals::If);
        add_fn!(table, conditionals::And);
        add_fn!(table, conditionals::Or);

        add_fn!(table, cycles::Loop);
        add_fn!(table, cycles::Recur);

        add_fn!(table, factorops::FactorOp::Add);
        add_fn!(table, factorops::FactorOp::Sub);
        add_fn!(table, factorops::FactorOp::Mul);
        add_fn!(table, factorops::FactorOp::Div);

        add_fn!(table, groupingfns::Do);

        add_fn!(table, iofns::Print);
        add_fn!(table, iofns::Println);
        add_fn!(table, iofns::Read);

        add_fn!(table, lambda::Lambda);

        add_fn!(table, scopefns::Def);
        add_fn!(table, scopefns::Defn);
        add_fn!(table, scopefns::Let);

        add_fn!(table, typecastingfns::NumberCast);
        add_fn!(table, typecastingfns::StringCast);
        add_fn!(table, typecastingfns::Ord);
        add_fn!(table, typecastingfns::Chr);

        CallablesTable(table)
    }
}

impl CallablesTable {
    pub fn get(&self, name: &str) -> Option<Box<dyn Callable>> {
        self.0.get(name).cloned()
    }
}

impl std::fmt::Debug for CallablesTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallablesTable({} functions included)", self.0.len())
    }
}

pub mod prelude {
    pub use super::Callable;
    pub use crate::{
        compiler::{CompilationError, CompilerState},
        memaddress::MemAddress,
        vm::{RuntimeError, RuntimeResult, VMState, Value},
    };
}
