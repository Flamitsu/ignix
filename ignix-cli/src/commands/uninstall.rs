// SPDX-License-Identifier: GPL-3.0-only
use crate::cli::args::RemoveOptions;
use crate::errors::IgnixError;
use crate::cli::validate::ask_user_confirmation;
use crate::boot::{esp, disk};
pub fn remove_ignix(options: RemoveOptions) -> Result<(), IgnixError> {
    if !options.force {
        ask_user_confirmation("uninstall")?;
    }

    let scanner = disk::DiskScanner::new(true, true); 
    let esp_target = scanner.find_compatible_esp()?;

    esp::delete_ignix_structure(&esp_target)?;
    
    Ok(())
}
