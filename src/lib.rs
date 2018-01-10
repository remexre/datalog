//! A series of interpreters for Datalog.
#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
#[cfg_attr(test, macro_use)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod errors;
mod naive;
pub(crate) mod parser;

pub use ast::{Clause, Literal, Program, Statement, Symbol, Term, Variable};
pub use errors::{Error, ErrorKind, Result, ResultExt};
pub use naive::NaiveInterpreter;

/// A Datalog interpreter.
pub trait Interpeter {
    /// Runs a single statement.
    fn run_stmt(&mut self, stmt: Statement) -> Result<()>;

    /// Loads a program into the interpreter.
    fn load_program(&mut self, program: Program) -> Result<()> {
        for stmt in program.0 {
            self.run_stmt(stmt)?;
        }
        Ok(())
    }
}

/// An enumeration over the interpreters defined in this crate. Using this type
/// lowers dynamic dispatch overhead somewhat.
#[derive(Debug)]
pub enum DynamicInterpreter {
    Naive(NaiveInterpreter),
}

impl From<NaiveInterpreter> for DynamicInterpreter {
    fn from(i: NaiveInterpreter) -> DynamicInterpreter {
        DynamicInterpreter::Naive(i)
    }
}

impl Interpeter for DynamicInterpreter {
    fn run_stmt(&mut self, stmt: Statement) -> Result<()> {
        match *self {
            DynamicInterpreter::Naive(ref mut i) => i.run_stmt(stmt),
        }
    }

    fn load_program(&mut self, program: Program) -> Result<()> {
        match *self {
            DynamicInterpreter::Naive(ref mut i) => i.load_program(program),
        }
    }
}
