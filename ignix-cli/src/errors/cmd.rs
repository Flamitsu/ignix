// SPDX-License-Identifier: GPL-3.0-only
#[derive(Debug)]
/// Errors related to the bad usage of the ignix command. Like for example an invalid argument
#[allow(unused)]
pub enum Error {
    InvalidArgument(String),
    UserAborted,
    NotEFIPartitionFound,
    KeyValueMissing(String, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            Error::UserAborted => write!(f, "User aborted the process."),
            Error::NotEFIPartitionFound => write!(f, "Not UEFI partition found in the system."),
            Error::KeyValueMissing(arg, file) => write!(f, "Missing {} value in {}", arg, file),
        }
    }
}

impl std::error::Error for Error {}
