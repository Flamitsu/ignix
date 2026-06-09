// SPDX-License-Identifier: GPL-3.0-only
use crate::errors::IgnixError;
use crate::errors::{cmd, io};
use std::io::{Write, stdin, stdout};
use std::path::PathBuf;

pub fn is_valid_efi_bin_path(route: &str) -> Result<PathBuf, IgnixError> {
    let path = PathBuf::from(route);
    if !path.exists() || path.extension().is_none_or(|ext| ext != "efi") {
        Err(io::Error::NotFound(path.display().to_string()))?;
    }
    Ok(path)
}

#[allow(unused)]
pub fn is_valid_install_path(route: &str) -> Result<PathBuf, IgnixError> {
    let path = PathBuf::from(route);
    if path.exists() {
        return Ok(path);
    }
    Err(io::Error::NotFound(path.display().to_string()))?
}

pub fn ask_user_confirmation(context: &str) -> Result<bool, IgnixError> {
    println!("Remember to use capital letters as shown:");
    println!("Type 'YES' to {} or 'NO' to cancel.", context);

    stdout().flush().ok();

    let mut lector = String::new();
    stdin().read_line(&mut lector).ok();

    match lector.trim() {
        "YES" => Ok(true),
        "NO" => Err(cmd::Error::UserAborted)?,
        _ => {
            eprintln!(
                "The program did not understoot the input '{}', assuming 'NO'.",
                lector
            );
            Err(cmd::Error::UserAborted)?
        }
    }
}
