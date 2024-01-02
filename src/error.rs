use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
    des: String,
}

impl Error {
    pub fn new(des: &str) -> Self {
        Error {
            des: des.to_string(),
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config Error {}", self.des)
    }
}
