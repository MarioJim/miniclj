use std::collections::HashMap;

use lalrpop_util::{lalrpop_mod, ParseError};
use num::Rational64;

use crate::{
    callables::CallablesTable, compiler::SExpr, constant::Constant, instruction::Instruction,
    memaddress::MemAddress,
};

lalrpop_mod!(
    #[allow(clippy::all)]
    bytecodeparser,
    "/src/parsers/bytecodeparser.rs"
);
lalrpop_mod!(
    #[allow(clippy::all)]
    lispparser,
    "/src/parsers/lispparser.rs"
);

type ConstantsInstructionsTuple = (HashMap<MemAddress, Constant>, Vec<Instruction>);
type BytecodeParseError<'a> = ParseError<usize, bytecodeparser::Token<'a>, String>;
type LispParseError<'a> = ParseError<usize, lispparser::Token<'a>, &'static str>;

/// Encloses the parser generated by `lalrpop`, used to parse bytecode
pub struct BytecodeParser;

impl BytecodeParser {
    pub fn parse(input: &str) -> Result<ConstantsInstructionsTuple, BytecodeParseError<'_>> {
        let callables_table = CallablesTable::default();
        bytecodeparser::BytecodeParser::new().parse(&callables_table, input)
    }
}

/// Encloses the parser generated by `lalrpop`, used to parse number literals
pub struct NumberLiteralParser;

impl NumberLiteralParser {
    pub fn parse(input: &str) -> Result<Rational64, LispParseError<'_>> {
        lispparser::NumberLiteralParser::new().parse(input)
    }
}

/// Encloses the parser generated by `lalrpop`, used to parse s-expressions
pub struct SExprsParser;

impl SExprsParser {
    pub fn parse(input: &str) -> Result<Vec<SExpr>, LispParseError<'_>> {
        lispparser::SExprsParser::new().parse(input)
    }
}
