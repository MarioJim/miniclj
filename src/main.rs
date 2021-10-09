use std::rc::Rc;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(#[allow(dead_code)] #[allow(clippy::all)] pub parser);
mod cli;
mod compiler;

use crate::{cli::read_file_from_opts, compiler::Scope};

fn main() -> Result<(), String> {
    match cli::args().get_matches().subcommand().unwrap() {
        ("check", opts) => {
            let input = read_file_from_opts(opts)?;
            if let Err(err) = parser::SExprsParser::new().parse(&input) {
                println!("{:#?}", err)
            }
        }
        ("ast", opts) => {
            let input = read_file_from_opts(opts)?;
            match parser::SExprsParser::new().parse(&input) {
                Ok(tree) => println!("{:#?}", tree),
                Err(err) => println!("{:#?}", err),
            }
        }
        ("build", _) => todo!(),
        ("exec", _) => todo!(),
        ("run", opts) => {
            let input = read_file_from_opts(opts)?;
            let tree = parser::SExprsParser::new()
                .parse(&input)
                .map_err(|e| format!("{:#?}", e))?;

            let scope = Rc::new(Scope::new(None));

            for expr in tree {
                println!("{:#?}", expr.eval(&scope));
            }
        }
        (_, _) => unreachable!(),
    }

    Ok(())
}
