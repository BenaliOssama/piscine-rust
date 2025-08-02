use std::{error::Error, fmt::Display};
use std::fmt;
use std::error;

// Change the alias to use `Box<dyn error::Error>`.
//type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum ParseErr {
    // expected public fields
    Empty,
    Malformed(Box<dyn error::Error>),
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Failed to parse todo file"),
            ParseErr::Malformed(_) => write!(f, "Failed to parse todo file"),
        }
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(err) => Some(&**err),
        }
    }
}


#[derive(Debug)]
pub struct ReadErr {
    // expected public fields
    pub child_err : Box<dyn Error>
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.child_err)
    }
}
