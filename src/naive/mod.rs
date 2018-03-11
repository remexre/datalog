use std::collections::{BTreeMap, HashMap};
use std::iter::empty;

use {Clause, Interpeter, Literal, Name, Result, Variable};

/// A naive interpreter.
// An interpreter based on the one in [the first edition of Modern Compiler
// Design](https://dickgrune.com/Books/MCD_1st_Edition/), page 601.
#[derive(Debug)]
pub struct NaiveInterpreter {
    facts: HashMap<(Name, usize), Vec<Vec<Name>>>,
    rules: HashMap<(Name, usize), Vec<Clause>>,
}

impl NaiveInterpreter {
    /// Creates a new instance of `NaiveInterpreter`.
    pub fn new() -> NaiveInterpreter {
        NaiveInterpreter {
            facts: HashMap::new(),
            rules: HashMap::new(),
        }
    }

    /// Attempts to solve a goal.
    pub fn solve<'a>(
        &'a self,
        goal: Literal,
    ) -> Box<'a + Iterator<Item = BTreeMap<Variable, Name>>> {
        Box::new(self.solve_facts(goal.clone()).chain(self.solve_rules(goal)))
    }

    /// Attempts to solve a goal, only using facts.
    pub fn solve_facts<'a>(
        &'a self,
        goal: Literal,
    ) -> Box<'a + Iterator<Item = BTreeMap<Variable, Name>>> {
        if let Some(facts) = self.facts.get(&goal.signature()) {
            Box::new(
                facts
                    .iter()
                    .filter_map(move |fact| goal.try_instantiate_fact(fact)),
            )
        } else {
            Box::new(empty())
        }
    }

    /// Attempts to solve a goal, without using facts about the goal.
    pub fn solve_rules<'a>(
        &'a self,
        goal: Literal,
    ) -> Box<'a + Iterator<Item = BTreeMap<Variable, Name>>> {
        if let Some(rules) = self.rules.get(&goal.signature()) {
            Box::new(
                rules
                    .iter()
                    .flat_map(move |rule| self.solve_rule(&goal, rule)),
            )
        } else {
            Box::new(empty())
        }
    }

    /// Attempts to solve a goal, using the given rule.
    pub fn solve_rule<'a>(
        &'a self,
        goal: &Literal,
        rule: &'a Clause,
    ) -> Box<'a + Iterator<Item = BTreeMap<Variable, Name>>> {
        use styles::ERROR;
        use sparkly::Doc;

        let opt = rule.try_instantiate(goal.clone());
        if opt.is_none() {
            return Box::new(empty());
        }
        let body = opt.unwrap();

        eprintln!(
            "{}",
            Doc::text("TODO:", ERROR).append(Doc::lines(body)).display()
        );
        Box::new(empty())
    }
}

impl Interpeter for NaiveInterpreter {
    fn run_assertion(&mut self, clause: Clause) -> Result<()> {
        if let Some(fact) = clause.as_fact() {
            self.facts
                .entry(clause.pred())
                .or_insert_with(Vec::new)
                .push(fact)
        } else {
            self.rules
                .entry(clause.pred())
                .or_insert_with(Vec::new)
                .push(clause)
        }
        Ok(())
    }

    fn run_retraction(&mut self, clause: Clause) -> Result<()> {
        unimplemented!()
    }

    fn run_query<'a>(
        &'a self,
        query: Literal,
    ) -> Box<'a + Iterator<Item = BTreeMap<Variable, Name>>> {
        self.solve(query)
    }
}
