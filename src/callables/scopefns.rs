use std::collections::HashSet;

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState, Literal, SExpr},
    constant::Constant,
    instruction::Instruction,
    memaddress::Lifetime,
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

#[derive(Debug, Clone)]
pub struct Def;

impl Callable for Def {
    fn name(&self) -> &'static str {
        "def"
    }

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return Err(CompilationError::WrongArity(
                self.name(),
                "<symbol> <value>",
            ));
        }
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

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
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

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 3 {
            return Err(CompilationError::WrongArity(
                self.name(),
                "<symbol> <args vector> <body>",
            ));
        }

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
                .collect::<Result<Vec<String>, CompilationError>>()
        } else {
            Err(CompilationError::WrongArgument(
                self.name(),
                "a vector of symbols",
                args_vec_arg.type_str(),
            ))
        }?;

        let jump_lambda_instr = Instruction::new_jump(None);
        let jump_lambda_instr_ptr = state.add_instruction(jump_lambda_instr);
        let lambda_start_ptr = state.instruction_ptr();
        let lambda_const = Constant::new_lambda(lambda_start_ptr, arg_names.len());
        let lambda_addr = state.insert_constant(lambda_const);

        let global_val_addr = state.new_address(Lifetime::GlobalVar);
        let mov_instruction = Instruction::new_assignment(lambda_addr, global_val_addr);
        state.add_instruction(mov_instruction);
        state.insert_symbol(symbol, global_val_addr);

        state.compile_lambda(arg_names, body_arg)?;
        state.fill_jump(jump_lambda_instr_ptr, state.instruction_ptr());

        Ok(lambda_addr)
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Defn);

fn as_bindings_vector(
    fn_name: &'static str,
    expr: SExpr,
) -> Result<Vec<(String, SExpr)>, CompilationError> {
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

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return Err(CompilationError::WrongArity(
                self.name(),
                "<bindings vector> <body>",
            ));
        }

        let mut args_iter = args.into_iter();
        let bindings_vector_arg = args_iter.next().unwrap();
        let bindings = as_bindings_vector(self.name(), bindings_vector_arg)?;

        let mut symbols = HashSet::new();
        for (symbol, val) in bindings {
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

        Ok(result_addr)
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Let);

#[derive(Debug, Clone)]
pub struct Loop;

impl Callable for Loop {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return Err(CompilationError::WrongArity(
                self.name(),
                "<bindings vector> <body>",
            ));
        }

        let mut args_iter = args.into_iter();
        let bindings_vector_arg = args_iter.next().unwrap();
        let bindings = as_bindings_vector(self.name(), bindings_vector_arg)?;

        let mut symbols = HashSet::new();
        let mut binding_addrs = Vec::new();
        for (symbol, val) in bindings {
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

        Ok(result_addr)
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
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

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        let (jump_ptr, symbol_addrs) = state
            .pop_loop_jump()
            .ok_or_else(|| CompilationError::CallableNotDefined(String::from(self.name())))?;

        if args.len() != symbol_addrs.len() {
            return Err(CompilationError::WrongRecurCall(
                symbol_addrs.len(),
                args.len(),
            ));
        }

        for (arg, symbol_addr) in args.into_iter().zip(symbol_addrs.iter()) {
            let arg_addr = state.compile(arg)?;

            let mov_instruction = Instruction::new_assignment(arg_addr, *symbol_addr);
            state.add_instruction(mov_instruction);
        }

        let goto_instruction = Instruction::new_jump(None);
        let goto_instruction_ptr = state.add_instruction(goto_instruction);
        state.fill_jump(goto_instruction_ptr, jump_ptr);

        state.push_loop_jump(jump_ptr, symbol_addrs);

        Ok(state.new_address(Lifetime::Temporal))
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Recur);
