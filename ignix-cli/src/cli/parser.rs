// SPDX-License-Identifier: GPL-3.0-only
use crate::IgnixError;
use crate::errors::cmd;
use crate::config::Flag;
use std::path::PathBuf;
use crate::cli::validate;
#[allow(unused)]
pub fn parse_prefixed_arg( arg: &str, efi: &mut Option<PathBuf>) 
    -> Result<(), IgnixError> {
    /*if let Some(path) = arg.strip_prefix(Flag::INSTALL_ROUTE) {
        *route = Some(validate::is_valid_install_path(path)?);
    } else */
    if let Some(path) = arg.strip_prefix(Flag::EFI_BIN_PATH) {
        *efi = Some(validate::is_valid_efi_bin_path(path)?);
    } else {
        Err(cmd::Error::InvalidArgument(arg.to_string()))?
    }
    Ok(())
}
