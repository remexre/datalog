use std::fmt::{Display, Formatter, Result as FmtResult};

/// A symbol, for example `foo`, `42`, or `"qwerty\nasdf\n\u03bb"`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Symbol(String);

impl Symbol {
    /// Tries to convert a `String` to a `Symbol`, returning the `Symbol` if
    /// the string is a valid symbol and passing the `String` back if it is not.
    pub fn new(ident: String) -> Result<Symbol, String> {
        if Symbol::is_valid(&ident) {
            Ok(Symbol(ident))
        } else {
            Err(ident)
        }
    }

    /// Returns whether the string is a valid symbol.
    pub fn is_valid(s: &str) -> bool {
        if let Some(ch) = s.chars().next() {
            !(('A' <= ch && ch <= 'Z') || ch == '_')
        } else {
            false
        }
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fn is_ident_char(ch: char) -> bool {
            ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z')
                || ('0' <= ch && ch <= '9') || ch == '_'
                || ch == '-'
        }

        if self.0.chars().all(is_ident_char) {
            fmt.write_str(&self.0)
        } else {
            unimplemented!("Print symbol {:?} in string syntax", self.0)
        }
    }
}

impl From<&'static str> for Symbol {
    fn from(s: &'static str) -> Symbol {
        Symbol::new(s.to_string()).unwrap()
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        self.0
    }
}
