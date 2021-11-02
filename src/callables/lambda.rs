use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState, Literal, SExpr},
    constant::Constant,
    instruction::Instruction,
};

#[derive(Debug, Clone)]
pub struct Lambda;

impl Callable for Lambda {
    fn name(&self) -> &'static str {
        "fn"
    }

    fn compile(&self, state: &mut CompilerState, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return Err(CompilationError::Arity(self.name(), "<args vector> <body>"));
        }

        let mut args_iter = args.into_iter();
        let args_vec_arg = args_iter.next().unwrap();
        let body_arg = args_iter.next().unwrap();

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

        state.compile_lambda(arg_names, body_arg)?;
        state.fill_jump(jump_lambda_instr_ptr, state.instruction_ptr());
        Ok(lambda_addr)
    }

    fn find_callable_by_arity(&self, _: &mut CompilerState, _: usize) -> CompilationResult {
        unimplemented!()
    }
}

display_for_callable!(Lambda);
