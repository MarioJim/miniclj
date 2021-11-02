use lalrpop_util::lalrpop_mod;

lalrpop_mod!(#[allow(clippy::all)] pub lispparser);
lalrpop_mod!(#[allow(clippy::all)] pub bytecodeparser);
mod callables;
mod cli;
mod compiler;
mod constant;
mod instruction;
mod memaddress;
mod vm;

use crate::cli::{args, output_file_from_opts, read_file_from_opts};

fn main() -> Result<(), String> {
    let start_time = std::time::Instant::now();

    match args().get_matches().subcommand().unwrap() {
        ("check", opts) => {
            let input = read_file_from_opts(opts)?;
            if let Err(err) = lispparser::SExprsParser::new().parse(&input) {
                println!("{:#?}", err)
            }
        }
        ("ast", opts) => {
            let input = read_file_from_opts(opts)?;
            match lispparser::SExprsParser::new().parse(&input) {
                Ok(tree) => println!("{:#?}", tree),
                Err(err) => println!("{:#?}", err),
            }
        }
        ("build", opts) => {
            let input = read_file_from_opts(opts)?;
            let mut output_file = output_file_from_opts(opts)?;
            let tree = lispparser::SExprsParser::new()
                .parse(&input)
                .map_err(|e| format!("{:#?}", e))?;

            let mut compiler_state = compiler::CompilerState::default();
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
            let callables_table = callables::CallablesTable::default();

            let (constants, instructions) = bytecodeparser::BytecodeParser::new()
                .parse(&callables_table, &input)
                .map_err(|e| format!("Bytecode error: {}", e))?;

            vm::VMState::new(constants, instructions)
                .execute()
                .map_err(|err| format!("Runtime error: {}", err))?
        }
        ("run", opts) => {
            let input = read_file_from_opts(opts)?;
            let tree = lispparser::SExprsParser::new()
                .parse(&input)
                .map_err(|e| format!("{:#?}", e))?;

            let mut compiler_state = compiler::CompilerState::default();
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

            vm::VMState::new(constants, instructions)
                .execute()
                .map_err(|err| format!("Runtime error: {}", err))?
        }
        (_, _) => unreachable!(),
    }

    let execution_time = start_time.elapsed();
    println!("Finished in {}ms", execution_time.as_millis());

    Ok(())
}
