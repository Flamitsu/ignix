#[derive(Debug)]
/// Errors related to the usage of the spark command. Like for example an invalid argument 
pub enum Error{
    InvalidArgument(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgument(s) => write!(f, "Invalid argument: {}", s),
        }
    }
}

impl std::error::Error for Error {}
