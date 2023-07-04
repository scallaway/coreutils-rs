use core::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Flags {
    Words,
    Lines,
    Bytes,
    Chars,
    LongestLine,
}

impl FromStr for Flags {
    // TODO: Handle this error case properly
    type Err = ();

    fn from_str(arg_str: &str) -> Result<Flags, ()> {
        match arg_str {
            "-c" | "--bytes" => Ok(Flags::Bytes),
            "-l" | "--lines" => Ok(Flags::Lines),
            "-w" | "--words" => Ok(Flags::Words),
            "-m" | "--chars" => Ok(Flags::Chars),
            "-L" | "--max-line-length" => Ok(Flags::LongestLine),
            _ => Err(()),
        }
    }
}
