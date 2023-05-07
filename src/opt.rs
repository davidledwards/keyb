use crate::error::{Error, Result};

pub struct Options {
    pub help: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options { help: false }
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
                _ => return Err(Error::Options(format!("{}: unexpected argument", arg))),
            };
        }
        Ok(opts)
    }
}
