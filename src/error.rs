pub use logos::Span;

pub struct Error {
    pub message: String,
    pub location: Option<Span>,
}

impl Error {
    pub fn new(message: String, location: Option<Span>) -> Error {
        Error { message, location }
    }
}
