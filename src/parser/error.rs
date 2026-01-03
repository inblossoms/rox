#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub position: usize,
}

impl Error {
    pub fn new(message: String, position: usize) -> Self {
        Self { message, position }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
