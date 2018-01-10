#![allow(missing_docs)]

use std::io::Error as IoError;

error_chain! {
    errors {
        Parse(msg: String) {
            description(&msg)
        }
    }
    foreign_links {
        Io(IoError);
    }
}
