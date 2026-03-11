pub mod cmd;
pub mod nvram;

#[derive(Debug)]
pub enum SparkError {
    Cmd(cmd::Error),
    NVRAM(nvram::Error),
}

impl std::fmt::Display for SparkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cmd(e) => write!(f, "{}", e),
            Self::NVRAM(e) => write!(f, "NVRAM: {}", e),
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
        Self::NVRAM(err)
    }
}
