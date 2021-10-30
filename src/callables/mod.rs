macro_rules! display_for_callable {
    ($callable:ty) => {
        impl std::fmt::Display for $callable {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}

pub mod collectionfns;
pub mod comparisonops;
pub mod conditionals;
pub mod factorops;
pub mod groupingfns;
pub mod iofns;
pub mod scopefns;
pub mod seqtransformfns;
pub mod typecastingfns;

pub trait Callable: std::fmt::Display + std::fmt::Debug + dyn_clone::DynClone {
    fn name(&self) -> &'static str;

    fn compile(
        &self,
        state: &mut crate::compiler::State,
        args: Vec<crate::compiler::SExpr>,
    ) -> crate::compiler::CompilationResult;

    fn arity_err(&self, expected: &'static str) -> crate::compiler::CompilationResult {
        Err(crate::compiler::CompilationError::Arity(
            self.name(),
            expected,
        ))
    }
}

dyn_clone::clone_trait_object!(Callable);
