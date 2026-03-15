pub mod cmd;
pub mod nvram;
pub mod io;
#[derive(Debug)]
pub enum SparkError {
    Cmd(cmd::Error),
    Nvram(nvram::Error),
    Io(io::Error),
}

impl std::fmt::Display for SparkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cmd(e) => write!(f, "{}", e),
            Self::Nvram(e) => write!(f, "NVRAM: {}", e),
            Self::Io(e) => write!(f,"IO: {}",e),
        }
    }
}

impl std::error::Error for SparkError {}
// Conversions from a type of error to another (keeps compatibility)
impl From<cmd::Error> for SparkError {
    fn from(err: cmd::Error) -> Self {
        Self::Cmd(err)
    }
}

impl From<nvram::Error> for SparkError {
    fn from(err: nvram::Error) -> Self {
        Self::Nvram(err)
    }
}
// Converts from std::io::Error to SparkError 
impl From<std::io::Error> for SparkError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(io::Error::from(err))
    }
}
// This packages the result to SparkError::Io
impl From<io::Error> for SparkError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
