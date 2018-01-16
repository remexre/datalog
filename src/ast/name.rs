use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(test)]
use regex::Regex;

use symbol::Symbol;

/// A name, for example `foo`, `42`, or `"qwerty\nasdf\n\u03bb"`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(Symbol);

impl Name {
    /// Tries to convert a `String` to a `Name`.
    pub fn new(ident: &str) -> Option<Name> {
        if Name::is_valid(ident) {
            Some(Name(ident.into()))
        } else {
            None
        }
    }

    /// Returns whether the string is a valid name.
    pub fn is_valid(s: &str) -> bool {
        if let Some(ch) = s.chars().next() {
            !(('A' <= ch && ch <= 'Z') || ch == '_')
        } else {
            false
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Name {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fn is_ident_char(ch: char) -> bool {
            ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z')
                || ('0' <= ch && ch <= '9') || ch == '_'
        }

        if self.0.chars().all(is_ident_char) {
            fmt.write_str(&self.0)
        } else {
            unimplemented!("Print name {:?} in string syntax", self.0)
        }
    }
}

#[cfg(test)]
lazy_static!{
    static ref NAME_REGEX: Regex = Regex::new("^[^A-Z_].*$").unwrap();
}

#[cfg(test)]
proptest! {
    #[test]
    fn name_new(ref s in ".*") {
        if NAME_REGEX.is_match(s) {
            prop_assert!(Name::new(s).is_some());
        } else {
            prop_assert_eq!(Name::new(s), None);
        }
    }
}
