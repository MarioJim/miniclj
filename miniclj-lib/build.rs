fn main() {
    lalrpop::Configuration::new()
        .log_verbose()
        .process_dir("src/parsers")
        .unwrap();
}
