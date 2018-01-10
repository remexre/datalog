use std::collections::HashMap;

use {Clause, Interpeter, Result, Statement, Symbol};

/// A naive interpreter.
#[derive(Debug)]
pub struct NaiveInterpreter {
    facts: HashMap<Symbol, Clause>,
}

impl NaiveInterpreter {
    /// Creates a new instance of `NaiveInterpreter`.
    pub fn new() -> NaiveInterpreter {
        NaiveInterpreter {
            facts: HashMap::new(),
        }
    }
}

impl Interpeter for NaiveInterpreter {
    fn run_stmt(&mut self, stmt: Statement) -> Result<()> {
        match stmt {
            Statement::Assertion(clause) => unimplemented!(),
            Statement::Retraction(clause) => unimplemented!(),
            Statement::Query(literal) => unimplemented!(),
        }
    }
}
