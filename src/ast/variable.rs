use std::fmt::{Display, Formatter, Result as FmtResult};

/// A variable, for example `X`, `Foo`, or `A123`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(String);

impl Variable {
    /// Tries to convert a `String` to an `Variable`, returning the `Variable`
    /// if the string is a valid variable and passing the `String` back if it
    /// is not.
    pub fn new(var: String) -> Result<Variable, String> {
        if Variable::is_valid(&var) {
            Ok(Variable(var))
        } else {
            Err(var)
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

impl From<&'static str> for Variable {
    fn from(s: &'static str) -> Variable {
        Variable::new(s.to_string()).unwrap()
    }
}

impl Into<String> for Variable {
    fn into(self) -> String {
        self.0
    }
}

fn is_var_char(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z')
        || ('0' <= ch && ch <= '9') || ch == '_' || ch == '-'
}

fn is_var_start_char(ch: char) -> bool {
    'A' <= ch && ch <= 'Z'
}
