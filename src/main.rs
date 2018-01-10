#[macro_use]
extern crate clap;
extern crate datalog;
extern crate error_chain;
extern crate liner;

use std::io::ErrorKind as IoErrorKind;
use std::process::exit;

use clap::ArgMatches;
use datalog::{DynamicInterpreter, Interpeter, NaiveInterpreter, Program,
              Result};
use error_chain::ChainedError;
use liner::Context;

fn main() {
    let matches = clap_app!((crate_name!()) =>
        (about: crate_description!())
        (author: crate_authors!())
        (version: crate_version!())
        (@arg FILE: "Loads the given Datalog file")
        (@arg INTERPRETER: -i +takes_value "The interpreter to use. One of: naive")
        (@arg STMTS: ... -e +takes_value "A statement to run instead of starting a REPL")
    ).get_matches();

    match run(matches) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err.display_chain());
            eprintln!("Exiting with error...");
            exit(1);
        }
    }
}

fn run(matches: ArgMatches) -> Result<()> {
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
            let stmt = stmt.parse()?;
            interpreter.run_stmt(stmt)?;
        }
        Ok(())
    } else {
        run_repl(interpreter)
    }
}

fn run_repl(interpreter: DynamicInterpreter) -> Result<()> {
    let mut con = Context::new();

    loop {
        match con.read_line("?- ", &mut |_| {}) {
            Ok(line) => {
                con.history.push(line.into())?;
            }
            Err(err) => match err.kind() {
                IoErrorKind::Interrupted => continue,
                IoErrorKind::UnexpectedEof => break,
                _ => return Err(err.into()),
            },
        }
    }

    con.history.commit_history();
    Ok(())
}
