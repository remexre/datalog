//! Styles used for pretty-printing.
//!
//! TODO: Use const fns to make these, once they're stable.

use sparkly::{Colour, Style};

const PLAIN: Style = Style {
    foreground: None,
    background: None,
    is_bold: false,
    is_dimmed: false,
    is_italic: false,
    is_underline: false,
    is_blink: false,
    is_reverse: false,
    is_hidden: false,
    is_strikethrough: false,
};

/// The style associated with an error.
pub const ERROR: Style = Style {
    foreground: Some(Colour::Red),
    ..PLAIN
};

/// The style associated with a name.
pub const NAME: Style = Style {
    foreground: Some(Colour::Blue),
    ..PLAIN
};

/// The style associated with a punctuation mark.
pub const PUNCTUATION: Style = Style {
    foreground: Some(Colour::Purple),
    ..PLAIN
};

/// The style associated with a variable.
pub const VARIABLE: Style = Style {
    foreground: Some(Colour::Green),
    ..PLAIN
};
