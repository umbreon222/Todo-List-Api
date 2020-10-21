use std::fmt;

pub struct ValidationError {
    message: String
}

impl ValidationError {
    pub fn new(msg: &str) -> ValidationError {
        ValidationError{ message: msg.to_string() }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValidationError {{ message: {} }}", self.message)
    }
}
