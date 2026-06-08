// SPDX-License-Identifier: GPL-3.0-only
use std::path::PathBuf;
pub struct InstallOptions {
    pub force: bool,
    pub allow_virtual: bool,
    pub no_nvram: bool,
    pub removable_device: bool,
    pub efi_bin: PathBuf,
}

pub struct RemoveOptions {
    pub force: bool,
}

#[allow(unused)]
pub struct AddOptions{
    pub title: String,
    pub kernel_version: String,
    pub machine_id: String,
    pub sort_key: String,
    pub options: String,
    pub linux: String,
    pub initrd: Vec<String>
}
/*
 * This is because the bash hook needs to know the esp mountpoint and i refuse to
 * depend on findmnt etc. That is why i added a new flag named "hook"
*/
pub struct HookHelp{
    pub get_machine_id: bool,
    pub get_esp_mountpoint: bool,
}
