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

const USAGE: &str = r#"
usage: keyb OPTIONS
       keyb --help

  Echoes individual bytes read from the terminal until ^D is pressed. If STDIN
  is not a terminal, then the input stream is processed as a sequence of bytes
  and terminates.

  This program is useful for observing sequences of bytes generated by various
  combinations of keystrokes. For example, Unicode characters would be encoded
  as a sequence of bytes.

  By default, each bytes read from the terminal is displayed on a single line
  of output as follows:

    ddd xx ooo [name]

  where ddd, xx, and ooo are the decimal, hexadecimal, and octal forms, and
  [name] is a name printed only when bytes are in the range of [0,127].

  Any of --dec, --hex, --oct, and --name can be used to print only those forms
  instead of the default behavior of printing all. However, --name by itself
  implies that all forms will be printed.

  optional:
    --dec, -d    : print decimal form
    --hex, -h    : print hexadecimal form
    --oct, -o    : print octal form
    --name, -n   : do not print description
"#;

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
    println!("press ^D to exit");
    let term = term::init()?;
    let mut tty = io::stdin().bytes();
    loop {
        match tty.next().transpose()? {
            Some(b'\x04') => break,
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
