// This archive is used to know which device is the correct for the installation of the ESP
use crate::errors::SparkError;
use std::fs::read_dir;

#[allow(unused)]
/// Returns the actual ESP device, so the esp.rs module can discover its path.
pub fn compatible_esp_device() -> Result<(), SparkError>{
    let aviable_disks = get_disks()?;
    for disk in aviable_disks{
        println!("{}",disk);
    }
    Ok(())
}

#[allow(unused)]
/// Returns a list of the disks in the system (block ones).
// pub fn get_disks() -> Result<Vec<&str>, SparkError>{
pub fn get_disks() -> Result<Vec<String>, SparkError>{
    // Creates a new empty vec where the found disk devices are going to be storaged
    let mut disks:Vec<String> = Vec::new();
    const BLOCK_DEV_ROUTE: &str = "/sys/block/";
    // Creates an empty vec
    let disk_devices = read_dir(BLOCK_DEV_ROUTE)?; 
    for device in disk_devices{
        
        // Unpacks the device, if there is an error it will jump to the next one 
        let Ok(disk) = device else{
            continue;
        };

        // Converts the possible disk into a string, if it fails it will jump to the next one
        let Ok(disk_name) = disk.file_name().into_string() else{
            continue;
        };
        
        // Checks if it is a valid block device, like for example 'nvmeXnY', not partitions.
        if check_valid_block_name(&disk_name){
            disks.push(disk_name);
        }
    }
    Ok(disks)
}

#[allow(unused)]
/// Checks if the provided name is valid or not. (Example: sda, nvme... Bad example: dm-0) 
pub fn check_valid_block_name(device_name: &str) -> bool {
    // If the device is a block device and not a partition then return true 
    if (device_name.contains("nvme") && device_name.contains("n")) && !device_name.contains("p"){
        return true
    }
    
    // If the device names start with vd or sd, then it will return true
    if (device_name.contains("vd") || device_name.contains("sd")){
        return true
    }
    
    false
}

#[allow(unused)]
/// Get disk logical sector size (Example: 512, 4096...)
pub fn get_disk_logical_sector_size(disk_block: String) -> Result<u16, SparkError>{
    // The route where the logical block sector size is storaged
    const BLOCK_DEV_ROUTE: &str = "/sys/block/";
    const LOGICAL_BLOCK_SS: &str = "/queue/logical_block_size";
    let complete_route = format!("{}{}{}",BLOCK_DEV_ROUTE,disk_block,LOGICAL_BLOCK_SS);
    Ok(1)
}

#[allow(unused)]
/// This function returns the number of partitions of an specific device.
pub fn get_number_partitions() -> Result<(), SparkError>{
    Ok(())
}

#[allow(unused)]
/// Checks and compare the partition type GUID
pub fn check_uefi_guid() -> Result<(), SparkError>{
    Ok(())
}
