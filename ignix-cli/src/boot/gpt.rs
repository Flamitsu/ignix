use crate::config::{EFI_PART_SIGN, MAX_BUFFER_SIZE, MAX_GPT_HEADER_SIZE, ESP_GUID_BYTES};
use crate::errors::{IgnixError, io};
use crate::boot::crc32::calculate_crc32;

pub fn is_disk_efi_signed(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<bool, IgnixError>{
    if buffer[0..8] != EFI_PART_SIGN{
        return Ok(false);
    }
    Ok(true)
}

pub fn validate_crc32_header_checksum(buffer:[u8;MAX_BUFFER_SIZE], header_size: u32) 
    -> Result<bool, IgnixError>{
    let size = header_size as usize;

    let stored_crc = u32::from_le_bytes(buffer[16..20].try_into()?);
    /* Copies only the header to change the header checksum field to 0, that is how the CRC32
     * checksums says is correct.*/
    let mut header_copy = [0u8; MAX_GPT_HEADER_SIZE];
    header_copy[..size].copy_from_slice(&buffer[..size]);
    header_copy[16..20].fill(0);
    
    let compute_crc = calculate_crc32(&header_copy[..size]);
    
    if compute_crc == stored_crc {
        return Ok(true)
    }

    Ok(false)
}

pub fn validate_crc32_partition_array_checksum(buffer: [u8;MAX_BUFFER_SIZE], gpt_max_partitions: u32, gpt_entry_size: u32, part_array_start: u64, sector_size: u64) -> Result<bool, IgnixError>{ 
    // Checks if it overflows and it is freed instantly after.
    let array_size = (gpt_max_partitions * gpt_entry_size) as usize;
    let offset = ((part_array_start - 1) * sector_size) as usize;
    
    if offset + sector_size as usize > MAX_BUFFER_SIZE{
        Err(io::Error::InvalidBufferOverflow(MAX_BUFFER_SIZE.to_string()))?
    }

    let part_array_crc = u32::from_le_bytes(buffer[88..92].try_into()?);
    
    let crc32 = calculate_crc32(&buffer[offset..(offset + array_size)]);
    
    if part_array_crc == crc32 {
        return Ok(true);
    }
    Ok(false)
}

pub fn get_esp_guid(buffer: &[u8;MAX_BUFFER_SIZE], gpt_max_partitions: u32, gpt_entry_size: u32, sector_size: u64, part_array_start: u64) -> Result<Option<[u8;16]>, IgnixError>{
    // This -1 here is because the iteration already skips the first LBA0 (MBR PROTECTIVE) sector.
    let offset = sector_size * (part_array_start - 1);
    for partition in 0..gpt_max_partitions{
        let entry_start = offset as usize + (partition as usize * gpt_entry_size as usize);
        let entry_end = entry_start + gpt_entry_size as usize;
        
        if entry_start > MAX_BUFFER_SIZE{
            Err(io::Error::InvalidBufferOverflow(MAX_BUFFER_SIZE.to_string()))?
        }
        
        let gpt_array_header = &buffer[entry_start..entry_end];
        let type_guid = &gpt_array_header[0..16];
        
        if type_guid == ESP_GUID_BYTES{
            let unique_guid: [u8;16] = gpt_array_header[16..32].try_into()?;
            return Ok(Some(unique_guid))
        }

    }
    Ok(None)
}


pub fn get_gpt_header_size(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    let header_size = u32::from_le_bytes(buffer[12..16].try_into()?);
    if header_size as usize > MAX_GPT_HEADER_SIZE {
        Err(io::Error::InvalidBufferOverflow(MAX_GPT_HEADER_SIZE.to_string()))?
    }
    Ok(header_size)
}

pub fn get_max_gpt_partition(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[80..84].try_into()?))
}

pub fn get_partition_array_start(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u64, IgnixError>{
    Ok(u64::from_le_bytes(buffer[72..80].try_into()?))
}

pub fn get_partition_max_size(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[84..88].try_into()?))
}

