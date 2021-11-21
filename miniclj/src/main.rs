use miniclj_lib::{BytecodeParser, CompilerState, SExprsParser, VMState};

/// This module exposes the `clap` `App` used to parse arguments
/// passed through the command-line interface, and shared
/// functionality between multiple subcommands
mod cli;

use crate::cli::{args, output_file_from_opts, read_file_from_opts};

/// The entry point for the command-line interface
fn main() -> Result<(), String> {
    let start_time = std::time::Instant::now();

    match args().get_matches().subcommand().unwrap() {
        ("check", opts) => {
            let input = read_file_from_opts(opts)?;
            if let Err(err) = SExprsParser::parse(&input) {
                println!("{}", err);
            }
        }
        ("ast", opts) => {
            let input = read_file_from_opts(opts)?;
            match SExprsParser::parse(&input) {
                Ok(tree) => println!("{:#?}", tree),
                Err(err) => println!("{}", err),
            }
        }
        ("build", opts) => {
            let input = read_file_from_opts(opts)?;
            let mut output_file = output_file_from_opts(opts)?;
            let tree = SExprsParser::parse(&input).map_err(|e| format!("{}", e))?;

            let mut compiler_state = CompilerState::default();
            for expr in tree {
                compiler_state
                    .compile(expr)
                    .map_err(|err| format!("Compilation error: {}", err))?;
            }
            println!("{:#?}", compiler_state);
            compiler_state
                .write_to(&mut output_file)
                .map_err(|err| format!("File error: {}", err))?;
        }
        ("exec", opts) => {
            let input = read_file_from_opts(opts)?;
            let (constants, instructions) =
                BytecodeParser::parse(&input).map_err(|e| format!("Bytecode error: {}", e))?;

            VMState::new(constants, instructions)
                .execute()
                .map_err(|err| format!("Runtime error: {}", err))?;
        }
        ("run", opts) => {
            let input = read_file_from_opts(opts)?;
            let tree = SExprsParser::parse(&input).map_err(|e| format!("{}", e))?;

            let mut compiler_state = CompilerState::default();
            for expr in tree {
                compiler_state
                    .compile(expr)
                    .map_err(|err| format!("Compilation error: {}", err))?;
            }

            let (constants_rev, instructions) = compiler_state.into_parts();
            let constants = constants_rev
                .into_iter()
                .map(|(constant, address)| (address, constant))
                .collect();

            VMState::new(constants, instructions)
                .execute()
                .map_err(|err| format!("Runtime error: {}", err))?;
        }
        (_, _) => unreachable!(),
    }

    let execution_time = start_time.elapsed();
    println!("Finished in {}ms", execution_time.as_millis());

    Ok(())
}
