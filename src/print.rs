use crate::opt::Options;

pub struct Printer {
    dec: bool,
    hex: bool,
    oct: bool,
    name: bool,
}

impl Printer {
    pub fn new(opts: &Options) -> Printer {
        let all = !opts.dec && !opts.hex && !opts.oct;
        Printer {
            dec: all || opts.dec,
            hex: all || opts.hex,
            oct: all || opts.oct,
            name: all || opts.name,
        }
    }

    pub fn print(&self, c: u8) {
        if self.dec {
            print!("{:3}", c);
        }
        if self.hex {
            if self.dec {
                print!(" ");
            }
            print!("{:02X}", c);
        }
        if self.oct {
            if self.dec || self.hex {
                print!(" ");
            }
            print!("{:3o}", c);
        }
        if self.name {
            if let Some(s) = describe(c) {
                print!(" {}", s);
            }
        }
        println!();
    }
}

fn describe(c: u8) -> Option<&'static str> {
    if c > 127 {
        None
    } else {
        let s = if c < 32 {
            CONTROL_CHARS[c as usize]
        } else {
            PRINTABLE_CHARS[c as usize - 32]
        };
        Some(s)
    }
}

const CONTROL_CHARS: [&str; 32] = [
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
];

const PRINTABLE_CHARS: [&str; 96] = [
    " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@", "A", "B", "C", "D", "E",
    "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
    "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~",
    "^? DEL",
];
