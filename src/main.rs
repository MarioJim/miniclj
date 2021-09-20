use std::env::args;
use std::fs::read_to_string;

use lalrpop_util::lalrpop_mod;
use regex::Regex;

lalrpop_mod!(pub miniclj);
mod ast;
mod value;

fn main() -> Result<(), &'static str> {
    let r = Regex::new("^$").unwrap();
    for v in &["a", "?", "a+sd'a*s>d<!?", "as\n", "0asdf", "sdf,asdf"] {
        println!("{} {}", v, r.is_match(v));
    }
    let mut args = args();
    let third_arg = args
        .nth(1)
        .ok_or("Expected path to input file as first argument")?;

    let input = read_to_string(third_arg).unwrap();
    match miniclj::SExprsParser::new().parse(&input) {
        Ok(exprs) => {
            for expr in exprs {
                println!("{}", expr)
            }
        }
        Err(e) => println!("{:#?}", e),
    }
    Ok(())
}
