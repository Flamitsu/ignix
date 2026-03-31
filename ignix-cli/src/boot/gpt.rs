use crate::boot::disk;
use crate::config::{BLOCK_DEV_ROUTE, LOGICAL_BLOCK};
use crate::errors::{IgnixError, io};
use crate::errors::cmd;
use crate::boot::crc32::calculate_crc32;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
// EFI SIGNATURE: in raw bytes: 'EFI PART' is 0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54 .
const EFI_PART_SIGN: [u8;8] = *b"EFI PART";
const MAX_HEADER_SIZE: usize = 4096;
const MAX_PARTITION_ARRAY_SIZE: usize = 32768;

pub fn compatible_esp_partition(devices: Vec<String>) -> Result<String, IgnixError>{

    for device in devices {
        
        let sector_size = get_disk_sector_size(&device, BLOCK_DEV_ROUTE, LOGICAL_BLOCK)?;
        let disk = File::open(format!("/dev/{}",device))?; 
        let mut buffer = get_gpt_header(sector_size, disk)?;
        
        if !is_efi_signed(buffer)?{
            continue;
        }
        
        let gpt_header_size: u32 = get_gpt_header_size(buffer)?;

        if !validate_crc32_header_checksum(&mut buffer, gpt_header_size)?{
            continue;
        }

        let gpt_max_partitions: usize = get_max_gpt_partition(buffer)?;
    }
    
    Err(cmd::Error::NotEFIPartitionFound)?
}


fn is_efi_signed(buffer: [u8;MAX_HEADER_SIZE]) -> Result<bool, IgnixError>{
    // Checks if the EFI PART signature located in bytes 1 to 8 in the LBA1 matches 
    if buffer[0..8] != EFI_PART_SIGN{
        return Ok(false);
    }
    Ok(true)
}

fn validate_crc32_header_checksum(buffer: &mut [u8;MAX_HEADER_SIZE], header_size: u32) 
    -> Result<bool, IgnixError>{
    let stored_crc_bytes: [u8;4] = buffer[16..20].try_into()?;
    let stored_crc = u32::from_le_bytes(stored_crc_bytes);
    buffer[16..20].fill(0);
    let compute_crc = calculate_crc32(&buffer[..header_size as usize]);
    
    if compute_crc == stored_crc {
        return Ok(true)
    }

    Ok(false)
}

fn validate_crc32_partition_array_checksum(buffer: [u8;MAX_HEADER_SIZE], header_size: usize) -> Result<bool, IgnixError> {
    todo!("Need to complete this function.");
}


fn get_gpt_header_size(buffer: [u8;MAX_HEADER_SIZE]) -> Result<u32, IgnixError>{
    let bytes = buffer[12..16].try_into()?;
    let header_size = u32::from_le_bytes(bytes);
    
    if header_size as usize >= MAX_HEADER_SIZE{
        return Err(io::Error::InvalidBuffer("Header size overflows. (MAX: 512B)".to_string()))?
    }

    Ok(header_size)
}

fn get_max_gpt_partition(buffer: [u8;MAX_HEADER_SIZE]) -> Result<usize, IgnixError>{
    let data = buffer[80..84].try_into()?;
    let bytes: usize = usize::from_le_bytes(data);
    Ok(bytes)
}

fn get_partition_array_start(buffer: [u8;MAX_HEADER_SIZE]) -> Result<usize, IgnixError>{
    let data = buffer[72..80].try_into()?;
    let bytes: usize = usize::from_le_bytes(data);
    Ok(bytes)
}

fn get_partition_array(lba_size: u64, mut disk: File, part_array_start: u64, 
    max_gpt_partition: u64) -> Result<[u8;MAX_PARTITION_ARRAY_SIZE], IgnixError>{
    let offset = lba_size * part_array_start;
    let mut buffer = [0u8;MAX_PARTITION_ARRAY_SIZE];

    if (max_gpt_partition * lba_size) as usize > MAX_PARTITION_ARRAY_SIZE{
        Err(io::Error::InvalidBuffer("Partition array size overflows. (MAX: 32KB)".to_string()))?
    }

    disk.seek(SeekFrom::Start(offset))?;
    disk.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn get_gpt_header(lba_size: u64, mut disk: File) -> Result<[u8;MAX_HEADER_SIZE], IgnixError>{
    let mut buffer = [0u8; MAX_HEADER_SIZE];
    
    disk.seek(SeekFrom::Start(lba_size))?;
    disk.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn get_disk_sector_size(disk: &str, block_route: &str, lba_size_route: &str)
    -> Result<u64, IgnixError>{
    // Example: /sys/class/block/nvme0n1/queue/logical_block_size
    let sector_size_path = &format!(r"{}{}{}",block_route,disk,lba_size_route);
    // Reads the value into a string and parses it into an unsigned int value.
    let value: u64 = std::fs::read_to_string(sector_size_path)?.trim().parse()?;
    
    Ok(value)
}
