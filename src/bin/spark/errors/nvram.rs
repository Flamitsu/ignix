#[derive(Debug)]
#[allow(unused)]
/// Errors that are related to the NVRAM operations. For example: ReadError. 
pub enum Error {
    Write,
    Read
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Write => write!(f,"Error while trying to write in the NVRAM."),
            Error::Read => write!(f,"Error while trying to read the NVRAM")
        }
    }
}

impl std::error::Error for Error {}
