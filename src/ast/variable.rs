use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(test)]
use regex::Regex;

use symbol::Symbol;

/// A variable, for example `X`, `Foo`, or `A123`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(Symbol);

impl Variable {
    /// Tries to convert a `String` to a `Variable`.
    pub fn new(var: &str) -> Option<Variable> {
        if Variable::is_valid(var) {
            Some(Variable(var.into()))
        } else {
            None
        }
    }

    /// Returns whether the string is a valid variable.
    pub fn is_valid(s: &str) -> bool {
        let mut chars = s.chars();
        if let Some(ch) = chars.next() {
            is_var_start_char(ch) && chars.all(is_var_char)
        } else {
            false
        }
    }
}

impl AsRef<str> for Variable {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Variable {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.write_str(&self.0)
    }
}

fn is_var_char(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z')
        || ('0' <= ch && ch <= '9') || ch == '_'
}

fn is_var_start_char(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z') || ch == '_'
}

#[cfg(test)]
lazy_static!{
    static ref VARIABLE_REGEX: Regex = Regex::new("^[A-Z_][A-Za-z0-9_]*$").unwrap();
}

#[cfg(test)]
proptest! {
    #[test]
    fn variable_new(ref s in ".*") {
        if VARIABLE_REGEX.is_match(s) {
            prop_assert!(Variable::new(s).is_some());
        } else {
            prop_assert_eq!(Variable::new(s), None);
        }
    }
}
