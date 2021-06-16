use std::fmt;

pub enum Error {
    PairParseLine(String),
    MapIOError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PairParseLine(line) => write!(f, "Parse to pair of line: {}", line),
            Error::MapIOError => write!(f, "An IO error occurred during read_to_string"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error::MapIOError
    }
}
