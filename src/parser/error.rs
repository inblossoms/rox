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
