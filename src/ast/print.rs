use sparkly::{Colour, Doc, Sparkly, Style};

use ast::{Clause, Literal, Name, Program, Statement, Term, Variable};

// TODO: Use const fns, once they're stable.
const NAME: Style = Style {
    foreground: Some(Colour::Cyan),
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
const PUNCTUATION: Style = Style {
    foreground: Some(Colour::Purple),
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
const VARIABLE: Style = Style {
    foreground: Some(Colour::Red),
    background: None,
    is_bold: true,
    is_dimmed: false,
    is_italic: false,
    is_underline: false,
    is_blink: false,
    is_reverse: false,
    is_hidden: false,
    is_strikethrough: false,
};

impl Sparkly for Program {
    fn to_doc(&self) -> Doc {
        Doc::lines(self.0.iter())
    }
}

impl Sparkly for Statement {
    fn to_doc(&self) -> Doc {
        match *self {
            Statement::Assertion(ref c) => {
                c.to_doc().append(Doc::from(".").style(PUNCTUATION))
            }
            Statement::Retraction(ref c) => {
                c.to_doc().append(Doc::from("~").style(PUNCTUATION))
            }
            Statement::Query(ref q) => {
                q.to_doc().append(Doc::from("?").style(PUNCTUATION))
            }
        }
    }
}

impl Sparkly for Clause {
    fn to_doc(&self) -> Doc {
        let Clause(ref head, ref body) = *self;
        if body.len() == 0 {
            head.to_doc()
        } else {
            let body = Doc::from(",")
                .style(PUNCTUATION)
                .join(body.iter().map(|l| Doc::space().append(l.to_doc())))
                .nest(4)
                .group();
            head.to_doc()
                .append(Doc::nbsp())
                .append(Doc::from(":-").style(PUNCTUATION))
                .append(body)
        }
    }
}

impl Sparkly for Literal {
    fn to_doc(&self) -> Doc {
        let Literal(ref pred, ref args) = *self;
        let args = Doc::from(",").append(Doc::space()).join(args);
        pred.to_doc().append(args.bracket("(", ")"))
    }
}

impl Sparkly for Term {
    fn to_doc(&self) -> Doc {
        match *self {
            Term::Name(ref n) => n.to_doc(),
            Term::Variable(ref v) => v.to_doc(),
        }
    }
}

impl Sparkly for Name {
    fn to_doc(&self) -> Doc {
        Doc::from(self.to_string()).style(NAME)
    }
}

impl Sparkly for Variable {
    fn to_doc(&self) -> Doc {
        Doc::from(self.to_string()).style(VARIABLE)
    }
}
