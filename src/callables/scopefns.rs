use crate::{
    callables::{Callable, CallableResult},
    compiler::{CompilationError, CompilationResult, CompilerState, Literal, SExpr},
    constant::Constant,
    instruction::Instruction,
    vm::{RuntimeError, VMState, Value},
};

#[derive(Debug, Clone)]
pub struct Def;

impl Callable for Def {
    fn name(&self) -> &'static str {
        "def"
    }

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return Err(CompilationError::Arity(self.name(), "<symbol> <value>"));
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
        if state.has_symbol_in_symtbl(&symbol) {
            Err(CompilationError::SymbolAlreadyDefined(symbol))
        } else {
            let value_addr = state.compile(value_arg)?;
            state.insert_in_root_symtbl(symbol, value_addr);
            Ok(value_addr)
        }
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> CallableResult {
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
            return Err(CompilationError::Arity(
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
        if state.has_symbol_in_symtbl(&symbol) {
            return Err(CompilationError::SymbolAlreadyDefined(symbol));
        }

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
        let lambda_addr = state.insert_in_consttbl(lambda_const);
        state.insert_in_root_symtbl(symbol, lambda_addr);

        state.compile_lambda(arg_names, body_arg)?;
        state.fill_jump(jump_lambda_instr_ptr, state.instruction_ptr());

        Ok(lambda_addr)
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> CallableResult {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Defn);

#[derive(Debug, Clone)]
pub struct Let;

impl Callable for Let {
    fn name(&self) -> &'static str {
        "let"
    }

    fn compile(&self, _state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return Err(CompilationError::Arity(
                self.name(),
                "<bindings vector> <body>",
            ));
        }

        let first_arg_error = Err(CompilationError::Error(format!(
            "First argument of {} must be a vector with symbol - value pairs",
            self.name()
        )));

        let mut args_iter = args.into_iter();
        let _bindings_vector = if let SExpr::Vector(v) = args_iter.next().unwrap() {
            v
        } else {
            return first_arg_error;
        };

        todo!()
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> CallableResult {
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

    fn compile(&self, _: &mut CompilerState, _: Vec<SExpr>) -> CompilationResult {
        todo!()
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> CallableResult {
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

    fn compile(&self, _: &mut CompilerState, _: Vec<SExpr>) -> CompilationResult {
        todo!()
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }

    fn execute(&self, _: &VMState, _: Vec<Value>) -> CallableResult {
        Err(RuntimeError::CompilerError(format!(
            "Compiler shouldn't output \"{}\" calls",
            self.name()
        )))
    }
}

display_for_callable!(Recur);
