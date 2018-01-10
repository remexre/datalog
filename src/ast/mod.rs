//! The Datalog basic AST.

mod symbol;
mod variable;

pub use self::symbol::Symbol;
pub use self::variable::Variable;

/// A complete program; really just a bunch of statements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program(pub Vec<Statement>);

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

/// A term, for example `foo`, `Bar`, or `baz(1, 2)`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    /// A literal term.
    Literal(Literal),

    /// A variable.
    Variable(Variable),
}

/// A literal term, for example `foo`, `bar(X)`, or `baz(quux(X, 2), X)`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal(pub Symbol, pub Vec<Term>);

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
