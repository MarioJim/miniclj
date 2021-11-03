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
mod collectionfns;
mod comparisonops;
mod conditionals;
mod factorops;
mod groupingfns;
mod iofns;
mod lambda;
mod scopefns;
mod seqtransformfns;
mod typecastingfns;

use std::collections::HashMap;

pub use callable::{Callable, CallableResult};
pub use comparisonops::ComparisonOp;
pub use factorops::FactorOp;

macro_rules! add_fn {
    ($table: expr, $callable: path) => {
        $table.insert(String::from($callable.name()), Box::new($callable));
    };
}

pub struct CallablesTable(HashMap<String, Box<dyn Callable>>);

impl Default for CallablesTable {
    fn default() -> CallablesTable {
        let mut table: HashMap<String, Box<dyn Callable>> = HashMap::new();
        add_fn!(table, collectionfns::First);
        add_fn!(table, collectionfns::Rest);
        add_fn!(table, collectionfns::Cons);
        add_fn!(table, collectionfns::Conj);
        add_fn!(table, collectionfns::Nth);
        add_fn!(table, collectionfns::Get);
        add_fn!(table, collectionfns::Count);
        add_fn!(table, collectionfns::IsEmpty);

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

        add_fn!(table, seqtransformfns::Map);
        add_fn!(table, seqtransformfns::Filter);
        add_fn!(table, seqtransformfns::Reduce);

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
