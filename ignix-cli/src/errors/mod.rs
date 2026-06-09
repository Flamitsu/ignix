// SPDX-License-Identifier: GPL-3.0-only
pub mod cmd;
pub mod io;
pub mod nvram;
use core::num::ParseIntError;
use std::array::TryFromSliceError;
#[derive(Debug)]
pub enum IgnixError {
    Cmd(cmd::Error),
    Nvram(nvram::Error),
    Io(io::Error),
}

impl std::fmt::Display for IgnixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cmd(e) => write!(f, "{}", e),
            Self::Nvram(e) => write!(f, "NVRAM: {}", e),
            Self::Io(e) => write!(f, "IO: {}", e),
        }
    }
}

impl std::error::Error for IgnixError {}
// Conversions from a type of error to another (keeps compatibility)
impl From<cmd::Error> for IgnixError {
    fn from(err: cmd::Error) -> Self {
        Self::Cmd(err)
    }
}

impl From<nvram::Error> for IgnixError {
    fn from(err: nvram::Error) -> Self {
        Self::Nvram(err)
    }
}
// Converts from std::io::Error to IgnixError
impl From<std::io::Error> for IgnixError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(io::Error::from(err))
    }
}
// This packages the result to IgnixError::Io
impl From<io::Error> for IgnixError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ParseIntError> for IgnixError {
    fn from(err: ParseIntError) -> Self {
        Self::Io(io::Error::from(err))
    }
}

// Makes the try_into compatible with the '?' operator
impl From<TryFromSliceError> for IgnixError {
    fn from(err: TryFromSliceError) -> Self {
        Self::Io(io::Error::from(err))
    }
}
