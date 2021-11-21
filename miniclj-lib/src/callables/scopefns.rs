use std::collections::HashSet;

use smol_str::SmolStr;

use crate::{
    callables::prelude::*,
    compiler::{CompilationResult, Literal, SExpr},
    constant::Constant,
    instruction::Instruction,
    memaddress::Lifetime,
};

#[derive(Debug, Clone)]
pub struct Def;

impl Callable for Def {
    fn name(&self) -> &'static str {
        "def"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 2 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<symbol> <value>",
            ))
        }
    }

    fn inner_compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let mut args_iter = args.into_iter();
        let symbol_arg = args_iter.next().unwrap();
        let value_arg = args_iter.next().unwrap();

        let symbol = if let SExpr::Literal(Literal::Symbol(symbol)) = symbol_arg {
            Ok(symbol)
        } else {
            Err(CompilationError::WrongArgument(
                self.name(),
                "a symbol",
                symbol_arg.type_str(),
            ))
        }?;

        let value_addr = state.compile(value_arg)?;
        let global_val_addr = state.new_address(Lifetime::GlobalVar);
        let mov_instruction = Instruction::new_assignment(value_addr, global_val_addr);
        state.add_instruction(mov_instruction);

        state.insert_symbol(symbol, global_val_addr);
        Ok(value_addr)
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Def);

#[derive(Debug, Clone)]
pub struct Defn;

impl Callable for Defn {
    fn name(&self) -> &'static str {
        "defn"
    }

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 3 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(
                self.name(),
                "<symbol> <args vector> <body>",
            ))
        }
    }

    fn inner_compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let mut args_iter = args.into_iter();
        let symbol_arg = args_iter.next().unwrap();
        let args_vec_arg = args_iter.next().unwrap();
        let body_arg = args_iter.next().unwrap();

        let symbol = if let SExpr::Literal(Literal::Symbol(symbol)) = symbol_arg {
            Ok(symbol)
        } else {
            Err(CompilationError::WrongArgument(
                self.name(),
                "a symbol",
                symbol_arg.type_str(),
            ))
        }?;

        let arg_names = if let SExpr::Vector(vector) = args_vec_arg {
            vector
                .into_iter()
                .map(|expr| {
                    if let SExpr::Literal(Literal::Symbol(arg_name)) = expr {
                        Ok(arg_name)
                    } else {
                        Err(CompilationError::WrongArgument(
                            self.name(),
                            "a vector of symbols",
                            "a vector of something else",
                        ))
                    }
                })
                .collect::<Result<Vec<SmolStr>, CompilationError>>()
        } else {
            Err(CompilationError::WrongArgument(
                self.name(),
                "a vector of symbols",
                args_vec_arg.type_str(),
            ))
        }?;

        // + 2 because first there is the mov to global addr and then the jump lambda
        let lambda_start_ptr = state.instruction_ptr() + 2;
        let lambda_const = Constant::new_lambda(lambda_start_ptr, arg_names.len());
        let lambda_const_addr = state.insert_constant(lambda_const);

        let lambda_global_addr = state.new_address(Lifetime::GlobalVar);
        let mov_instruction = Instruction::new_assignment(lambda_const_addr, lambda_global_addr);
        state.add_instruction(mov_instruction);
        state.insert_symbol(symbol, lambda_global_addr);

        let jump_lambda_instr = Instruction::new_jump(None);
        let jump_lambda_instr_ptr = state.add_instruction(jump_lambda_instr);
        state.compile_lambda(arg_names, body_arg)?;
        state.fill_jump(jump_lambda_instr_ptr, state.instruction_ptr());

        Ok(lambda_global_addr)
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Defn);

pub fn as_bindings_vector(
    fn_name: &'static str,
    expr: SExpr,
) -> Result<Vec<(SmolStr, SExpr)>, CompilationError> {
    let bindings_vector = match expr {
        SExpr::Vector(vector) if vector.len() % 2 == 0 => Ok(vector),
        other => Err(CompilationError::WrongArgument(
            fn_name,
            "a vector of symbol-value pairs",
            other.type_str(),
        )),
    }?;

    let mut bindings_iter = bindings_vector.into_iter();
    let mut result = Vec::new();

    while let Some(key) = bindings_iter.next() {
        let symbol = match key {
            SExpr::Literal(Literal::Symbol(symbol)) => Ok(symbol),
            _ => Err(CompilationError::WrongArgument(
                fn_name,
                "a vector of symbol-value pairs",
                "a vector with something other than symbols in odd positions",
            )),
        }?;

        let val = bindings_iter.next().unwrap();
        result.push((symbol, val));
    }

    Ok(result)
}

#[derive(Debug, Clone)]
pub struct Let;

impl Callable for Let {
    fn name(&self) -> &'static str {
        "let"
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
        }

        let body_arg = args_iter.next().unwrap();
        let result_addr = state.compile(body_arg)?;

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

display_for_callable!(Let);
