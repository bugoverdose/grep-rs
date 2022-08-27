use std::fmt;

pub enum Error {
    MissingKeyword,
    MissingPath,
    TooManyArgs(Vec<String>),
    InvalidPath(String),
    FileSystemError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingKeyword => write!(f, "Missing arguments: keyword, path"),
            Error::MissingPath => write!(f, "Missing arguments: path"),
            Error::TooManyArgs(redundant_args) => {
                let args = redundant_args.join(", ");
                write!(f, "Unnecessary arguments: {}", args)
            }
            Error::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            Error::FileSystemError => write!(f, "Internal Error"),
        }
    }
}