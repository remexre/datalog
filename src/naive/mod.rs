use std::collections::{BTreeMap, HashMap};

use symbol::Symbol;

use {Clause, Interpeter, Literal, Result};

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

    /// Attempts to solve a goal.
    pub fn solve(&self, goal: Literal) -> Option<BTreeMap<Symbol, Literal>> {
        unimplemented!()
    }
}

impl Interpeter for NaiveInterpreter {
    fn run_assertion(&mut self, clause: Clause) -> Result<()> {
        unimplemented!()
    }

    fn run_retraction(&mut self, clause: Clause) -> Result<()> {
        unimplemented!()
    }

    fn run_query(&self, query: Literal) -> Result<BTreeMap<Symbol, Literal>> {
        unimplemented!()
    }
}
