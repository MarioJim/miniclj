use miniclj_lib::{CompilerState, SExprsParser, VMState};
use wasm_bindgen::prelude::*;

mod result;
mod utils;

use result::JSResult;
use utils::set_panic_hook;

/// Prints the abstract syntaxt tree of the miniclj code
#[wasm_bindgen]
pub fn ast(code: &str) -> JsValue {
    set_panic_hook();

    match SExprsParser::new().parse(code) {
        Ok(trees) => JSResult::output(
            trees
                .into_iter()
                .map(|tree| format!("{:#?}\n", tree))
                .collect::<String>(),
        ),
        Err(err) => JSResult::error(format!("{:#?}", err)),
    }
}

/// Compiles miniclj code
/// Outputs the bytecode corresponding to the code
#[wasm_bindgen]
pub fn compile(code: &str) -> JsValue {
    set_panic_hook();

    let tree = match SExprsParser::new().parse(code) {
        Ok(tree) => tree,
        Err(err) => return JSResult::error(format!("{:#?}", err)),
    };

    let mut compiler_state = CompilerState::default();
    for expr in tree {
        if let Err(err) = compiler_state.compile(expr) {
            return JSResult::error(format!("Compilation error: {}", err));
        }
    }

    let mut buf = Vec::new();

    if let Err(err) = compiler_state.write_to(&mut buf) {
        return JSResult::error(format!("Output error: {}", err));
    }

    JSResult::output(std::str::from_utf8(buf.as_slice()).unwrap().to_string())
}

/// Executes miniclj code
/// - read instructions are executed as window.prompt calls
/// - print and println instructions append a new `p` node
///   to an `#output` node in the document's body
#[wasm_bindgen]
pub fn run(code: &str) -> JsValue {
    set_panic_hook();

    let tree = match SExprsParser::new().parse(code) {
        Ok(tree) => tree,
        Err(err) => return JSResult::error(format!("{:#?}", err)),
    };

    let mut compiler_state = CompilerState::default();
    for expr in tree {
        if let Err(err) = compiler_state.compile(expr) {
            return JSResult::error(format!("Compilation error: {}", err));
        }
    }

    let (constants_rev, instructions) = compiler_state.into_parts();
    let constants = constants_rev
        .into_iter()
        .map(|(constant, address)| (address, constant))
        .collect();

    if let Err(err) = VMState::new(constants, instructions).execute() {
        return JSResult::error(format!("Runtime error: {}", err));
    }

    JSResult::output(String::from("Correct compilation"))
}
