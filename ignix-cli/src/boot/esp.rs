// SPDX-License-Identifier: GPL-3.0-only
use crate::boot::disk::EspPartition;
use crate::cli::validate::ask_user_confirmation;
use crate::config::{EspStructure, Routes};
use crate::errors::{IgnixError, io};
use crate::utils::{self, setup_machine_id};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::{fs, path::Path, path::PathBuf};
pub fn create_ignix_structure(
    esp: &EspPartition,
    efi_bin: &Path,
    no_nvram: bool,
    force: bool,
) -> Result<(), IgnixError> {
    let route = &esp.mountpoint;
    let efi_fallback = route.join("EFI/BOOT/");

    if efi_fallback.exists() && no_nvram && !force {
        ask_user_confirmation("remove the BOOTX64.efi old binary to replace it with ignix one")?;
    }

    for dir in EspStructure::ESP_DIRECTORIES {
        let dir_route = route.join(dir);
        if !dir_route.exists() {
            fs::create_dir_all(&dir_route)?;
        }

        if dir.ends_with("EFI/ignix") {
            fs::copy(efi_bin, dir_route.join("ignixx64.efi.tmp"))?;
            fs::rename(
                dir_route.join("ignixx64.efi.tmp"),
                dir_route.join("ignixx64.efi"),
            )?;
        }

        if dir.ends_with("BOOT") {
            fs::copy(efi_bin, &efi_fallback.join("BOOTX64.efi.tmp"))?;
            fs::rename(
                &efi_fallback.join("BOOTX64.efi.tmp"),
                &efi_fallback.join("BOOTX64.efi"),
            )?;
        }
    }

    let mut random_seed: [u8; 32] = [0u8; 32];
    let source: File = File::open(Routes::RNG_SOURCE)?;
    utils::get_random(source, &mut random_seed)?;
    fs::write(route.join("loader/random-seed"), random_seed)?;
    let entries_route = route.join(setup_machine_id()?);
    if !entries_route.exists() {
        fs::create_dir(entries_route)?;
    }

    let ignix_dir = route.join("loader/ignix");
    let config_path = ignix_dir.join("loader.conf");
    if !config_path.exists() {
        let mut file = File::create_new(ignix_dir.join("loader.conf.tmp"))?;
        file.write_all(EspStructure::LOADER_CONFIG.as_bytes())?;
        file.sync_all()?;
        fs::rename(ignix_dir.join("loader.conf.tmp"), config_path)?;
    }
    let dir = File::open(route)?;
    dir.sync_all()?;
    Ok(())
}

pub fn delete_ignix_structure(esp: &EspPartition) -> Result<(), IgnixError> {
    let route = &esp.mountpoint;

    let bootloader_home = route.join("EFI/ignix");
    if bootloader_home.exists() {
        fs::remove_dir_all(bootloader_home)?;
    }

    let config_home = route.join("loader/ignix");
    if config_home.exists() {
        fs::remove_dir_all(config_home)?;
    }
    Ok(())
}

pub fn get_esp_mountpoint(partition_name: &str) -> Result<Option<PathBuf>, IgnixError> {
    let route = Path::new(Routes::MOUNTPOINTS);

    if !route.exists() {
        Err(io::Error::NotFound(route.display().to_string()))?
    }

    let file_content = read_to_string(route)?;
    let expected_dev_string = format!("/dev/{}", partition_name);

    for line in file_content.lines() {
        let mut parts = line.split_whitespace();

        let device = parts.next();
        let mountpoint = parts.next();
        if let (Some(dev), Some(mnt)) = (device, mountpoint)
            && dev == expected_dev_string
        {
            return Ok(Some(PathBuf::from(mnt)));
        }
    }
    Ok(None)
}
