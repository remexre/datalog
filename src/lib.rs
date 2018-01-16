//! A series of interpreters for Datalog.
#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
#[cfg_attr(test, macro_use)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate sparkly;
extern crate symbol;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
extern crate regex;

mod ast;
mod errors;
mod naive;
pub(crate) mod parser;
mod util;

use std::collections::BTreeMap;
use std::iter::empty;

use sparkly::Sparkly;

pub use ast::{styles, Clause, Literal, Name, Program, Statement, Term,
              Variable};
pub use errors::{Error, ErrorKind, Result, ResultExt};
pub use naive::NaiveInterpreter;

/// Bindings from variable names to values.
pub type Bindings = BTreeMap<Variable, Name>;

/// A Datalog interpreter.
pub trait Interpeter {
    /// Adds an assertion to the fact set.
    fn run_assertion(&mut self, clause: Clause) -> Result<()>;

    /// Retracts an assertion from the fact set.
    fn run_retraction(&mut self, clause: Clause) -> Result<()>;

    /// Runs a query against the fact set. Returns an iterator over variable
    /// bindings that make it true.
    fn run_query<'a>(
        &'a self,
        query: Literal,
    ) -> Box<'a + Iterator<Item = Bindings>>;

    /// Runs a statement.
    fn run_stmt<'a>(
        &'a mut self,
        stmt: Statement,
    ) -> Result<Box<'a + Iterator<Item = Bindings>>> {
        match stmt {
            Statement::Assertion(clause) => {
                self.run_assertion(clause)?;
                Ok(Box::new(empty()))
            }
            Statement::Retraction(clause) => {
                self.run_retraction(clause)?;
                Ok(Box::new(empty()))
            }
            Statement::Query(query) => Ok(self.run_query(query)),
        }
    }

    /// Loads a program into the interpreter.
    fn load_program(&mut self, program: Program) -> Result<()> {
        for stmt in program.0 {
            match stmt {
                Statement::Assertion(clause) => self.run_assertion(clause)?,
                Statement::Retraction(clause) => self.run_retraction(clause)?,
                Statement::Query(q) => {
                    unimplemented!("run query on load {}", q.to_doc().display())
                }
            }
        }
        Ok(())
    }
}

/// An enumeration over the interpreters defined in this crate. Using this type
/// lowers dynamic dispatch overhead somewhat.
#[derive(Debug)]
pub enum DynamicInterpreter {
    /// A naive interpreter.
    Naive(NaiveInterpreter),
}

impl From<NaiveInterpreter> for DynamicInterpreter {
    fn from(i: NaiveInterpreter) -> DynamicInterpreter {
        DynamicInterpreter::Naive(i)
    }
}

impl Interpeter for DynamicInterpreter {
    fn run_assertion(&mut self, clause: Clause) -> Result<()> {
        match *self {
            DynamicInterpreter::Naive(ref mut i) => i.run_assertion(clause),
        }
    }

    fn run_retraction(&mut self, clause: Clause) -> Result<()> {
        match *self {
            DynamicInterpreter::Naive(ref mut i) => i.run_retraction(clause),
        }
    }

    fn run_query<'a>(
        &'a self,
        query: Literal,
    ) -> Box<'a + Iterator<Item = Bindings>> {
        match *self {
            DynamicInterpreter::Naive(ref i) => i.run_query(query),
        }
    }

    fn run_stmt<'a>(
        &'a mut self,
        stmt: Statement,
    ) -> Result<Box<'a + Iterator<Item = Bindings>>> {
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
