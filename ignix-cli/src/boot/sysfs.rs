// SPDX-License-Identifier: GPL-3.0-only
use std::fs::{read_to_string, read_dir};
use std::path::Path;
use crate::config::LinuxTags;
use crate::IgnixError;

pub fn get_disk_partition_uevent(device_sysfs: &Path, device: &str) -> Result<Vec<String>, IgnixError>{
    let mut uevent_paths = Vec::new();
    let entries = read_dir(device_sysfs)?;
    
    for entry in entries{
        // Needs to unpack the entry because they can cause an error of lack of permission etc.
        let Ok(entry) = entry else {continue;};
        // Converts the entry from Path into a PathBuf.
        let path = entry.path();
        let Some(file_name_os) = path.file_name() else {continue;};
        let Some(file_name) = file_name_os.to_str() else {continue;};
        
        if !file_name.starts_with(device) || !path.is_dir(){
            continue;
        }
        
        let uevent_file = path.join("uevent");
        if !uevent_file.exists() {
            continue;
        }

        if let Some(subdevice) = uevent_file.to_str(){
            uevent_paths.push(subdevice.to_string());
        }
    }
    Ok(uevent_paths)
}

pub fn get_esp_partition(device: &str, sysfs_route: &Path, partition_guid: &str) 
    -> Result<Option<String>, IgnixError>{
    let subdevice_uevent = get_disk_partition_uevent(sysfs_route, device)?;
    for subdevice in subdevice_uevent{
        let archive = read_to_string(subdevice)?;
        
        let mut devname = None;
        let mut is_partition = false;
        let mut guid_matches = false;
        
        for line in archive.lines(){
            if let Some(value) = line.strip_prefix(LinuxTags::DEVNAME) {
                devname = Some(value.to_string())
            } else if let Some(value)=line.strip_prefix(LinuxTags::DEVTYPE) && value == "partition"{
                is_partition = true
            } else if let Some(value) = line.strip_prefix(LinuxTags::PARTUUID) && value == partition_guid {
                guid_matches = true;
            }
        }
        if is_partition && guid_matches{
            return Ok(devname);
        }
    } 
    Ok(None)
}

pub fn get_disk_sector_size(disk: &Path, lba_size_route: &str) -> Result<u64, IgnixError>{
    let sector_size_path = disk.join(lba_size_route);
    let value: u64 = std::fs::read_to_string(sector_size_path)?.trim().parse()?; 
    Ok(value)
}

pub fn is_virtual_device(device: &Path) -> Result<bool, IgnixError> {
    if device.join("device").exists() {
        return Ok(false);
    }
    if device.to_string_lossy().contains("/virtual/") {
        return Ok(true);
    }
    Ok(false)
}

pub fn is_removable_device(device: &Path) -> Result<bool, IgnixError> {
    let removable_path = device.join("removable");
    let final_path = if removable_path.exists() {
        removable_path
    } else if let Some(parent) = device.parent() {
        parent.join("removable")
    } else {
        removable_path
    };

    match read_to_string(final_path) {
        Ok(content) => {
            if content.trim() == "1" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Ok(false), 
    }
}
