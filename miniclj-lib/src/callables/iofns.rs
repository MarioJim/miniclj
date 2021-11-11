use std::io::Write;

use escape8259::unescape;

use crate::{
    callables::Callable,
    compiler::{CompilationError, CompilationResult, CompilerState},
    vm::{RuntimeError, RuntimeResult, VMState, Value},
};

fn inner_print<T: Write>(writer: &mut T, args: Vec<Value>) -> std::io::Result<()> {
    let mut args_iter = args.into_iter();
    if let Some(v) = args_iter.next() {
        if let Value::String(s) = v {
            writer.write_fmt(format_args!("{}", unescape(&s).unwrap()))?;
        } else {
            writer.write_fmt(format_args!("{}", v))?;
        }
    }
    for v in args_iter {
        if let Value::String(s) = v {
            writer.write_fmt(format_args!(" {}", unescape(&s).unwrap()))?;
        } else {
            writer.write_fmt(format_args!(" {}", v))?;
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn append_string_to_output_div(string: &str) {
    let window = web_sys::window().expect("not running in a browser environment");
    let document = window.document().expect("window should have a document");

    let output_div = document
        .get_element_by_id("output")
        .expect("'output' element not found in document");
    let new_print_row = document
        .create_element("p")
        .expect("couldn't create a p node");
    new_print_row.set_text_content(Some(string));
    output_div
        .append_child(&new_print_row)
        .expect("couldn't append the printed result to the output");
}

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 0 {
            Err(CompilationError::EmptyArgs(self.name()))
        } else {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        inner_print(&mut std::io::stdout(), args)
            .map(|()| Value::Nil)
            .map_err(|err| RuntimeError::Error(format!("Error trying to print to stdout: {}", err)))
    }

    #[cfg(target_arch = "wasm32")]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut buf = Vec::new();
        inner_print(&mut buf, args).map_err(|err| {
            RuntimeError::Error(format!("Error trying to print in a web context: {}", err))
        })?;
        append_string_to_output_div(std::str::from_utf8(&buf).unwrap());

        Ok(Value::Nil)
    }
}

display_for_callable!(Print);

#[derive(Debug, Clone)]
pub struct Println;

impl Callable for Println {
    fn name(&self) -> &'static str {
        "println"
    }

    fn find_callable_by_arity(&self, state: &mut CompilerState, _: usize) -> CompilationResult {
        Ok(state.get_callable_addr(Box::new(self.clone())))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let result = inner_print(&mut std::io::stdout(), args)
            .map(|()| Value::Nil)
            .map_err(|err| {
                RuntimeError::Error(format!("Error trying to print to stdout: {}", err))
            });
        println!();
        result
    }

    #[cfg(target_arch = "wasm32")]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut buf = Vec::new();
        inner_print(&mut buf, args).map_err(|err| {
            RuntimeError::Error(format!("Error trying to print in a web context: {}", err))
        })?;
        buf.write(b"\n").map_err(|err| {
            RuntimeError::Error(format!("Error trying to print in a web context: {}", err))
        })?;
        append_string_to_output_div(std::str::from_utf8(&buf).unwrap());

        Ok(Value::Nil)
    }
}

display_for_callable!(Println);

#[derive(Debug, Clone)]
pub struct Read;

impl Callable for Read {
    fn name(&self) -> &'static str {
        "read"
    }

    fn find_callable_by_arity(
        &self,
        state: &mut CompilerState,
        num_args: usize,
    ) -> CompilationResult {
        if num_args == 0 {
            Ok(state.get_callable_addr(Box::new(self.clone())))
        } else {
            Err(CompilationError::WrongArity(self.name(), ""))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| RuntimeError::Error(e.to_string()))?;
        Ok(Value::String(String::from(buffer.trim_end())))
    }

    #[cfg(target_arch = "wasm32")]
    fn execute(&self, _: &VMState, _: Vec<Value>) -> RuntimeResult<Value> {
        let window = web_sys::window().expect("not running in a browser environment");
        let input = window
            .prompt_with_message("Input:")
            .map_err(|e| match js_sys::JSON::stringify(&e) {
                Ok(error_str) => RuntimeError::Error(error_str.into()),
                Err(_) => RuntimeError::Error("Unknown input error".to_string()),
            })?
            .unwrap_or_else(|| String::new());

        Ok(Value::String(String::from(input.trim_end())))
    }
}

display_for_callable!(Read);
