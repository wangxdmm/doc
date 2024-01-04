use std::{error, fmt};

#[derive(Debug)]
pub struct DocError {
    des: String,
}

impl DocError {
    pub fn new(des: &str) -> Self {
        DocError {
            des: des.to_string(),
        }
    }
}

impl error::Error for DocError {}

impl fmt::Display for DocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config Error {}", self.des)
    }
}
