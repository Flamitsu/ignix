#[derive(Debug)]
/// Errors related to the usage of the spark command. Like for example an invalid argument 
pub enum Error{
    InvalidArgument(String),
    EFINotFound(String),
    UserAborted,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            Error::EFINotFound(path)=>write!(f,"Not valid EFI binary have been found in {}",path),
            Error::UserAborted => write!(f, "User aborted the process."),
        }
    }
}

impl std::error::Error for Error {}
