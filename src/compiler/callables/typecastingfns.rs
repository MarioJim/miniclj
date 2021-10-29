use crate::compiler::{
    callables::{Callable, CompilationResult},
    DataType, Instruction, SExpr, State,
};

#[derive(Debug, Clone)]
pub struct NumberCast;

impl Callable for NumberCast {
    fn name(&self) -> &'static str {
        "num"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        let arg = args.into_iter().next().unwrap();
        let arg_addr = state.compile(arg)?;
        let res_addr = state.new_tmp_address(DataType::Number);

        let instruction = Instruction::new_builtin_call(self.name(), vec![arg_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(NumberCast);

#[derive(Debug, Clone)]
pub struct StringCast;

impl Callable for StringCast {
    fn name(&self) -> &'static str {
        "str"
    }

    fn compile(&self, _state: &mut State, _args: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(StringCast);

#[derive(Debug, Clone)]
pub struct Ord;

impl Callable for Ord {
    fn name(&self) -> &'static str {
        "ord"
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<string>");
        }
        todo!()
    }
}

display_for_callable!(Ord);

#[derive(Debug, Clone)]
pub struct Chr;

impl Callable for Chr {
    fn name(&self) -> &'static str {
        "chr"
    }

    fn compile(&self, _state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<number>");
        }
        todo!()
    }
}

display_for_callable!(Chr);
