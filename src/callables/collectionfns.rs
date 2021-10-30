use crate::{
    callables::Callable,
    compiler::{CompilationResult, SExpr, State},
    instruction::Instruction,
    memaddress::DataType,
};

#[derive(Debug, Clone)]
pub struct First;

impl Callable for First {
    fn name(&self) -> &'static str {
        "first"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let arg = args.into_iter().next().unwrap();
        let arg_addr = state.compile(arg)?;
        let res_addr = state.new_tmp_address(DataType::Unknown);

        let instruction = Instruction::new_builtin_call(self.name(), vec![arg_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(First);

#[derive(Debug, Clone)]
pub struct Rest;

impl Callable for Rest {
    fn name(&self) -> &'static str {
        "rest"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let arg = args.into_iter().next().unwrap();
        let arg_addr = state.compile(arg)?;
        let res_addr = state.new_tmp_address(DataType::List);

        let instruction = Instruction::new_builtin_call(self.name(), vec![arg_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Rest);

#[derive(Debug, Clone)]
pub struct Cons;

impl Callable for Cons {
    fn name(&self) -> &'static str {
        "cons"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<value> <collection>");
        }
        let mut args_iter = args.into_iter();
        let value_arg = args_iter.next().unwrap();
        let coll_arg = args_iter.next().unwrap();

        let value_addr = state.compile(value_arg)?;
        let coll_addr = state.compile(coll_arg)?;

        let res_addr = state.new_tmp_address(DataType::List);
        let instruction =
            Instruction::new_builtin_call(self.name(), vec![value_addr, coll_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Cons);

#[derive(Debug, Clone)]
pub struct Conj;

impl Callable for Conj {
    fn name(&self) -> &'static str {
        "conj"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<collection> <value>");
        }
        let mut args_iter = args.into_iter();
        let coll_arg = args_iter.next().unwrap();
        let value_arg = args_iter.next().unwrap();

        let coll_addr = state.compile(coll_arg)?;
        let value_addr = state.compile(value_arg)?;

        let res_addr = state.new_tmp_address(DataType::Unknown);
        let instruction =
            Instruction::new_builtin_call(self.name(), vec![coll_addr, value_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Conj);

#[derive(Debug, Clone)]
pub struct Get;

impl Callable for Get {
    fn name(&self) -> &'static str {
        "get"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 2 {
            return self.arity_err("<collection> <key>");
        }
        let mut args_iter = args.into_iter();
        let coll_arg = args_iter.next().unwrap();
        let key_arg = args_iter.next().unwrap();

        let coll_addr = state.compile(coll_arg)?;
        let key_addr = state.compile(key_arg)?;

        let res_addr = state.new_tmp_address(DataType::Unknown);
        let instruction =
            Instruction::new_builtin_call(self.name(), vec![coll_addr, key_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Get);

#[derive(Debug, Clone)]
pub struct Len;

impl Callable for Len {
    fn name(&self) -> &'static str {
        "len"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let arg = args.into_iter().next().unwrap();
        let arg_addr = state.compile(arg)?;
        let res_addr = state.new_tmp_address(DataType::Number);

        let instruction = Instruction::new_builtin_call(self.name(), vec![arg_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(Len);

#[derive(Debug, Clone)]
pub struct IsEmpty;

impl Callable for IsEmpty {
    fn name(&self) -> &'static str {
        "empty?"
    }

    fn compile(&self, state: &mut State, args: Vec<SExpr>) -> CompilationResult {
        if args.len() != 1 {
            return self.arity_err("<collection>");
        }
        let arg = args.into_iter().next().unwrap();
        let arg_addr = state.compile(arg)?;
        let res_addr = state.new_tmp_address(DataType::Number);

        let instruction = Instruction::new_builtin_call(self.name(), vec![arg_addr], res_addr);
        state.add_instruction(instruction);

        Ok(res_addr)
    }
}

display_for_callable!(IsEmpty);
