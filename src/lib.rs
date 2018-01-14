//! A series of interpreters for Datalog.
#![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
#[cfg_attr(test, macro_use)]
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate sparkly;
extern crate symbol;

mod ast;
mod errors;
mod naive;
pub(crate) mod parser;

use std::collections::BTreeMap;

use symbol::Symbol;

pub use ast::{Clause, Literal, Name, Program, Statement, Term, Variable};
pub use errors::{Error, ErrorKind, Result, ResultExt};
pub use naive::NaiveInterpreter;

/// A Datalog interpreter.
pub trait Interpeter {
    /// Adds an assertion to the fact set.
    fn run_assertion(&mut self, clause: Clause) -> Result<()>;

    /// Retracts an assertion from the fact set.
    fn run_retraction(&mut self, clause: Clause) -> Result<()>;

    /// Runs a query against the fact set. Returns the variable bindings that
    /// make it true, or `None` if the query is unprovable.
    fn run_query(&self, query: Literal) -> Result<BTreeMap<Symbol, Literal>>;

    /// Runs a statement.
    fn run_stmt(
        &mut self,
        stmt: Statement,
    ) -> Result<BTreeMap<Symbol, Literal>> {
        match stmt {
            Statement::Assertion(clause) => {
                self.run_assertion(clause).map(|()| BTreeMap::default())
            }
            Statement::Retraction(clause) => {
                self.run_retraction(clause).map(|()| BTreeMap::default())
            }
            Statement::Query(query) => self.run_query(query),
        }
    }

    /// Loads a program into the interpreter.
    fn load_program(&mut self, program: Program) -> Result<()> {
        for stmt in program.0 {
            match stmt {
                Statement::Assertion(clause) => self.run_assertion(clause)?,
                Statement::Retraction(clause) => self.run_retraction(clause)?,
                Statement::Query(_) => {}
            }
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

    fn run_query(&self, query: Literal) -> Result<BTreeMap<Symbol, Literal>> {
        match *self {
            DynamicInterpreter::Naive(ref i) => i.run_query(query),
        }
    }

    fn run_stmt(
        &mut self,
        stmt: Statement,
    ) -> Result<BTreeMap<Symbol, Literal>> {
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
