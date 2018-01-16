#![allow(missing_docs)]

use std::io::Error as IoError;

use sparkly::Sparkly;

use ast::Clause;

error_chain! {
    errors {
        NoSuchClause(clause: Clause) {
            description("A non-existent clause was found")
            display("The clause {} does not exist", clause.to_doc().display())
        }
        Parse(msg: String) {
            description(&msg)
        }
    }
    foreign_links {
        Io(IoError);
    }
}
