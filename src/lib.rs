//! A series of interpreters for Datalog.
#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
#[cfg_attr(test, macro_use)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
mod errors;
mod parser;

pub use errors::{Error, ErrorKind, Result, ResultExt};
pub use parser::{parse_program_file, parse_program_string, parse_stmt};
