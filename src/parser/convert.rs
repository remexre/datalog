use std::char::from_u32 as char_from_u32;

use pest::Error;
use pest::iterators::{Pair, Pairs};
use pest::inputs::Input;

use ast::{Clause, Literal, Name, Program, Statement, Term, Variable};
use parser::Rule;
use parser::utils::{as_amb, as_one, as_one_any};

pub fn convert_program<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Program, Error<Rule, I>> {
    as_one(pairs, Rule::program, |pairs| {
        pairs
            .map(Pair::into_inner)
            .map(convert_statement_one)
            .collect::<Result<_, _>>()
            .map(Program)
    })
}

pub fn convert_statement<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Statement, Error<Rule, I>> {
    as_one(pairs, Rule::stmt, convert_statement_one)
}

pub fn convert_statement_one<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Statement, Error<Rule, I>> {
    as_one_any(pairs, Rule::stmt, |token| match token.as_rule() {
        Rule::assertion => {
            convert_clause(token.into_inner()).map(Statement::Assertion)
        }
        Rule::retraction => {
            convert_clause(token.into_inner()).map(Statement::Retraction)
        }
        Rule::query => {
            convert_literal(token.into_inner()).map(Statement::Query)
        }
        _ => Err(Error::ParsingError {
            positives: vec![Rule::assertion, Rule::retraction, Rule::query],
            negatives: vec![],
            pos: token.into_span().start_pos(),
        }),
    })
}

pub fn convert_clause<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Clause, Error<Rule, I>> {
    as_one(pairs, Rule::clause, |pairs| {
        as_amb(pairs, Rule::literal, Rule::literal_list, |head, body| {
            let head = convert_literal_one(head)?;
            let body = if let Some(body) = body {
                body.map(Pair::into_inner)
                    .map(convert_literal_one)
                    .collect()
            } else {
                Ok(vec![])
            };
            Ok(Clause(head, body?))
        })
    })
}

pub fn convert_literal<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Literal, Error<Rule, I>> {
    as_one(pairs, Rule::literal, convert_literal_one)
}

pub fn convert_literal_one<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Literal, Error<Rule, I>> {
    as_amb(pairs, Rule::name, Rule::term_list, |pred, args| {
        let pred = convert_name_one(pred)?;
        let args = if let Some(args) = args {
            args.map(Pair::into_inner).map(convert_term_one).collect()
        } else {
            Ok(vec![])
        };
        Ok(Literal(pred, args?))
    })
}

#[cfg(test)]
pub fn convert_term<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Term, Error<Rule, I>> {
    as_one(pairs, Rule::term, convert_term_one)
}

pub fn convert_term_one<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Term, Error<Rule, I>> {
    as_one_any(pairs, Rule::term, |token| match token.as_rule() {
        Rule::name => convert_name_one(token.into_inner()).map(Term::Name),
        Rule::variable => Ok(Term::Var(convert_variable_one(token))),
        _ => Err(Error::ParsingError {
            positives: vec![Rule::name, Rule::variable],
            negatives: vec![],
            pos: token.into_span().start_pos(),
        }),
    })
}

#[cfg(test)]
pub fn convert_name<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Name, Error<Rule, I>> {
    as_one(pairs, Rule::name, convert_name_one)
}

pub fn convert_name_one<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Name, Error<Rule, I>> {
    as_one_any(pairs, Rule::name, |token| {
        match token.as_rule() {
            Rule::ident => {
                // This should be infallible.
                Ok(Name::new(token.as_str()).unwrap())
            }
            Rule::string => {
                // This might fail, since "X" is a valid string but an invalid
                // name.
                let string = token
                    .clone()
                    .into_inner()
                    .map(convert_char)
                    .collect::<Result<String, _>>()?;
                Name::new(&string).ok_or_else(|| {
                    Error::ParsingError {
                        positives: vec![Rule::name],
                        negatives: vec![],
                        pos: token.into_span().start_pos(),
                    }
                })
            }
            _ => Err(Error::ParsingError {
                positives: vec![Rule::ident, Rule::string],
                negatives: vec![],
                pos: token.into_span().start_pos(),
            }),
        }
    })
}

pub fn convert_char<I: Input>(
    token: Pair<Rule, I>,
) -> Result<char, Error<Rule, I>> {
    match token.as_rule() {
        Rule::raw_ch => {
            let mut s = token.as_str().chars();
            let ch = s.next().unwrap();
            assert_eq!(s.next(), None);
            Ok(ch)
        }
        Rule::esc_ch => {
            let mut pairs = token.into_inner();
            let esc = pairs.next().unwrap();
            assert_eq!(pairs.next(), None);
            convert_escape(esc)
        }
        rule => unimplemented!(
            "error at {:?} unrecognized char {}",
            token.into_span(),
            rule,
        ),
    }
}

pub fn convert_escape<I: Input>(
    token: Pair<Rule, I>,
) -> Result<char, Error<Rule, I>> {
    match token.as_rule() {
        Rule::hex_esc | Rule::uni4_esc | Rule::uni8_esc => {
            let mut n = 0;
            for token in token.into_inner() {
                assert_eq!(token.as_rule(), Rule::hex_digit);
                n = (n << 4) | match token.as_str() {
                    "0" => 0,
                    "1" => 1,
                    "2" => 2,
                    "3" => 3,
                    "4" => 4,
                    "5" => 5,
                    "6" => 6,
                    "7" => 7,
                    "8" => 8,
                    "9" => 9,
                    "a" | "A" => 10,
                    "b" | "B" => 11,
                    "c" | "C" => 12,
                    "d" | "D" => 13,
                    "e" | "E" => 14,
                    "f" | "F" => 15,
                    s => unimplemented!("error: unrecognized hex digit {}", s),
                };
            }
            if let Some(ch) = char_from_u32(n) {
                Ok(ch)
            } else {
                unimplemented!("bad hex escape")
            }
        }
        Rule::predef_esc => match token.as_str() {
            "n" => Ok('\n'),
            "r" => Ok('\r'),
            "t" => Ok('\t'),
            "\\" => Ok('\\'),
            "\"" => Ok('"'),
            "'" => Ok('\''),
            s => unimplemented!("error: unrecognized predef esc {}", s),
        },
        rule => unimplemented!(
            "error at {:?} unrecognized escape {}",
            token.into_span(),
            rule,
        ),
    }
}

#[cfg(test)]
pub fn convert_variable<I: Input>(
    pairs: Pairs<Rule, I>,
) -> Result<Variable, Error<Rule, I>> {
    as_one_any(
        pairs,
        Rule::variable,
        |token| Ok(convert_variable_one(token)),
    )
}

pub fn convert_variable_one<I: Input>(token: Pair<Rule, I>) -> Variable {
    Variable::new(token.as_str()).unwrap()
}
