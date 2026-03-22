// This archive is used to know which device is the correct for the installation of the ESP
use crate::errors::IgnixError;
use std::fs::read_dir;
use std::fs::File;
use std::io::{Seek,SeekFrom,Read};
use crate::errors::cmd;
use crate::config::{BLOCK_DEV_ROUTE, LOGICAL_BLOCK};
/// Returns the actual ESP partition, so the esp.rs module can discover its path.
pub fn compatible_esp_partition() -> Result<String, IgnixError>{
    let aviable_disks = get_disks(BLOCK_DEV_ROUTE)?;

    // Iterates the aviable disks in the system (plugged in)
    for disk in aviable_disks{
        
        let dev_route_disk: String = format!("/dev/{}",disk);
        // You may wonder why this is u64, whenever you use the seek method you need a u64 int
        let block_size: u64 = get_disk_logical_sector_size(&disk,BLOCK_DEV_ROUTE,LOGICAL_BLOCK)?;
        let open_disk = File::open(&dev_route_disk)?;
        
        // If the disk is not a GPT valid disk, it will jump to the next one 
        if !try_gpt_sectors(block_size, &open_disk)?{
            eprintln!("Warning: the disk {} may not be a valid GPT. Skipping...",dev_route_disk);
            continue
        }
        let uefi_partition_number = find_uefi_partition(block_size, &open_disk)?;
        /* Unpacks the number, if there is a number it will return it as a valid disk, if not, it
         * will skip to the next disk in the iteration.*/ 
        match uefi_partition_number{
            Some(number) => {return build_partition_name(&disk, number)},
            None => continue
        }
    }
    Err(cmd::Error::NotEFIPartitionFound)?
}

/// Returns a list of the disks in the system (block ones).
fn get_disks(block_route: &str) -> Result<Vec<String>, IgnixError>{
    // Creates a new empty vec where the found disk devices are going to be storaged
    let mut disks:Vec<String> = Vec::new();
    // Creates an empty vec to storage the disks
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
        
        // Checks if it is a valid block device, like for example 'nvmeXnY', not partitions.
        if is_valid_block_name(&disk_name){
            disks.push(disk_name);
        }
    }
    Ok(disks)
}

/// Checks if the provided name is valid or not. (Example: sda, nvme... Bad example: dm-0,sda1...) 
fn is_valid_block_name(device_name: &str) -> bool {
    // If the device is a block device and not a partition then return true 
    if device_name.starts_with("nvme") && !device_name.contains("p"){
        return true
    }
    
    // If the device names start with vd or sd, and its not a partition then it will return true
    if (device_name.starts_with("vd") || device_name.starts_with("sd")) && 
        !device_name.ends_with(|c: char| c.is_numeric()) {
        return true
    }
    
    false
}

fn build_partition_name(disk_name: &str, partition_number: u8) -> Result<String, IgnixError>{
    if disk_name.starts_with("nvme"){
        return Ok(format!("{}p{}",disk_name,partition_number))
    }
    Ok(format!("{}{}",disk_name,partition_number))
}

/// Get disk logical sector size (Example: 512, 4096...)
fn get_disk_logical_sector_size(disk_block: &str, block_route: &str, 
    logical_block_route: &str) -> Result<u64, IgnixError>{
    
    // The route where the logical block sector size is storaged in
    let complete_route = format!("{}{}{}",block_route,disk_block,logical_block_route);
    // Gets the LBA sector size value and converts it to a string
    let value : u64 = std::fs::read_to_string(complete_route)?.trim().parse()?;
    
    Ok(value)
}

/// Gets the proper bytes from the disk to know if it is a good GPT and have LBA1 sign "EFI PART" 
fn try_gpt_sectors(logical_block_size: u64, mut open_disk: &File) -> Result<bool, IgnixError>{
    
    // The EFI part signature. "EFI PART" in ascii, (This is in raw bytes) 
    const EFI_PART_SIGN: [u8;8] = [0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54];
    let mut buffer = [0u8;8];
    
    // Moves the pointer to the logical_block_size value (byte 512... 4096...)
    open_disk.seek(SeekFrom::Start(logical_block_size))?;
    // Reads the exact buffer and storages it 
    open_disk.read_exact(&mut buffer)?;
    
    // If the buffer matches with the EFI_PART_SIGN then it is a valid GPT partition.
    if buffer == EFI_PART_SIGN{
        return Ok(true)
    }
    Ok(false)
}

/// Checks and compare the partition type GUID and returns the partition number 
fn find_uefi_partition(logical_block_size:u64,mut open_disk:&File) -> Result<Option<u8>, IgnixError>{ 
    // This number represent where the partitions info begin. (LBA2)
    const LBA: u64 = 2;
    // Maximum partition for device in GPT
    const MAX_PARTITIONS: u8 = 128;
    /*
     * This is the partition sign for the UEFI partition
     * All this values from the last GUID in LE to the first:
     * LE and BE (GUID RAW): 28 73 2A C1 1F F8 11 D2 BA 4B 00 A0 C9 3E C9 3B 
     * String (GUID LE and BE to String): C12A7328 1FF8 D211 BA4B 00A0C93EC93B 
    */
    const ESP_GUID_BYTES: [u8;16]= [
        0x28, 0x73, 0x2A, 0xC1, // (LE) DATASET1 -> 28 73 2A C1 -> C12A7328 
        0x1F, 0xF8, // (LE) DATASET2 -> 1F F8 -> 1FF8
        0xD2, 0x11, // (LE) DATASET3 -> 11 D2 -> D211
        0xBA, 0x4B, // (BE) DATASET4 -> BA 4B -> BA4B
        0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B // (BE) DATASET5 -> 00 A0 C9 3E C9 3B -> 00A0C93EC93B  
    ];

    let offset: u64 = logical_block_size * LBA;
    // This counter will go up every partition sector that isn't empty.
    let mut partitions = 0;
    for _ in 0..MAX_PARTITIONS {
        let mut buffer = [0u8;16];
        open_disk.seek(SeekFrom::Start(offset))?;
        open_disk.read_exact(&mut buffer)?;
        // If the partition sign isn't empty then its a partition. 
        if buffer != [0u8;16]{
            partitions += 1;
        }
        // If the buffer matches the sign, then it is an ESP valid partition.
        if buffer == ESP_GUID_BYTES{
            return Ok(Some(partitions));
        }
    // Moves the cursor 112 bytes in the disk. If there is any error it will skip the iteration 
        open_disk.seek(SeekFrom::Current(112))?;
    }

    Ok(None)
}
