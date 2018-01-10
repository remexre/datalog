use pest::Parser;

use ast::{Clause, Literal, Name, Program, Statement, Term, Variable};
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
                Literal(Name::new("red").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
                ])),
            Statement::Assertion(Clause(
                Literal(Name::new("fruit").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
                ])),
            Statement::Assertion(Clause(
                Literal(Name::new("spicy").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())]), vec![
                    Literal(Name::new("red").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())]),
                    Literal(Name::new("vegetable").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())]),
                ])),
            Statement::Query(
                Literal(Name::new("spicy").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())])),
            Statement::Assertion(Clause(
                Literal(Name::new("vegetable").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
                ])),
            Statement::Query(
                Literal(Name::new("spicy").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))])),
            Statement::Retraction(Clause(
                Literal(Name::new("vegetable").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
                ])),
            Statement::Query(
                Literal(Name::new("spicy").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))])),
        ]);

    [convert_statement, stmt] as statement:
        "red(apple)." => Statement::Assertion(Clause(
            Literal(Name::new("red").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
            ])),
        "fruit(apple)." => Statement::Assertion(Clause(
            Literal(Name::new("fruit").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
            ])),
        "spicy(X) :-\n\tred(X),\n\tvegetable(X)." => Statement::Assertion(Clause(
            Literal(Name::new("spicy").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())]), vec![
                Literal(Name::new("red").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())]),
                Literal(Name::new("vegetable").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())]),
            ])),
        "spicy(X)?" => Statement::Query(
            Literal(Name::new("spicy").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())])),
        "vegetable(apple)." => Statement::Assertion(Clause(
            Literal(Name::new("vegetable").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
            ])),
        "spicy(apple)?" => Statement::Query(
            Literal(Name::new("spicy").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))])),
        "vegetable(apple)~" => Statement::Retraction(Clause(
            Literal(Name::new("vegetable").unwrap(), vec![Term::Literal(Literal(Name::new("apple").unwrap(), vec![]))]), vec![
            ]));

    [convert_clause, clause] as clause:
        "red(apple)" => Clause(Literal(Name::new("red").unwrap(), vec![
            Term::Literal(Literal(Name::new("apple").unwrap(), vec![])),
        ]), vec![]),
        "fruit(apple)" => Clause(Literal(Name::new("fruit").unwrap(), vec![
            Term::Literal(Literal(Name::new("apple").unwrap(), vec![])),
        ]), vec![]),
        "spicy(X) :-\n\tred(X),\n\tvegetable(X)" =>
            Clause(Literal(Name::new("spicy").unwrap(), vec![ Term::Variable(Variable::new("X").unwrap()) ]), vec![
                Literal(Name::new("red").unwrap(), vec![ Term::Variable(Variable::new("X").unwrap()) ]),
                Literal(Name::new("vegetable").unwrap(), vec![ Term::Variable(Variable::new("X").unwrap()) ]),
            ]);

    [convert_literal, literal] as literal:
        "foo" => Literal(Name::new("foo").unwrap(), vec![]),
        "bar(X)" => Literal(Name::new("bar").unwrap(), vec![Term::Variable(Variable::new("X").unwrap())]),
        "baz(quux(X, 2), X)" => Literal(Name::new("baz").unwrap(), vec![
            Term::Literal(Literal(Name::new("quux").unwrap(), vec![
                Term::Variable(Variable::new("X").unwrap()),
                Term::Literal(Literal(Name::new("2").unwrap(), vec![])),
            ])),
            Term::Variable(Variable::new("X").unwrap()),
        ]);

    [convert_term, term] as term:
        "foo" => Term::Literal(Literal(Name::new("foo").unwrap(), vec![])),
        "Bar" => Term::Variable(Variable::new("Bar").unwrap()),
        "baz(1, 2)" => Term::Literal(Literal(Name::new("baz").unwrap(), vec![
            Term::Literal(Literal(Name::new("1").unwrap(), vec![])),
            Term::Literal(Literal(Name::new("2").unwrap(), vec![])),
        ]));

    [convert_name, name] as name:
        "foo" => Name::new("foo").unwrap(),
        "42" => Name::new("42").unwrap(),
        r#""qwerty\nasdf\n\u03bb""# => Name::new("qwerty\nasdf\n\u{3bb}").unwrap();

    [convert_variable, variable] as variable:
        "X" => Variable::new("X").unwrap(),
        "Foo" => Variable::new("Foo").unwrap(),
        "A123" => Variable::new("A123").unwrap();
}
