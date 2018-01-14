use pest::Error;
use pest::iterators::{Pair, Pairs};
use pest::inputs::Input;

use parser::Rule;

pub fn as_amb<F, I, T>(
    pairs: Pairs<Rule, I>,
    a_rule: Rule,
    b_rule: Rule,
    func: F,
) -> Result<T, Error<Rule, I>>
where
    F: Fn(Pairs<Rule, I>, Option<Pairs<Rule, I>>) -> Result<T, Error<Rule, I>>,
    I: Input,
{
    as_ht(pairs, a_rule, |a, mut pairs| {
        if let Some(b) = pairs.next() {
            if let Some(token) = pairs.next() {
                Err(Error::ParsingError {
                    positives: vec![],
                    negatives: vec![token.as_rule()],
                    pos: token.into_span().start_pos(),
                })
            } else if b.as_rule() == b_rule {
                func(a, Some(b.into_inner()))
            } else {
                Err(Error::ParsingError {
                    positives: vec![b_rule],
                    negatives: vec![],
                    pos: b.into_span().start_pos(),
                })
            }
        } else {
            func(a, None)
        }
    })
}

fn as_ht<F, I, T>(
    mut pairs: Pairs<Rule, I>,
    head_rule: Rule,
    func: F,
) -> Result<T, Error<Rule, I>>
where
    F: Fn(Pairs<Rule, I>, Pairs<Rule, I>) -> Result<T, Error<Rule, I>>,
    I: Input,
{
    if let Some(head) = pairs.next() {
        if head.as_rule() == head_rule {
            func(head.into_inner(), pairs)
        } else {
            Err(Error::ParsingError {
                positives: vec![head_rule],
                negatives: vec![],
                pos: head.into_span().start_pos(),
            })
        }
    } else {
        panic!("Could not find expected {}", head_rule)
    }
}

pub fn as_one_any<F, I, T>(
    mut pairs: Pairs<Rule, I>,
    rule: Rule,
    func: F,
) -> Result<T, Error<Rule, I>>
where
    F: Fn(Pair<Rule, I>) -> Result<T, Error<Rule, I>>,
    I: Input,
{
    if let Some(token) = pairs.next() {
        if let Some(token) = pairs.next() {
            Err(Error::ParsingError {
                positives: vec![],
                negatives: vec![token.as_rule()],
                pos: token.into_span().start_pos(),
            })
        } else {
            func(token)
        }
    } else {
        panic!("Could not find expected {}", rule)
    }
}

pub fn as_one<F, I, T>(
    pairs: Pairs<Rule, I>,
    rule: Rule,
    func: F,
) -> Result<T, Error<Rule, I>>
where
    F: Fn(Pairs<Rule, I>) -> Result<T, Error<Rule, I>>,
    I: Input,
{
    as_one_any(pairs, rule, |token| {
        if token.as_rule() == rule {
            func(token.into_inner())
        } else {
            Err(Error::ParsingError {
                positives: vec![rule],
                negatives: vec![],
                pos: token.into_span().start_pos(),
            })
        }
    })
}
