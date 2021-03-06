use std::io::Write;

use escape8259::unescape;

use crate::callables::prelude::*;

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
            match unescape(&s) {
                Ok(a) => writer.write_fmt(format_args!(" {}", a))?,
                Err(e) => {
                    println!("{:?}", e);
                    writer.write_all(b"")?;
                }
            }
        } else {
            writer.write_fmt(format_args!(" {}", v))?;
        }
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct MinicljOutputWindow {
    pub minicljoutput: String,
}

#[cfg(target_arch = "wasm32")]
fn append_string_to_output_div(new_output: &str) {
    use js_sys::{JsString, Object};
    use wasm_bindgen::prelude::*;

    let window = web_sys::window().expect("not running in a browser environment");

    let prev_output_obj = window
        .get("minicljoutput")
        .unwrap_or_else(|| JsString::from(String::new()).into());
    let prev_output_jsstr = prev_output_obj.to_string();
    let prev_output_string = String::from(prev_output_jsstr);
    let minicljoutput = prev_output_string + new_output;
    let output = MinicljOutputWindow { minicljoutput };

    let output_obj = Object::from(JsValue::from_serde(&output).unwrap());
    Object::assign(&window, &output_obj);
}

#[derive(Debug, Clone)]
pub struct Print;

impl Callable for Print {
    fn name(&self) -> &'static str {
        "print"
    }

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        inner_print(&mut std::io::stdout(), args)
            .map(|()| Value::Nil)
            .map_err(|err| RuntimeError::IOError("print to stdout", err))
    }

    #[cfg(target_arch = "wasm32")]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut buf = Vec::new();
        inner_print(&mut buf, args)
            .map_err(|err| RuntimeError::IOError("print in a web context", err))?;
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

    fn check_arity(&self, _: usize) -> Result<(), CompilationError> {
        Ok(())
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let result = inner_print(&mut std::io::stdout(), args)
            .map(|()| Value::Nil)
            .map_err(|err| RuntimeError::IOError("print to stdout", err));
        println!();
        result
    }

    #[cfg(target_arch = "wasm32")]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        let mut buf = Vec::new();
        inner_print(&mut buf, args)
            .map_err(|err| RuntimeError::IOError("print in a web context", err))?;
        buf.write(b"\n")
            .map_err(|err| RuntimeError::IOError("print in a web context", err))?;
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

    fn check_arity(&self, num_args: usize) -> Result<(), CompilationError> {
        if num_args == 0 {
            Ok(())
        } else {
            Err(CompilationError::WrongArity(self.name(), ""))
        }
    }

    fn get_as_address(&self, state: &mut CompilerState) -> Option<MemAddress> {
        Some(state.get_callable_addr(Box::new(self.clone())))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        if !args.is_empty() {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "no arguments",
                args.len(),
            ));
        }

        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| RuntimeError::IOError("read from stdin", e))?;
        Ok(Value::String(String::from(buffer.trim())))
    }

    #[cfg(target_arch = "wasm32")]
    fn execute(&self, _: &VMState, args: Vec<Value>) -> RuntimeResult<Value> {
        use std::io::{Error, ErrorKind};

        if !args.is_empty() {
            return Err(RuntimeError::WrongArityS(
                self.name(),
                "no arguments",
                args.len(),
            ));
        }

        let window = web_sys::window().expect("not running in a browser environment");
        let input = window
            .prompt_with_message("Input:")
            .map_err(|_| {
                RuntimeError::IOError("read in a web context", Error::from(ErrorKind::Unsupported))
            })?
            .unwrap_or_else(|| String::new());

        Ok(Value::String(String::from(input.trim())))
    }
}

display_for_callable!(Read);
