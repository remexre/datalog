use pest::Parser;

use ast::{Clause, Literal, Program, Statement, Symbol, Term, Variable};
use parser::{convert, DatalogParser, Rule};

macro_rules! ast_parse_test {
    ($([$converter:ident , $rule:ident] as $name:ident : $($src:expr => $ast:expr),*;)*) => {
        $(
            #[test]
            fn $name() {
                $(match DatalogParser::parse_str(Rule::$rule, $src)
                        .map_err(|err| panic!("{}", err))
                        .and_then(convert::$converter) {
                    Ok(expr) => assert_eq!(expr, $ast),
                    Err(err) => panic!("{}", err),
                })*
            }
        )*
    };
}

ast_parse_test! {
    [convert_program, program] as program:
        concat!("red(apple).\nfruit(apple).\nspicy(X) :-\n\tred(X),\n\tvegeta",
            "ble(X).\nspicy(X)?\nvegetable(apple).\nspicy(apple)?\nvegetable(",
            "apple)~\nspicy(apple)?") =>
        Program(vec![
            Statement::Assertion(Clause(
                Literal("red".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
                ])),
            Statement::Assertion(Clause(
                Literal("fruit".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
                ])),
            Statement::Assertion(Clause(
                Literal("spicy".into(), vec![Term::Variable("X".into())]), vec![
                    Literal("red".into(), vec![Term::Variable("X".into())]),
                    Literal("vegetable".into(), vec![Term::Variable("X".into())]),
                ])),
            Statement::Query(
                Literal("spicy".into(), vec![Term::Variable("X".into())])),
            Statement::Assertion(Clause(
                Literal("vegetable".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
                ])),
            Statement::Query(
                Literal("spicy".into(), vec![Term::Literal(Literal("apple".into(), vec![]))])),
            Statement::Retraction(Clause(
                Literal("vegetable".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
                ])),
            Statement::Query(
                Literal("spicy".into(), vec![Term::Literal(Literal("apple".into(), vec![]))])),
        ]);

    [convert_statement, stmt] as statement:
        "red(apple)." => Statement::Assertion(Clause(
            Literal("red".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
            ])),
        "fruit(apple)." => Statement::Assertion(Clause(
            Literal("fruit".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
            ])),
        "spicy(X) :-\n\tred(X),\n\tvegetable(X)." => Statement::Assertion(Clause(
            Literal("spicy".into(), vec![Term::Variable("X".into())]), vec![
                Literal("red".into(), vec![Term::Variable("X".into())]),
                Literal("vegetable".into(), vec![Term::Variable("X".into())]),
            ])),
        "spicy(X)?" => Statement::Query(
            Literal("spicy".into(), vec![Term::Variable("X".into())])),
        "vegetable(apple)." => Statement::Assertion(Clause(
            Literal("vegetable".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
            ])),
        "spicy(apple)?" => Statement::Query(
            Literal("spicy".into(), vec![Term::Literal(Literal("apple".into(), vec![]))])),
        "vegetable(apple)~" => Statement::Retraction(Clause(
            Literal("vegetable".into(), vec![Term::Literal(Literal("apple".into(), vec![]))]), vec![
            ]));

    [convert_clause, clause] as clause:
        "red(apple)" => Clause(Literal(Symbol::from("red"), vec![
            Term::Literal(Literal(Symbol::from("apple"), vec![])),
        ]), vec![]),
        "fruit(apple)" => Clause(Literal(Symbol::from("fruit"), vec![
            Term::Literal(Literal(Symbol::from("apple"), vec![])),
        ]), vec![]),
        "spicy(X) :-\n\tred(X),\n\tvegetable(X)" =>
            Clause(Literal(Symbol::from("spicy"), vec![ Term::Variable(Variable::from("X")) ]), vec![
                Literal(Symbol::from("red"), vec![ Term::Variable(Variable::from("X")) ]),
                Literal(Symbol::from("vegetable"), vec![ Term::Variable(Variable::from("X")) ]),
            ]);

    [convert_literal, literal] as literal:
        "foo" => Literal(Symbol::from("foo"), vec![]),
        "bar(X)" => Literal(Symbol::from("bar"), vec![Term::Variable(Variable::from("X"))]),
        "baz(quux(X, 2), X)" => Literal(Symbol::from("baz"), vec![
            Term::Literal(Literal(Symbol::from("quux"), vec![
                Term::Variable(Variable::from("X")),
                Term::Literal(Literal(Symbol::from("2"), vec![])),
            ])),
            Term::Variable(Variable::from("X")),
        ]);

    [convert_term, term] as term:
        "foo" => Term::Literal(Literal(Symbol::from("foo"), vec![])),
        "Bar" => Term::Variable(Variable::from("Bar")),
        "baz(1, 2)" => Term::Literal(Literal(Symbol::from("baz"), vec![
            Term::Literal(Literal(Symbol::from("1"), vec![])),
            Term::Literal(Literal(Symbol::from("2"), vec![])),
        ]));

    [convert_symbol, symbol] as symbol:
        "foo" => Symbol::from("foo"),
        "42" => Symbol::from("42"),
        r#""qwerty\nasdf\n\u03bb""# => Symbol::from("qwerty\nasdf\n\u{3bb}");

    [convert_variable, variable] as variable:
        "X" => Variable::from("X"),
        "Foo" => Variable::from("Foo"),
        "A123" => Variable::from("A123");
}
