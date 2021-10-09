use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::compiler::{
    callables::{self, Callable},
    value::Value,
};

macro_rules! add_fn {
    ($table: expr, $callable: path) => {
        $table.insert(
            String::from($callable.name()),
            Value::Fn(Box::new($callable)),
        );
    };
}

type SymbolTable = RefCell<HashMap<String, Value>>;

#[derive(Debug)]
pub enum Scope {
    RootScope(SymbolTable),
    LocalScope(SymbolTable, Rc<Scope>),
}

impl Scope {
    pub fn new(parent_scope: Option<Rc<Scope>>) -> Scope {
        if let Some(scope) = parent_scope {
            return Scope::LocalScope(Default::default(), scope);
        }
        let mut symbol_table = HashMap::new();

        add_fn!(symbol_table, callables::collectionfns::First);
        add_fn!(symbol_table, callables::collectionfns::Rest);
        add_fn!(symbol_table, callables::collectionfns::Cons);
        add_fn!(symbol_table, callables::collectionfns::Conj);
        add_fn!(symbol_table, callables::collectionfns::Get);
        add_fn!(symbol_table, callables::collectionfns::Len);
        add_fn!(symbol_table, callables::collectionfns::IsEmpty);

        add_fn!(symbol_table, callables::comparisonops::ComparisonOp::Eq);
        add_fn!(symbol_table, callables::comparisonops::ComparisonOp::Ne);
        add_fn!(symbol_table, callables::comparisonops::ComparisonOp::Gt);
        add_fn!(symbol_table, callables::comparisonops::ComparisonOp::Lt);
        add_fn!(symbol_table, callables::comparisonops::ComparisonOp::Ge);
        add_fn!(symbol_table, callables::comparisonops::ComparisonOp::Le);

        add_fn!(symbol_table, callables::conditionals::IsTrue);
        add_fn!(symbol_table, callables::conditionals::If);
        add_fn!(symbol_table, callables::conditionals::And);
        add_fn!(symbol_table, callables::conditionals::Or);

        add_fn!(symbol_table, callables::factorops::FactorOp::Add);
        add_fn!(symbol_table, callables::factorops::FactorOp::Sub);
        add_fn!(symbol_table, callables::factorops::FactorOp::Mul);
        add_fn!(symbol_table, callables::factorops::FactorOp::Div);

        add_fn!(symbol_table, callables::groupingfns::Do);

        add_fn!(symbol_table, callables::iofns::Print);
        add_fn!(symbol_table, callables::iofns::Read);

        add_fn!(symbol_table, callables::lambdafns::AnonymousFn);
        // callables::lambdafns::AnonymousLambdaFn can't be called directly, just constructed

        add_fn!(symbol_table, callables::scopefns::Def);
        add_fn!(symbol_table, callables::scopefns::Defn);
        add_fn!(symbol_table, callables::scopefns::Let);
        add_fn!(symbol_table, callables::scopefns::Loop);
        // callables::scopefns::Recur only available inside a Loop scope

        add_fn!(symbol_table, callables::seqtransformfns::Map);
        add_fn!(symbol_table, callables::seqtransformfns::Filter);
        add_fn!(symbol_table, callables::seqtransformfns::Reduce);

        add_fn!(symbol_table, callables::typecastingfns::NumberCast);
        add_fn!(symbol_table, callables::typecastingfns::StringCast);
        add_fn!(symbol_table, callables::typecastingfns::Ord);
        add_fn!(symbol_table, callables::typecastingfns::Chr);

        Scope::RootScope(RefCell::new(symbol_table))
    }

    pub fn get(&self, symbol: &str) -> Option<Value> {
        match self {
            Scope::RootScope(table) => table.borrow().get(symbol).cloned(),
            Scope::LocalScope(table, top_scope) => table
                .borrow()
                .get(symbol)
                .cloned()
                .or_else(|| top_scope.get(symbol)),
        }
    }

    pub fn insert(&self, symbol: String, value: Value) {
        match self {
            Scope::RootScope(t) => t,
            Scope::LocalScope(t, _) => t,
        }
        .borrow_mut()
        .insert(symbol, value);
    }

    pub fn insert_in_root(&self, symbol: String, value: Value) {
        match self {
            Scope::LocalScope(_, s) => s.insert_in_root(symbol, value),
            Scope::RootScope(_) => self.insert(symbol, value),
        };
    }
}
