mod convert;
mod utils;

#[cfg(test)]
mod ast_tests;
#[cfg(test)]
mod cst_tests;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;
use std::rc::Rc;

use pest::Parser;
use pest::inputs::FileInput;

use ast::{Program, Statement};
use errors::Result;

use self::convert::{convert_program, convert_statement};

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("datalog.pest");

#[derive(Parser)]
#[grammar = "parser/datalog.pest"]
struct DatalogParser;

impl Display for Rule {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Rule::ident_ch => fmt.write_str("identifier character"),
            Rule::ident => fmt.write_str("identifier"),
            Rule::variable => fmt.write_str("variable"),
            Rule::raw_ch => fmt.write_str("non-escape character"),
            Rule::hex_digit => fmt.write_str("hex digit"),
            Rule::hex_esc => fmt.write_str("\\x escape"),
            Rule::uni4_esc => fmt.write_str("\\u escape"),
            Rule::uni8_esc => fmt.write_str("\\U escape"),
            Rule::predef_esc => fmt.write_str("predefined escape"),
            Rule::esc_ch => fmt.write_str("escape character"),
            Rule::string => fmt.write_str("string"),
            Rule::symbol => fmt.write_str("symbol"),
            Rule::literal => fmt.write_str("literal"),
            Rule::literal_list => fmt.write_str("list of literals"),
            Rule::term => fmt.write_str("term"),
            Rule::term_list => fmt.write_str("list of terms"),
            Rule::clause => fmt.write_str("clause"),
            Rule::assertion => fmt.write_str("assertion"),
            Rule::retraction => fmt.write_str("retraction"),
            Rule::query => fmt.write_str("query"),
            Rule::stmt => fmt.write_str("statement"),
            Rule::program => fmt.write_str("program"),
            Rule::stmt_all => fmt.write_str("statement"),
            Rule::whitespace => fmt.write_str("whitespace"),
            Rule::line_break => fmt.write_str("line break"),
            Rule::comment => fmt.write_str("comment"),
        }
    }
}

/// Parses a program from the given file.
pub fn parse_program_file<P: AsRef<Path>>(path: P) -> Result<Program> {
    let input = FileInput::new(path)?;
    DatalogParser::parse(Rule::program, Rc::new(input))
        .and_then(convert_program)
        .map_err(|_err| unimplemented!()) // TODO
}

/// Parses a program from the given string.
pub fn parse_program_string(src: &str) -> Result<Program> {
    DatalogParser::parse_str(Rule::program, src)
        .and_then(convert_program)
        .map_err(|_err| unimplemented!()) // TODO
}

/// Parses a statement from the given string.
pub fn parse_stmt(src: &str) -> Result<Statement> {
    DatalogParser::parse_str(Rule::stmt, src)
        .and_then(convert_statement)
        .map_err(|_err| unimplemented!()) // TODO
}
