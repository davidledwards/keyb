use crate::error::{Error, Result};

pub struct Options {
    pub dec: bool,
    pub hex: bool,
    pub oct: bool,
    pub bin: bool,
    pub name: bool,
    pub help: bool,
    pub version: bool,
}

#[allow(clippy::derivable_impls, reason = "retain expressiveness")]
impl Default for Options {
    fn default() -> Options {
        Options {
            dec: false,
            hex: false,
            oct: false,
            bin: false,
            name: false,
            help: false,
            version: false,
        }
    }
}

impl Options {
    pub fn parse<T>(args: T) -> Result<Options>
    where
        T: IntoIterator<Item = String>,
    {
        let mut opts = Options::default();
        for arg in args.into_iter() {
            match arg.as_str() {
                "--dec" | "-d" => opts.dec = true,
                "--hex" | "-x" => opts.hex = true,
                "--oct" | "-o" => opts.oct = true,
                "--bin" | "-b" => opts.bin = true,
                "--name" | "-n" => opts.name = true,
                "--help" | "-h" => opts.help = true,
                "--version" | "-v" => opts.version = true,
                arg => return Err(Error::unexpected_arg(arg)),
            };
        }
        Ok(opts)
    }
}
