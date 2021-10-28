use lalrpop_util::lalrpop_mod;

lalrpop_mod!(#[allow(dead_code)] #[allow(clippy::all)] pub parser);
mod cli;
mod compiler;
mod vm;

use crate::cli::{output_file_from_opts, read_file_from_opts};

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
        ("build", opts) => {
            let input = read_file_from_opts(opts)?;
            let output_file = output_file_from_opts(opts)?;
            let tree = parser::SExprsParser::new()
                .parse(&input)
                .map_err(|e| format!("{:#?}", e))?;

            let mut compiler_state = compiler::State::default();
            for expr in tree {
                compiler_state
                    .compile(expr)
                    .map_err(|err| format!("Compilation error: {}", err))?;
            }
            println!("{:#?}", compiler_state);
            // compiler_state.write_to(output_file);
        }
        ("exec", opts) => {
            let input = read_file_from_opts(opts)?;
            let mut vm_state =
                vm::State::try_from_string(input).map_err(|e| format!("Execution error: {}", e))?;
            vm_state.execute();
        }
        ("run", opts) => {
            let input = read_file_from_opts(opts)?;
            let tree = parser::SExprsParser::new()
                .parse(&input)
                .map_err(|e| format!("{:#?}", e))?;

            let mut compiler_state = compiler::State::default();
            for expr in tree {
                compiler_state
                    .compile(expr)
                    .map_err(|err| format!("Compilation error: {}", err))?;
            }

            let mut vm_state = vm::State::from_compiler_state(compiler_state);
            vm_state.execute();
        }
        (_, _) => unreachable!(),
    }

    Ok(())
}
