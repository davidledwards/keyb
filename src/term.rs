use crate::error::{Error, Result};
use libc::c_int;
use libc::{STDIN_FILENO, TCSADRAIN, VMIN, VTIME};
use std::io::{stdin, IsTerminal};
use std::mem::MaybeUninit;

pub use libc::termios;

pub fn is_tty() -> bool {
    stdin().is_terminal()
}

pub fn init() -> Result<termios> {
    let term = unsafe {
        let mut term = MaybeUninit::<termios>::uninit();
        check_err(libc::tcgetattr(STDIN_FILENO, term.as_mut_ptr()))?;
        term.assume_init()
    };
    unsafe {
        let mut raw_term = term;
        libc::cfmakeraw(&mut raw_term);
        raw_term.c_cc[VMIN] = 0;
        raw_term.c_cc[VTIME] = 1;
        check_err(libc::tcsetattr(STDIN_FILENO, TCSADRAIN, &raw_term))?;
    };
    Ok(term)
}

pub fn restore(term: &termios) -> Result<()> {
    unsafe { check_err(libc::tcsetattr(STDIN_FILENO, TCSADRAIN, term)) }
}

fn check_err(err: c_int) -> Result<()> {
    if err < 0 {
        Err(Error::os())
    } else {
        Ok(())
    }
}
