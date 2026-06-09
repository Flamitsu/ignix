// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    boot::{disk::DiskScanner, esp},
    cli::{args::InstallOptions, validate::ask_user_confirmation},
    errors::IgnixError,
};
pub fn install_ignix(options: InstallOptions) -> Result<(), IgnixError> {
    if !options.force {
        ask_user_confirmation("install")?;
    }
    let scanner = DiskScanner::new(options.allow_virtual, options.removable_device);
    let esp = scanner.find_compatible_esp()?;
    esp::create_ignix_structure(&esp, &options.efi_bin, options.no_nvram, options.force)?;
    Ok(())
}
