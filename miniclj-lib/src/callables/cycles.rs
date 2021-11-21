use std::collections::HashSet;

use smol_str::SmolStr;

use crate::{
    callables::prelude::*,
    compiler::{CompilationResult, SExpr},
    instruction::Instruction,
    memaddress::Lifetime,
};

use super::scopefns::as_bindings_vector;

#[derive(Debug, Clone)]
pub struct Loop;

impl Callable for Loop {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 2 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<bindings vector> <body>",
            ))
        }
    }

    fn inner_compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let mut args_iter = args.into_iter();
        let bindings_vector_arg = args_iter.next().unwrap();
        let bindings = as_bindings_vector(self.name(), bindings_vector_arg)?;
        let mut overriden_bindings = Vec::new();

        let mut symbols = HashSet::new();
        let mut binding_addrs = Vec::new();
        for (symbol, val) in bindings {
            if let Some(overriden_addr) = state.get_symbol(&symbol) {
                overriden_bindings.push((symbol.clone(), overriden_addr));
            }
            let symbol_addr = state.new_address(Lifetime::LocalVar);
            let value_addr = state.compile(val)?;

            let mov_instruction = Instruction::new_assignment(value_addr, symbol_addr);
            state.add_instruction(mov_instruction);
            state.insert_symbol(symbol.clone(), symbol_addr);
            symbols.insert(symbol);
            binding_addrs.push(symbol_addr);
        }

        let instruction_ptr = state.instruction_ptr();
        state.push_loop_jump(instruction_ptr, binding_addrs);

        let body_arg = args_iter.next().unwrap();
        let result_addr = state.compile(body_arg)?;

        state.pop_loop_jump();

        for symbol in symbols {
            state.remove_symbol(&symbol);
        }
        for (ov_symbol, ov_address) in overriden_bindings {
            state.insert_symbol(ov_symbol, ov_address);
        }

        Ok(result_addr)
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Loop);

#[derive(Debug, Clone)]
pub struct Recur;

impl Callable for Recur {
    fn name(&self) -> &'static str {
        "recur"
    }

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        unreachable!()
    }

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let (jump_ptr, symbol_addrs) = state
            .pop_loop_jump()
            .ok_or_else(|| CompilationError::CallableNotDefined(SmolStr::from(self.name())))?;

        if args.len() != symbol_addrs.len() {
            return Err(CompilationError::WrongRecurCall(
                symbol_addrs.len(),
                args.len(),
            ));
        }

        let mut mov_instructions = Vec::new();
        for (arg, symbol_addr) in args.into_iter().zip(symbol_addrs.iter()) {
            let arg_addr = state.compile(arg)?;
            mov_instructions.push(Instruction::new_assignment(arg_addr, *symbol_addr));
        }
        for instruction in mov_instructions {
            state.add_instruction(instruction);
        }

        let goto_instruction = Instruction::new_jump(None);
        let goto_instruction_ptr = state.add_instruction(goto_instruction);
        state.fill_jump(goto_instruction_ptr, jump_ptr);

        state.push_loop_jump(jump_ptr, symbol_addrs);

        Ok(state.new_address(Lifetime::Temporal))
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Recur);
