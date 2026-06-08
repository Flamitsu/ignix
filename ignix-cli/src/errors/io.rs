// SPDX-License-Identifier: GPL-3.0-only
use std::io;
use std::num::ParseIntError;
use std::array::TryFromSliceError;
#[derive(Debug)]
pub enum Error {
    PermissionDenied,
    NotFound(String),
    InvalidFormat(String),
    InvalidBufferOverflow{
        context: String,
        found: usize,
        limit: usize
    },
    Unknown(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PermissionDenied => write!(f, "Access denied. Please run Ignix with higher privileges."),
            Self::NotFound(path) => write!(f, "The system could not find the specified path: {}", path),
            Self::InvalidFormat(e) => write!(f, "Data format error: {}",e),
            Self::InvalidBufferOverflow{context, found, limit} => write!(f, "Invalid {} size: {} bytes. (Limit exceeded: max allowed is {} bytes). The GPT structure may be corrupt.",context,found,limit),
            Self::Unknown(e) => write!(f, "An unexpected system error occurred: {}", e),
        }
    }
}

impl std::error::Error for Error {}
// Convers the raw std::io::Error to io::Error
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            io::ErrorKind::NotFound => Self::NotFound("unknown path".to_string()),
            _ => Self::Unknown(err),
        }
    }
}
// Converts the raw ParseIntError to the IgnixError (used in the module gpt.rs and others). 
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidFormat(err.to_string())
    }
}
impl From<TryFromSliceError> for Error {
    fn from(err: TryFromSliceError) -> Self {
        // Usamos InvalidBuffer porque un error de try_into en este contexto
        // significa que el slice de bytes no encaja en el array (ej: GUID o CRC)
        Self::InvalidFormat(format!("Slice conversion failed: {}", err))
    }
}
