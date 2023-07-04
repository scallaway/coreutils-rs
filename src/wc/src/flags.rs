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
            "-c" => Ok(Flags::Bytes),
            "-l" => Ok(Flags::Lines),
            "-w" => Ok(Flags::Words),
            "-m" => Ok(Flags::Chars),
            "-L" => Ok(Flags::LongestLine),
            _ => Err(()),
        }
    }
}
