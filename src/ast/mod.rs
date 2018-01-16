//! The basic Datalog AST.

mod eq;
mod name;
mod pattern_match;
mod print;
pub mod styles;
mod variable;

use std::path::Path;
use std::str::FromStr;

use Bindings;
use errors::{Error, Result};

pub use self::name::Name;
pub use self::variable::Variable;

/// A complete program; really just a bunch of statements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(pub Vec<Statement>);

impl Program {
    /// Parses a file and returns the `Program` inside it.
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Program> {
        ::parser::parse_program_file(path)
    }
}

impl FromStr for Program {
    type Err = Error;
    fn from_str(src: &str) -> Result<Program> {
        ::parser::parse_program_string(src)
    }
}

/// A statement in a program, either an assertion, retraction, or query.
///
/// For example, each of the following is a statement:
///
/// ```datalog
/// red(apple).
/// fruit(apple).
///
/// spicy(X) :-
///     red(X),
///     vegetable(X).
/// spicy(X)?
///
/// vegetable(apple).
/// spicy(apple)?
/// vegetable(apple)~
/// spicy(apple)?
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    /// An assertion that a clause is true.
    Assertion(Clause),

    /// A retraction, which removes a clause from the database.
    Retraction(Clause),

    /// A query, which produces all possible instantiations of its variables,
    /// or simply `true`.
    Query(Literal),
}

impl FromStr for Statement {
    type Err = Error;
    fn from_str(src: &str) -> Result<Statement> {
        ::parser::parse_stmt(&src)
    }
}

/// A clause (a fact or a rule).
///
/// For example, the following are all clauses:
///
/// ```datalog
/// red(apple)
///
/// fruit(apple)
///
/// spicy(X) :-
///     red(X),
///     vegetable(X)
/// ```
///
/// Note the lack of trailing `.`, `~`, or `?` on clauses; adding these marks
/// makes the clause an assertion, retraction, or query (respectively), all of
/// which are statements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clause(pub Literal, pub Vec<Literal>);

impl Clause {
    /// If this clause is a fact, returns its arguments. Otherwise, returns
    /// `None`.
    pub fn as_fact(&self) -> Option<Vec<Name>> {
        let Clause(Literal(_, ref args), ref body) = *self;
        if body.len() != 0 {
            return None;
        }

        let mut names = Vec::new();
        for arg in args {
            if let Term::Name(ref n) = *arg {
                names.push(n);
            } else {
                return None;
            }
        }
        Some(names.into_iter().cloned().collect())
    }

    /// Returns the head of the clause.
    pub fn head(&self) -> &Literal {
        let Clause(ref head, _) = *self;
        head
    }

    /// Returns the name and arity of the predicate this rule is for.
    ///
    /// For the clause `path(X, Y) :- path(X, Z), edge(Z, Y)`, this returns
    /// `("path", 2)`.
    pub fn pred(&self) -> (Name, usize) {
        self.head().signature()
    }
}

/// A literal term, for example `foo`, `bar(X)`, or `baz(quux(X, 2), X)`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal(pub Name, pub Vec<Term>);

impl Literal {
    /// Returns the name and arity of this literal.
    pub fn signature(&self) -> (Name, usize) {
        let Literal(ref pred, ref args) = *self;
        (pred.clone(), args.len())
    }

    /// Tries to instantiate the two literals to each other. Returns the
    /// variable bindings required to do so if it is possible, as well as the
    /// resulting literal.
    pub fn try_instantiate(
        self,
        other: Literal,
    ) -> Option<(Literal, Bindings)> {
        if self.signature() != other.signature() {
            return None;
        }
        let Literal(name, sargs) = self;
        let Literal(_, oargs) = other;

        unimplemented!()
    }

    /// Tries to instantiate this literal to the argument tuple of a fact
    /// describing this literal. Panics if `arg_tuple.len()` is not equal to
    /// the arity of this literal.
    pub fn try_instantiate_fact(&self, arg_tuple: &[Name]) -> Option<Bindings> {
        let Literal(_, ref args) = *self;
        assert_eq!(args.len(), arg_tuple.len());
        let mut bindings = Bindings::new();
        for i in 0..args.len() {
            let r = &arg_tuple[i];
            match args[i] {
                Term::Name(ref l) => if l != r {
                    return None;
                },
                Term::Var(ref l) => {
                    bindings.insert(l.clone(), r.clone());
                }
            }
        }
        Some(bindings)
    }
}

/// A term, for example `foo` or `Bar`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    /// A name.
    Name(Name),

    /// A variable.
    Var(Variable),
}

impl Term {
    /// Creates a new Term from the valid string, trying to parse as both a
    /// `Name` and a `Variable`.
    pub fn new<S: AsRef<str>>(s: S) -> Option<Term> {
        let s = s.as_ref();
        if let Some(name) = Name::new(s) {
            Some(Term::Name(name))
        } else if let Some(var) = Variable::new(s) {
            Some(Term::Var(var))
        } else {
            None
        }
    }
}
