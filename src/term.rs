use crate::error::Result;
use libc::c_int;
use libc::{STDIN_FILENO, TCSADRAIN, VMIN, VTIME};
use std::io::{self, stdin, IsTerminal};
use std::mem::MaybeUninit;

pub use libc::termios;

pub fn is_tty() -> bool {
    stdin().is_terminal()
}

pub fn init() -> Result<termios> {
    // Fetch current terminal info for restoration.
    let term = unsafe {
        let mut term = MaybeUninit::<termios>::uninit();
        os_result(libc::tcgetattr(STDIN_FILENO, term.as_mut_ptr()))?;
        term.assume_init()
    };

    // Shift terminal into raw mode so keystrokes can be read as typed.
    // Note that reads will block if waiting for data.
    let mut raw_term = term.clone();
    unsafe {
        libc::cfmakeraw(&mut raw_term);
        raw_term.c_cc[VMIN] = 1;
        raw_term.c_cc[VTIME] = 0;
        os_result(libc::tcsetattr(STDIN_FILENO, TCSADRAIN, &raw_term))?;
    };

    Ok(term)
}

pub fn restore(term: &termios) -> Result<()> {
    unsafe { os_result(libc::tcsetattr(STDIN_FILENO, TCSADRAIN, term)) }
}

fn os_result(err: c_int) -> Result<()> {
    if err < 0 {
        Err(io::Error::last_os_error().into())
    } else {
        Ok(())
    }
}
