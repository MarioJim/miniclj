macro_rules! display_for_callable {
    ($callable:ty) => {
        impl std::fmt::Display for $callable {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}

mod callable;
mod collection;
mod comparisonops;
mod conditionals;
mod factorops;
mod groupingfns;
mod iofns;
mod lambda;
mod scopefns;
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

pub struct CallablesTable(RustHashMap<String, Box<dyn Callable>>);

impl Default for CallablesTable {
    fn default() -> CallablesTable {
        let mut table: RustHashMap<String, Box<dyn Callable>> = RustHashMap::new();
        add_fn!(table, collection::access::First);
        add_fn!(table, collection::access::Rest);
        add_fn!(table, collection::access::Cons);
        add_fn!(table, collection::access::Conj);
        add_fn!(table, collection::access::Nth);
        add_fn!(table, collection::access::Get);
        add_fn!(table, collection::access::Count);
        add_fn!(table, collection::access::IsEmpty);

        add_fn!(table, collection::creation::List);
        add_fn!(table, collection::creation::Vector);
        add_fn!(table, collection::creation::Set);
        add_fn!(table, collection::creation::HashMap);

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
        add_fn!(table, scopefns::Loop);
        add_fn!(table, scopefns::Recur);

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
