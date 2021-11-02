use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState, SExpr},
    instruction::Instruction,
};

#[derive(Debug, Clone)]
pub struct IsTrue;

impl Callable for IsTrue {
    fn name(&self) -> &'static str {
        "true?"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 1 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::Arity(self.name(), "<value>"))
        }
    }
}

display_for_callable!(IsTrue);

#[derive(Debug, Clone)]
pub struct If;

impl Callable for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 3 {
            return Err(CompilationError::Arity(
                self.name(),
                "<condition> <true expression> <false expression>",
            ));
        }
        let mut args_iter = args.into_iter();
        let cond_arg = args_iter.next().unwrap();
        let true_arg = args_iter.next().unwrap();
        let false_arg = args_iter.next().unwrap();

        let cond_addr = IsTrue.compile(state, vec![cond_arg])?;
        let jump_on_false_ins = Instruction::new_jump(Some((false, cond_addr)));
        let jump_on_false_ins_ptr = state.add_instruction(jump_on_false_ins);

        let return_addr = state.new_tmp_address();

        let true_addr = state.compile(true_arg)?;
        let assign_true_to_return_addr_ins = Instruction::new_assignment(true_addr, return_addr);
        state.add_instruction(assign_true_to_return_addr_ins);
        let jump_ins = Instruction::new_jump(None);
        let jump_ins_ptr = state.add_instruction(jump_ins);

        state.fill_jump(jump_on_false_ins_ptr, state.instruction_ptr());
        let false_addr = state.compile(false_arg)?;
        let assign_false_to_return_addr_ins = Instruction::new_assignment(false_addr, return_addr);
        state.add_instruction(assign_false_to_return_addr_ins);
        state.fill_jump(jump_ins_ptr, state.instruction_ptr());

        Ok(return_addr)
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }
}

display_for_callable!(If);

#[derive(Debug, Clone)]
pub struct And;

impl Callable for And {
    fn name(&self) -> &'static str {
        "and"
    }

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
    }
}

display_for_callable!(And);

#[derive(Debug, Clone)]
pub struct Or;

impl Callable for Or {
    fn name(&self) -> &'static str {
        "or"
    }

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
    }
}

display_for_callable!(Or);
