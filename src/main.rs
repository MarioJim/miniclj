use std::{env::args, fs::read_to_string};

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(#[allow(clippy::all)] pub miniclj);
mod callables;
mod scope;
mod sexpr;
mod value;

fn main() -> Result<(), String> {
    let mut args = args();
    let third_arg = args
        .nth(1)
        .ok_or_else(|| String::from("Expected path to input file as first argument"))?;

    let input = read_to_string(third_arg).unwrap();
    let syntax_tree = miniclj::SExprsParser::new()
        .parse(&input)
        .map_err(|e| format!("{:#?}", e))?;
    let scope = scope::Scope::new(None);

    for expr in syntax_tree {
        println!("{}", expr.eval(&scope));
    }

    Ok(())
}
