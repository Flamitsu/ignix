use std::path::{Path, PathBuf};
use std::fs::{read_dir, read_to_string, File};
use std::io::{Seek, SeekFrom, Read};
use crate::cli::InstallOptions;
use crate::config::{DEVNAME, DEVTYPE, PARTUUID, BLOCK_DEV_ROUTE,LOGICAL_BLOCK,MAX_BUFFER_SIZE};
use crate::errors::IgnixError;
use crate::boot::gpt;
use crate::errors::cmd;

/// This function gets the disks and returns the `Vec<String>` containing them depending on the arguments given in the execution.
pub fn get_system_disks(block_route: &str, options: &InstallOptions) 
    -> Result<Vec<String>, IgnixError> {
    
    let mut disks:Vec<String> = Vec::new();
    let disk_devices = read_dir(block_route)?; 
    
    for device in disk_devices{    
        // Unpacks the device, if there is an error it will jump to the next one 
        let Ok(disk) = device else{
            continue;
        };

        // Converts the possible disk into a string, if it fails it will jump to the next one
        let Ok(disk_name) = disk.file_name().into_string() else{
            continue;
        };
        
        if is_valid_block_device(&disk_name, options)?{
            disks.push(disk_name);
        }
    }
    Ok(disks)
}

pub fn compatible_esp_partition(devices: Vec<String>) -> Result<String, IgnixError>{
    for device in devices {
        let disk_sysfs_route = PathBuf::from(BLOCK_DEV_ROUTE).join(&device);
        
        let sector_size = get_disk_sector_size(&disk_sysfs_route, LOGICAL_BLOCK)?;
        let disk = File::open(PathBuf::from("/dev/").join(&device))?;
        let buffer = get_gpt_structure(sector_size, &disk)?;
        
        if !gpt::is_disk_efi_signed(buffer)?{
            eprintln!("{device} isn't EFI signed. Skipping...");
            continue;
        }
        
        let gpt_header_size: u32 = gpt::get_gpt_header_size(buffer)?;

        if !gpt::validate_crc32_header_checksum(buffer, gpt_header_size)?{
            eprintln!("{device} is probably corrupt (GPT header). Skipping...");
            continue;
        }

        // The standard says here the max should be 128, however it is dynamic here just in case.
        let gpt_max_partitions: u32 = gpt::get_max_gpt_partition(buffer)?;
        let gpt_entry_size: u32 = gpt::get_partition_max_size(buffer)?;
        let part_array_start: u64 = gpt::get_partition_array_start(buffer)?;
        
        if !gpt::validate_crc32_partition_array_checksum(buffer, gpt_max_partitions, gpt_entry_size, part_array_start, sector_size)?{
            eprintln!("{device} is probably corrupt (partition array). Skipping...");
            continue;
        }

        let Some(part_guid) = gpt::get_esp_guid(&buffer, 
            gpt_max_partitions, 
            gpt_entry_size, 
            sector_size, 
            part_array_start)? 
        else { 
            eprintln!("Not GUID valid found in {device}"); 
            continue; 
        };
        
        let guid_string = format_partuuid(&part_guid)?;
        
        let Some(partition_name) = get_esp_partition(&device, 
            &disk_sysfs_route, 
            &guid_string)? 
        else { 
            eprintln!("Didn't found {device} with {guid_string} in the sysfs interface."); 
            continue; 
        };

        return Ok(partition_name);
    }

    Err(cmd::Error::NotEFIPartitionFound)?
}

/// Check if a partition gets a valid block name or not depending on the arguments provided in the moment of the execution.
fn is_valid_block_device(device_name: &str, options: &InstallOptions) -> Result<bool, IgnixError>{
    let route = PathBuf::from(BLOCK_DEV_ROUTE).join(device_name);
    
    /* If the device is a virtual device and the options says to install it in a virutal device
     * then the program will mark it as valid disk*/
    if is_virtual_device(&route)? && !options.allow_virtual {
        return Ok(false);
    }

    if is_removable_device(&route)? && !options.removable_device{
        return Ok(false);
    }

    Ok(true)
}

fn is_virtual_device(device: &Path) -> Result<bool, IgnixError>{
    if device.join("device").exists() {
        return Ok(false);
    }
    Ok(true)
}

fn is_removable_device(device: &Path) -> Result<bool, IgnixError>{
    let content = read_to_string(device.join("removable"))?;
    if content.trim() == "0"{
        return Ok(false)
    }
    Ok(true)
}

fn get_esp_partition(device: &str, sysfs_route: &Path, partition_guid: &str) 
    -> Result<Option<String>, IgnixError>{
    let subdevice_uevent = get_disk_partition_uevent(sysfs_route, device)?;
    for subdevice in subdevice_uevent{
        let archive = read_to_string(subdevice)?;
        
        let mut devname = None;
        let mut is_partition = false;
        let mut guid_matches = false;
        
        for line in archive.lines(){
            if let Some(value) = line.strip_prefix(DEVNAME) {
                devname = Some(value.to_string())
            } else if let Some(value) = line.strip_prefix(DEVTYPE) && value == "partition"{
                is_partition = true
            } else if let Some(value) = line.strip_prefix(PARTUUID) && value == partition_guid {
                guid_matches = true;
            }
        }
        if is_partition && guid_matches{
            return Ok(devname);
        }
    } 
    Ok(None)
}

fn get_disk_partition_uevent(device_sysfs: &Path, device: &str) -> Result<Vec<String>, IgnixError>{
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

fn format_partuuid(guid: &[u8;16]) -> Result<String, IgnixError>{
    let data1 = u32::from_le_bytes(guid[0..4].try_into()?);
    let data2 = u16::from_le_bytes(guid[4..6].try_into()?);
    let data3 = u16::from_le_bytes(guid[6..8].try_into()?);
    Ok(
        // If the field isn't big enough the format says to add padding. x is for small letters.
        format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        data1, data2, data3,
        guid[8], guid[9],
        guid[10], guid[11], guid[12], guid[13], guid[14], guid[15]
        )
    )
}

fn get_gpt_structure(lba_size: u64, mut disk: &File) -> Result<[u8;MAX_BUFFER_SIZE], IgnixError>{
    let mut buffer = [0u8;MAX_BUFFER_SIZE];
    
    disk.seek(SeekFrom::Start(lba_size))?;
    disk.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn get_disk_sector_size(disk: &Path, lba_size_route: &str) -> Result<u64, IgnixError>{
    let sector_size_path = disk.join(lba_size_route);
    let value: u64 = std::fs::read_to_string(sector_size_path)?.trim().parse()?; 
    Ok(value)
}
