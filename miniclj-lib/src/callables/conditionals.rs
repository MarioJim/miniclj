use crate::{
    callables::prelude::*,
    compiler::{CompilationResult, SExpr},
    instruction::Instruction,
    memaddress::Lifetime,
};

#[derive(Debug, Clone)]
pub struct IsTrue;

impl Callable for IsTrue {
    fn name(&self) -> &'static str {
        "true?"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 1 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), "<value>"))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if args.len() != 1 {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "one value",
                args.len(),
            ));
        }

        let val = args.get(0).unwrap();
        Ok(Value::from(val.is_truthy()))
    }
}

display_for_callable!(IsTrue);

#[derive(Debug, Clone)]
pub struct If;

impl Callable for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 3 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<condition> <true expression> <false expression>",
            ))
        }
    }

    fn inner_compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let mut args_iter = args.into_iter();
        let cond_arg = args_iter.next().unwrap();
        let true_arg = args_iter.next().unwrap();
        let false_arg = args_iter.next().unwrap();

        let cond_addr = IsTrue.compile(state, vec![cond_arg])?;
        let jump_on_false_ins = Instruction::new_jump(Some((false, cond_addr)));
        let jump_on_false_ins_ptr = state.add_instruction(jump_on_false_ins);

        let return_addr = state.new_address(Lifetime::Temporal);

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

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(If);

#[derive(Debug, Clone)]
pub struct And;

impl Callable for And {
    fn name(&self) -> &'static str {
        "and"
    }

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        for arg in args {
            if !arg.is_truthy() {
                return Ok(Value::from(false));
            }
        }
        Ok(Value::from(true))
    }
}

display_for_callable!(And);

#[derive(Debug, Clone)]
pub struct Or;

impl Callable for Or {
    fn name(&self) -> &'static str {
        "or"
    }

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        for arg in args {
            if arg.is_truthy() {
                return Ok(Value::from(true));
            }
        }
        Ok(Value::from(false))
    }
}

display_for_callable!(Or);
