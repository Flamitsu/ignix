// SPDX-License-Identifier: GPL-3.0-only
use crate::cli::args::{AddOptions, HookHelp, InstallOptions, RemoveOptions};
use crate::cli::{parser, validate};
use crate::config::{AddFlag, Flag, Routes};
use crate::errors::{IgnixError, cmd};
use crate::utils::SystemInfo;
pub fn parse_install_args(args: &[String]) -> Result<InstallOptions, IgnixError> {
    let mut force = false;
    let mut allow_virtual = false;
    let mut no_nvram = true;
    let mut removable_device = false;
    let mut efi_bin_provided = None;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            Flag::FORCE_FLAG => force = true,
            Flag::ALLOW_VIRTUAL_FLAG => allow_virtual = true,
            Flag::NO_NVRAM => no_nvram = true,
            Flag::REMOVABLE_FLAG => removable_device = true,
            _ => parser::parse_prefixed_arg(arg, &mut efi_bin_provided)?,
        }
    }

    let efi_bin = match efi_bin_provided {
        Some(path) => path,
        None => validate::is_valid_efi_bin_path(Routes::DEFAULT_EFI_BIN_PATH)?,
    };

    Ok(InstallOptions {
        force,
        allow_virtual,
        no_nvram,
        removable_device,
        efi_bin,
    })
}

pub fn parse_remove_args(args: &[String]) -> Result<RemoveOptions, IgnixError> {
    Ok(RemoveOptions {
        force: args.iter().skip(2).any(|a| a == Flag::FORCE_FLAG),
    })
}

pub fn parse_add_args(args: &[String]) -> Result<AddOptions, IgnixError> {
    let mut kernel_version: Option<String> = None;
    let mut linux: Option<String> = None;
    let mut initrd: Vec<String> = Vec::new();
    for arg in args.iter().skip(2) {
        if let Some(val) = arg.strip_prefix(AddFlag::KERNEL_VERSION) {
            kernel_version = Some(val.to_string());
        } else if let Some(val) = arg.strip_prefix(AddFlag::LINUX) {
            linux = Some(val.to_string());
        } else if let Some(val) = arg.strip_prefix(AddFlag::INITRD) {
            initrd.push(val.to_string());
        } else {
            return Err(cmd::Error::InvalidArgument(arg.to_string()).into());
        }
    }
    let linux = match linux {
        Some(value) => value,
        None => Err(cmd::Error::KeyValueMissing(
            "linux path".into(),
            "CLI arguments".into(),
        ))?,
    };
    let kernel_version = match kernel_version {
        Some(value) => value,
        None => Err(cmd::Error::KeyValueMissing(
            "kernel version".into(),
            "CLI arguments".into(),
        ))?,
    };

    let os_info: SystemInfo = SystemInfo::new()?;
    // This is just to make the compiler to shut up (not final version)
    Ok(AddOptions {
        title: os_info.title,
        kernel_version,
        machine_id: os_info.machine_id,
        sort_key: os_info.sort_key,
        options: os_info.options,
        linux,
        initrd,
    })
}
pub fn parse_hook_args(args: &[String]) -> Result<HookHelp, IgnixError> {
    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--get-machine-id" => {
                return Ok(HookHelp {
                    get_machine_id: true,
                    get_esp_mountpoint: false,
                });
            }
            "--get-esp-mountpoint" => {
                return Ok(HookHelp {
                    get_machine_id: false,
                    get_esp_mountpoint: true,
                });
            }
            _ => Err(cmd::Error::InvalidArgument(arg.to_string()))?,
        }
    }
    Err(cmd::Error::InvalidArgument(
        "too short arguments for hook flag".into(),
    ))?
}
