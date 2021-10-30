use crate::{
    callables::Callable,
    compiler::{CompilationResult, SExpr, State},
    instruction::Instruction,
    memaddress::DataType,
};

#[derive(Debug, Clone)]
pub struct IsTrue;

impl Callable for IsTrue {
    fn name(&self) -> &'static str {
        "true?"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<value>");
        }
        let arg = args.into_iter().next().unwrap();
        let arg_addr = state.compile(arg)?;
        let res_addr = state.new_tmp_address(DataType::Number);

        let instruction = Instruction::new_builtin_call(self.name(), vec![arg_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(IsTrue);

#[derive(Debug, Clone)]
pub struct If;

impl Callable for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 3 {
            return self.arity_err("<condition> <true expression> <false expression>");
        }
        let mut args_iter = args.into_iter();
        let cond_arg = args_iter.next().unwrap();
        let true_arg = args_iter.next().unwrap();
        let false_arg = args_iter.next().unwrap();

        let cond_addr = IsTrue.compile(state, vec![cond_arg])?;
        let jump_on_false_ins = Instruction::new_jump(Some((false, cond_addr)));
        let jump_on_false_ins_ptr = state.add_instruction(jump_on_false_ins);

        let return_addr = state.new_tmp_address(DataType::Unknown);

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
}

display_for_callable!(If);

#[derive(Debug, Clone)]
pub struct And;

impl Callable for And {
    fn name(&self) -> &'static str {
        "and"
    }

    fn compile(&self, _state: &mut State, _args: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(And);

#[derive(Debug, Clone)]
pub struct Or;

impl Callable for Or {
    fn name(&self) -> &'static str {
        "or"
    }

    fn compile(&self, _state: &mut State, _args: Vec<SExpr>) -> CompilationResult {
        todo!()
    }
}

display_for_callable!(Or);
