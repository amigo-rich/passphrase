use std::fmt;

pub enum Error {
    FileNotFound(String),
    ParseLine(String),
    ParseLineNumber(String),
    IO,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FileNotFound(path) => write!(f, "Failed to open file: {}", path),
            Error::ParseLine(line) => write!(f, "Parse to pair of line: {}", line),
            Error::ParseLineNumber(line) => write!(f, "Parse of number on line: {}", line),
            Error::IO => write!(f, "An IO error occurred during read_to_string"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error::IO
    }
}
