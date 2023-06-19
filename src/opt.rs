use crate::error::{Error, Result};

pub struct Options {
    pub help: bool,
    pub dec: bool,
    pub hex: bool,
    pub oct: bool,
    pub bin: bool,
    pub name: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            help: false,
            dec: false,
            hex: false,
            oct: false,
            bin: false,
            name: false,
        }
    }
}

impl Options {
    pub fn parse<T>(args: T) -> Result<Options>
    where
        T: IntoIterator<Item = String>,
    {
        let mut opts = Options::default();
        let mut it = args.into_iter();
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--help" => opts.help = true,
                "--dec" | "-d" => opts.dec = true,
                "--hex" | "-h" => opts.hex = true,
                "--oct" | "-o" => opts.oct = true,
                "--bin" | "-b" => opts.bin = true,
                "--name" | "-n" => opts.name = true,
                _ => return Err(Error::Options(format!("{}: unexpected argument", arg))),
            };
        }
        Ok(opts)
    }
}
