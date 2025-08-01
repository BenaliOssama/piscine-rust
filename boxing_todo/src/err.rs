use std::{error::Error, fmt::Display};
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum ParseErr {
    // expected public fields
    Empty,
    Malformed(Box<dyn error::Error>),
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse todo file")
    }
}

impl Error for ParseErr {
}

#[derive(Debug)]
pub struct ReadErr {
    // expected public fields
    pub child_err : Box<dyn Error>
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse todo file")
    }
}

impl Error for ReadErr {
}
