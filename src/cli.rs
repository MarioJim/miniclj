use std::{
    fs::{read_to_string, File},
    path::PathBuf,
    str::FromStr,
};

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};

pub fn args() -> App<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("check")
                .about("Checks if a source code file can be correctly parsed")
                .arg(
                    Arg::new("FILE")
                        .about("File to check")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("ast")
                .about("Prints the abstract syntax tree from a source code file")
                .arg(
                    Arg::new("FILE")
                        .about("File to read")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("build")
                .about("Compiles a source code file into a bytecode file")
                .arg(
                    Arg::new("FILE")
                        .about("File to compile")
                        .required(true)
                        .index(1))
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .takes_value(true)
                        .value_name("COMPILED_FILE")
                        .about("Filename of the output file, default is the same name as the original file, but with a .mclj extension")
                )
        )
        .subcommand(
            App::new("exec").about("Executes a bytecode file").arg(
                Arg::new("FILE")
                    .about("File to execute")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            App::new("run")
                .about("Compiles and executes a source code file")
                .arg(
                    Arg::new("FILE")
                        .about("File to run")
                        .required(true)
                        .index(1),
                ),
        )
}

pub fn read_file_from_opts(opts: &ArgMatches) -> Result<String, String> {
    let filename = opts.value_of("FILE").unwrap();
    read_to_string(filename).map_err(|e| format!("Couldn't read file: {}", e))
}

pub fn output_file_from_opts(opts: &ArgMatches) -> Result<File, String> {
    let path = match opts.value_of("output") {
        Some(filename) => PathBuf::from_str(filename).unwrap(),
        None => {
            let filename = opts.value_of("FILE").unwrap();
            let mut path = PathBuf::from_str(filename).unwrap();
            path.set_extension("mclj");
            path
        }
    };

    File::create(path).map_err(|e| format!("Couldn't open file: {}", e))
}
