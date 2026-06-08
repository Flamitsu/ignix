// SPDX-License-Identifier: GPL-3.0-only
use std::fs::{read_dir, File};
use crate::{boot::gpt, config::LIMITS, utils};
use std::path::PathBuf;
use crate::{boot::sysfs, boot::esp, config::Routes, errors::IgnixError};
pub struct EspPartition{
    pub device_name: String,
    pub mountpoint: PathBuf,
    pub part_uuid: String,
    pub disk_sysfs_route: PathBuf
}

pub struct DiskScanner{
    allow_virtual: bool,
    allow_removable: bool,
    block_route: &'static str
}

impl DiskScanner {
    pub fn new(allow_virtual: bool, allow_removable: bool) -> Self {
        Self{
            allow_virtual,
            allow_removable,
            block_route: Routes::BLOCK_DEV_ROUTE
        }
    }

    pub fn find_compatible_esp(&self) -> Result<EspPartition, IgnixError> {
        let disks=Self::get_system_disks(self.block_route,self.allow_virtual,self.allow_removable)?;
        for device in disks {
            let disk_sysfs_route = PathBuf::from(&self.block_route).join(&device);
            let sector_size = sysfs::get_disk_sector_size(&disk_sysfs_route,Routes::LOGICAL_BLOCK_SIZE)?;
        
            let disk_file = File::open(PathBuf::from("/dev/").join(&device))?;
            let mut buffer = [0u8; LIMITS.buffer_size];
            gpt::get_gpt_structure(sector_size, &disk_file, &mut buffer)?;

            if !gpt::is_disk_efi_signed(&buffer) {
                continue;
            }

            let gpt_header_size = gpt::get_gpt_header_size(&buffer)?;
            if !gpt::validate_crc32_header_checksum(&buffer, gpt_header_size)? {
                continue;
            }

            let gpt_max_partitions = gpt::get_max_gpt_partition(&buffer)?;
            let gpt_entry_size = gpt::get_partition_max_size(&buffer)?;
            let part_array_start = gpt::get_partition_array_start(&buffer)?;

            let Some(part_guid) = gpt::get_esp_guid(
                &buffer, 
                gpt_max_partitions, 
                gpt_entry_size, 
                sector_size, 
                part_array_start
            )? else { 
                continue; 
            };

            let guid_string = utils::format_uuid(&part_guid)?;
        
            let Some(partition_name) = sysfs::get_esp_partition(&device, &disk_sysfs_route, &guid_string)? else {
                continue;
            };

            let Some(mountpoint) = esp::get_esp_mountpoint(&partition_name)? else {
                continue;
            };

            return Ok(EspPartition {
                device_name: partition_name,
                mountpoint,
                part_uuid: guid_string,
                disk_sysfs_route,
            });
        }

        Err(crate::errors::cmd::Error::NotEFIPartitionFound)?
    }
    /// This function gets the disks and returns the `Vec<String>` containing them depending on the arguments given in the execution.
    fn get_system_disks(block_route: &str, allow_virtual: bool, allow_removable: bool) 
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
        
            if Self::is_valid_block_device(&disk_name, allow_removable, allow_virtual)?{
                disks.push(disk_name);
            }
        }
        Ok(disks)
    }


    /// Check if a partition gets a valid block name or not depending on the arguments provided in the moment of the execution.
    fn is_valid_block_device(device_name: &str, allow_removable: bool, allow_virtual: bool) 
        -> Result<bool, IgnixError>{
        let route = PathBuf::from(Routes::BLOCK_DEV_ROUTE).join(device_name);
    
        /* If the device is a virtual device and the options says to install it in a virutal device
        * then the program will mark it as valid disk*/
        if sysfs::is_virtual_device(&route)? && !allow_virtual {
            return Ok(false);
        }

        if sysfs::is_removable_device(&route)? && !allow_removable{
            return Ok(false);
        }

        Ok(true)
    }
}
