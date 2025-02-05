use crate::opt::Options;

pub struct Printer {
    dec: bool,
    hex: bool,
    oct: bool,
    bin: bool,
    name: bool,
}

impl Printer {
    pub fn new(opts: &Options) -> Printer {
        let all = !opts.dec && !opts.hex && !opts.oct && !opts.bin;
        Printer {
            dec: all || opts.dec,
            hex: all || opts.hex,
            oct: all || opts.oct,
            bin: all || opts.bin,
            name: all || opts.name,
        }
    }

    pub fn print(&self, c: u8, highlight: bool) {
        if highlight {
            print!("\x1b[31m* \x1b[0m");
        } else {
            print!("  ");
        }
        if self.dec {
            print!("{c:3}");
        }
        if self.hex {
            if self.dec {
                print!(" ");
            }
            print!("{c:02x}");
        }
        if self.oct {
            if self.dec || self.hex {
                print!(" ");
            }
            print!("{c:3o}");
        }
        if self.bin {
            if self.dec || self.hex || self.oct {
                print!(" ");
            }
            print!("{c:08b}");
        }
        if self.name {
            if let Some(s) = describe(c) {
                print!(" {s}");
            }
        }
        println!();
    }
}

fn describe(c: u8) -> Option<&'static str> {
    if c > 127 {
        None
    } else {
        Some(ASCII_DESCRIPS[c as usize])
    }
}

const ASCII_DESCRIPS: [&str; 128] = [
    "^@ \"\\0\"",
    "^A",
    "^B",
    "^C",
    "^D",
    "^E",
    "^F",
    "^G \"\\a\"",
    "^H \"\\b\"",
    "^I \"\\t\"",
    "^J \"\\n\"",
    "^K \"\\v\"",
    "^L \"\\f\"",
    "^M \"\\r\"",
    "^N",
    "^O",
    "^P",
    "^Q",
    "^R",
    "^S",
    "^T",
    "^U",
    "^V",
    "^W",
    "^X",
    "^Y",
    "^Z",
    "^[ ESC",
    "^\\",
    "^]",
    "^^",
    "^_",
    " ",
    "!",
    "\"",
    "#",
    "$",
    "%",
    "&",
    "'",
    "(",
    ")",
    "*",
    "+",
    ",",
    "-",
    ".",
    "/",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    ":",
    ";",
    "<",
    "=",
    ">",
    "?",
    "@",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "[",
    "\\",
    "]",
    "^",
    "_",
    "`",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "{",
    "|",
    "}",
    "~",
    "^? DEL",
];
