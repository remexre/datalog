use parser::{DatalogParser, Rule};

#[test]
fn idents() {
    parses_to! {
        parser: DatalogParser,
        input: "parent",
        rule: Rule::ident,
        tokens: [
            ident(0, 6),
        ]
    }
    parses_to! {
        parser: DatalogParser,
        input: "1234",
        rule: Rule::ident,
        tokens: [
            ident(0, 4),
        ]
    }
}

#[test]
fn vars() {
    parses_to! {
        parser: DatalogParser,
        input: "Foo",
        rule: Rule::variable,
        tokens: [
            variable(0, 3),
        ]
    }
}

#[test]
fn strings() {
    parses_to! {
        parser: DatalogParser,
        input: r#""""#,
        rule: Rule::string,
        tokens: [
            string(0, 2),
        ]
    }
    parses_to! {
        parser: DatalogParser,
        input: r#""-:-""#,
        rule: Rule::string,
        tokens: [
            string(0, 5, [
                raw_ch(1, 2),
                raw_ch(2, 3),
                raw_ch(3, 4),
            ]),
        ]
    }
    parses_to! {
        parser: DatalogParser,
        input: r#""first\nsecond""#,
        rule: Rule::string,
        tokens: [
            string(0, 15, [
                raw_ch(1, 2),
                raw_ch(2, 3),
                raw_ch(3, 4),
                raw_ch(4, 5),
                raw_ch(5, 6),
                esc_ch(6, 8, [
                    predef_esc(7, 8)
                ]),
                raw_ch(8, 9),
                raw_ch(9, 10),
                raw_ch(10, 11),
                raw_ch(11, 12),
                raw_ch(12, 13),
                raw_ch(13, 14),
            ]),
        ]
    }
}

#[test]
fn literals() {
    parses_to! {
        parser: DatalogParser,
        input: "zero-arity-literal",
        rule: Rule::literal,
        tokens: [
            literal(0, 18, [
                symbol(0, 18, [
                    ident(0, 18),
                ]),
            ]),
        ]
    }
    parses_to! {
        parser: DatalogParser,
        input: "also-zero-arity()",
        rule: Rule::literal,
        tokens: [
            literal(0, 17, [
                symbol(0, 15, [
                    ident(0, 15),
                ]),
            ]),
        ]
    }
    parses_to! {
        parser: DatalogParser,
        input: "1(arg)",
        rule: Rule::literal,
        tokens: [
            literal(0, 6, [
                symbol(0, 1, [
                    ident(0, 1),
                ]),
                term_list(2, 5, [
                    term(2, 5, [
                        literal(2, 5, [
                            symbol(2, 5, [
                                ident(2, 5),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]
    }
    parses_to! {
        parser: DatalogParser,
        input: "parent(john, douglas)",
        rule: Rule::literal,
        tokens: [
            literal(0, 21, [
                symbol(0, 6, [
                    ident(0, 6),
                ]),
                term_list(7, 20, [
                    term(7, 11, [
                        literal(7, 11, [
                            symbol(7, 11, [
                                ident(7, 11),
                            ]),
                        ]),
                    ]),
                    term(13, 20, [
                        literal(13, 20, [
                            symbol(13, 20, [
                                ident(13, 20),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]
    }
    parses_to! {
        parser: DatalogParser,
        input: r#"aBcD(-0, "\n\u03bb")"#,
        rule: Rule::literal,
        tokens: [
            literal(0, 20, [
                symbol(0, 4, [
                    ident(0, 4),
                ]),
                term_list(5, 19, [
                    term(5, 7, [
                        literal(5, 7, [
                            symbol(5, 7, [
                                ident(5, 7),
                            ]),
                        ]),
                    ]),
                    term(9, 19, [
                        literal(9, 19, [
                            symbol(9, 19, [
                                string(9, 19, [
                                    esc_ch(10, 12, [
                                        predef_esc(11, 12)
                                    ]),
                                    esc_ch(12, 18, [
                                        uni4_esc(13, 18, [
                                            hex_digit(14, 15),
                                            hex_digit(15, 16),
                                            hex_digit(16, 17),
                                            hex_digit(17, 18),
                                        ]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]
    }
}
