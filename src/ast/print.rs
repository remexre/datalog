use sparkly::{Doc, Sparkly};

use ast::{Clause, Literal, Name, Program, Statement, Term, Variable};
use styles::{NAME, PUNCTUATION, VARIABLE};

impl_Display_for_Sparkly!(Program);
impl Sparkly for Program {
    fn to_doc(&self) -> Doc {
        Doc::lines(self.0.iter())
    }
}

impl_Display_for_Sparkly!(Statement);
impl Sparkly for Statement {
    fn to_doc(&self) -> Doc {
        match *self {
            Statement::Assertion(ref c) => {
                c.to_doc().append(Doc::text(".", PUNCTUATION))
            }
            Statement::Retraction(ref c) => {
                c.to_doc().append(Doc::text("~", PUNCTUATION))
            }
            Statement::Query(ref q) => {
                q.to_doc().append(Doc::text("?", PUNCTUATION))
            }
        }
    }
}

impl_Display_for_Sparkly!(Clause);
impl Sparkly for Clause {
    fn to_doc(&self) -> Doc {
        let Clause(ref head, ref body) = *self;
        if body.len() == 0 {
            head.to_doc()
        } else {
            let body = Doc::text(",", PUNCTUATION)
                .join(body.iter().map(|l| Doc::space().append(l.to_doc())))
                .nest(4)
                .group();
            head.to_doc()
                .append(Doc::nbsp())
                .append(Doc::text(":-", PUNCTUATION))
                .append(body)
        }
    }
}

impl_Display_for_Sparkly!(Literal);
impl Sparkly for Literal {
    fn to_doc(&self) -> Doc {
        let Literal(ref pred, ref args) = *self;
        let args = Doc::from(",").append(Doc::space()).join(args);
        pred.to_doc().append(args.bracket("(", ")"))
    }
}

impl_Display_for_Sparkly!(Term);
impl Sparkly for Term {
    fn to_doc(&self) -> Doc {
        match *self {
            Term::Name(ref n) => n.to_doc(),
            Term::Var(ref v) => v.to_doc(),
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
