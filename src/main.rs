mod error;
mod opt;
mod print;
mod term;

use crate::error::{Error, Result};
use crate::opt::Options;
use crate::print::Printer;
use std::env;
use std::io::{self, Read, Write};
use std::process::ExitCode;

const USAGE: &str = include_str!("usage.in");

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            match e {
                Error::Options(s) => {
                    println!("{}", s);
                    println!("use --help for options");
                }
                Error::IO(s) => println!("I/O error: {}", s),
            }
            ExitCode::from(1)
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

fn run() -> Result<()> {
    let opts = Options::parse(env::args().skip(1))?;
    if opts.help {
        println!("{}", USAGE);
        Ok(())
    } else if opts.version {
        println!(
            "{} {} ({} {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("KEYB_VERSION_HASH"),
            env!("KEYB_VERSION_DATE")
        );
        Ok(())
    } else {
        let pr = Printer::new(&opts);
        if term::is_tty() {
            process_tty(&pr)
        } else {
            process_file(&pr)
        }
    }
}

fn process_tty(pr: &Printer) -> Result<()> {
    println!("press ^C to exit (use 'keyb --help' for additional info)");
    let term = term::init()?;
    let mut tty = io::stdin().bytes();
    loop {
        match tty.next().transpose()? {
            Some(b'\x03') => break,
            Some(c) => {
                pr.print(c);
                print!("\r");
            }
            None => (),
        }
        io::stdout().flush()?;
    }
    term::restore(&term)?;
    Ok(())
}

fn process_file(pr: &Printer) -> Result<()> {
    for b in io::stdin().bytes() {
        pr.print(b?);
    }
    Ok(())
}
