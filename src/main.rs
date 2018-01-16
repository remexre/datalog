#[macro_use]
extern crate clap;
extern crate datalog;
extern crate error_chain;
extern crate liner;
extern crate sparkly;

use std::io::ErrorKind as IoErrorKind;
use std::process::exit;

use clap::ArgMatches;
use datalog::{DynamicInterpreter, ErrorKind, Interpeter, NaiveInterpreter,
              Program, Result, Statement};
use datalog::styles::{ERROR, PUNCTUATION};
use error_chain::ChainedError;
use liner::Context;
use sparkly::{Doc, Sparkly};

fn main() {
    let matches = clap_app!((crate_name!()) =>
        (about: crate_description!())
        (author: crate_authors!())
        (version: crate_version!())
        (@subcommand pretty =>
            (about: "Pretty-prints Datalog code")
            (@arg FILE: +required "Loads the given Datalog file")
        )
        (@subcommand run =>
            (about: "Runs Datalog code")
            (@arg FILE: "Loads the given Datalog file")
            (@arg INTERPRETER: -i +takes_value "The interpreter to use. One of: naive")
            (@arg STMTS: ... -e +takes_value "A statement to run instead of starting a REPL")
        )
    ).get_matches();

    let result = match matches.subcommand() {
        ("pretty", Some(matches)) => pretty(matches),
        ("run", Some(matches)) => run(matches),
        _ => {
            eprintln!("{}", matches.usage());
            exit(1);
        }
    };
    match result {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err.display_chain());
            eprintln!("Exiting with error...");
            exit(1);
        }
    }
}

fn pretty(matches: &ArgMatches) -> Result<()> {
    let file = matches.value_of("FILE").unwrap();
    let program = if file == "-" {
        use std::io::{stdin, Read};

        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        buf.parse()?
    } else {
        Program::parse_file(file)?
    };

    program.to_doc().writeln_to_tty().map_err(|e| e.into())
}

fn run(matches: &ArgMatches) -> Result<()> {
    let mut interpreter: DynamicInterpreter =
        match matches.value_of("INTERPRETER") {
            Some("naive") | None => NaiveInterpreter::new().into(),
            Some(_) => {
                eprintln!("{}", matches.usage());
                exit(1);
            }
        };

    if let Some(path) = matches.value_of("FILE") {
        let program = Program::parse_file(path)?;
        interpreter.load_program(program)?;
    }

    if let Some(stmts) = matches.values_of("STMTS") {
        for stmt in stmts {
            let stmt: Statement = stmt.parse()?;
            Doc::text("?-", PUNCTUATION)
                .append(Doc::space())
                .append(stmt.to_doc())
                .writeln_to_tty()?;
            run_stmt(&mut interpreter, stmt)?;
        }
        Ok(())
    } else {
        let mut ctx = Context::new();
        let r = loop {
            if let Err(err) = run_repl(&mut interpreter, &mut ctx) {
                match err.kind() {
                    &ErrorKind::Io(ref err) => match err.kind() {
                        IoErrorKind::UnexpectedEof => break Ok(()),
                        IoErrorKind::Interrupted => continue,
                        _ => {}
                    },
                    _ => {}
                }
                break Err(err);
            }
        };
        ctx.history.commit_history();
        r
    }
}

fn run_repl(
    interpreter: &mut DynamicInterpreter,
    ctx: &mut Context,
) -> Result<()> {
    let line = ctx.read_line("?- ", &mut |_| {})?;

    let stmt = line.parse()?;
    run_stmt(interpreter, stmt)?;

    ctx.history.push(line.into()).map_err(|err| err.into())
}

fn run_stmt(
    interpreter: &mut DynamicInterpreter,
    stmt: Statement,
) -> Result<()> {
    for binding in interpreter.run_stmt(stmt)? {
        Doc::text(",", PUNCTUATION)
            .append(Doc::space())
            .join(binding.iter().map(|(v, n)| {
                v.to_doc()
                    .append(Doc::nbsp())
                    .append(Doc::text("=", PUNCTUATION))
                    .append(Doc::nbsp())
                    .append(n.to_doc())
            }))
            .append(Doc::text(";", PUNCTUATION))
            .writeln_to_tty()?;
    }
    Doc::text("false.", ERROR).writeln_to_tty()?;
    Ok(())
}
