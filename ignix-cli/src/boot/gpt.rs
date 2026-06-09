// SPDX-License-Identifier: GPL-3.0-only
use crate::boot::crc32::calculate_crc32;
use crate::config::{GptEntryOffsets, GptHeaderOffsets, GptSpecification, LIMITS};
use crate::errors::{IgnixError, io};
use std::io::{Read, Seek, SeekFrom};

pub fn is_disk_efi_signed(buffer: &[u8]) -> bool {
    buffer[GptHeaderOffsets::SIG] == GptSpecification::EFI_SIGN
}

pub fn validate_crc32_header_checksum<const SIZE: usize>(
    buffer: &[u8; SIZE],
    header_size: u32,
) -> Result<bool, IgnixError> {
    let size = header_size as usize;
    let stored_crc = u32::from_le_bytes(buffer[GptHeaderOffsets::CRC].try_into()?);

    /* Copies only the header to change the header checksum field to 0, that is how the CRC32
     * checksums says is correct.*/
    let mut header_copy = [0u8; SIZE];
    header_copy[..size].copy_from_slice(&buffer[..size]);
    header_copy[GptHeaderOffsets::CRC].fill(0);

    let compute_crc = calculate_crc32(&header_copy[..size]);

    if compute_crc == stored_crc {
        return Ok(true);
    }

    Ok(false)
}

pub fn validate_crc32_partition_array_checksum<const SIZE: usize>(
    buffer: &[u8; SIZE],
    gpt_max_partitions: u32,
    gpt_entry_size: u32,
    part_array_start: u64,
    sector_size: u64,
) -> Result<bool, IgnixError> {
    let array_size = (gpt_max_partitions * gpt_entry_size) as usize;
    let offset = ((part_array_start - 1) * sector_size) as usize;
    if offset + sector_size as usize > SIZE {
        Err(io::Error::InvalidBufferOverflow {
            context: "Partition array offset overflows the buffer".to_string(),
            found: offset + sector_size as usize,
            limit: SIZE,
        })?
    }

    let part_array_crc = u32::from_le_bytes(buffer[GptHeaderOffsets::PART_CRC].try_into()?);

    let crc32 = calculate_crc32(&buffer[offset..(offset + array_size)]);

    if part_array_crc == crc32 {
        return Ok(true);
    }
    Ok(false)
}

pub fn get_esp_guid<const MAX_BUFFER_SIZE: usize>(
    buffer: &[u8; MAX_BUFFER_SIZE],
    gpt_max_partitions: u32,
    gpt_entry_size: u32,
    sector_size: u64,
    part_array_start: u64,
) -> Result<Option<[u8; 16]>, IgnixError> {
    // This -1 here is because the iteration already skips the first LBA0 (MBR PROTECTIVE) sector.
    let offset = sector_size * (part_array_start - 1);

    for partition in 0..gpt_max_partitions {
        let entry_start = offset as usize + (partition as usize * gpt_entry_size as usize);
        let entry_end = entry_start + gpt_entry_size as usize;

        /* If you're wondering why this is a separated if instead of one,
         * its because I prefer to be clear of what happened and inform to the user the correct information
         * rather than saying '"Partition entry start" or "Partition entry end" overflows the buffer,
         * figure out by yourself which one is failing, have fun!'*/
        if entry_start > MAX_BUFFER_SIZE {
            Err(io::Error::InvalidBufferOverflow {
                context: "Partition entry start overflows.".to_string(),
                found: entry_start,
                limit: MAX_BUFFER_SIZE,
            })?
        }

        if entry_end > MAX_BUFFER_SIZE {
            Err(io::Error::InvalidBufferOverflow {
                context: "Partition entry end overflows.".to_string(),
                found: entry_end,
                limit: MAX_BUFFER_SIZE,
            })?
        }

        let gpt_array_header = &buffer[entry_start..entry_end];
        let type_guid = &gpt_array_header[GptEntryOffsets::TYPE_GUID];

        if type_guid == GptSpecification::ESP_GUID_SIG {
            let unique_guid: [u8; 16] =
                gpt_array_header[GptEntryOffsets::UNIQUE_GUID].try_into()?;
            return Ok(Some(unique_guid));
        }
    }
    Ok(None)
}

pub fn get_gpt_header_size(buffer: &[u8]) -> Result<u32, IgnixError> {
    let header_size = u32::from_le_bytes(buffer[GptHeaderOffsets::SIZE].try_into()?);
    if header_size as usize > LIMITS.header_size {
        Err(io::Error::InvalidBufferOverflow {
            context: "GPT header size".to_string(),
            found: header_size as usize,
            limit: LIMITS.header_size,
        })?
    }
    Ok(header_size)
}

pub fn get_max_gpt_partition(buffer: &[u8]) -> Result<u32, IgnixError> {
    let max_partitions = u32::from_le_bytes(buffer[GptHeaderOffsets::PART_COUNT].try_into()?);
    if max_partitions as usize > LIMITS.gpt_partitions {
        Err(io::Error::InvalidBufferOverflow {
            context: "Max partitions in disk".to_string(),
            found: max_partitions as usize,
            limit: LIMITS.gpt_partitions,
        })?
    }
    Ok(max_partitions)
}

pub fn get_partition_array_start(buffer: &[u8]) -> Result<u64, IgnixError> {
    let gpt_hdr_part_lba = u64::from_le_bytes(buffer[GptHeaderOffsets::PART_LBA].try_into()?);
    if gpt_hdr_part_lba as usize > LIMITS.header_part_lba {
        Err(io::Error::InvalidBufferOverflow {
            context: "Partition array start".to_string(),
            found: gpt_hdr_part_lba as usize,
            limit: LIMITS.header_part_lba,
        })?
    }
    Ok(gpt_hdr_part_lba)
}

pub fn get_partition_max_size(buffer: &[u8]) -> Result<u32, IgnixError> {
    let gpt_hdr_part_size = u32::from_le_bytes(buffer[GptHeaderOffsets::PART_SIZE].try_into()?);
    if gpt_hdr_part_size as usize > LIMITS.header_part_size {
        Err(io::Error::InvalidBufferOverflow {
            context: "Partition array entry".to_string(),
            found: gpt_hdr_part_size as usize,
            limit: LIMITS.header_part_size,
        })?
    }
    Ok(gpt_hdr_part_size)
}

pub fn get_gpt_structure<DISK: Read + Seek>(
    lba_size: u64,
    mut disk: DISK,
    buffer: &mut [u8],
) -> Result<(), IgnixError> {
    disk.seek(SeekFrom::Start(lba_size))?;
    disk.read_exact(buffer)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::{GptEntryOffsets, GptHeaderOffsets, GptSpecification, LIMITS};
    use std::io::Cursor;

    #[cfg(test)]
    enum FillMode {
        Sequential,
        Random,
    }

    #[cfg(test)]
    fn fill_test_data(buffer: &mut [u8], start: usize, end: usize, mode: FillMode) {
        match mode {
            FillMode::Sequential => {
                for index in start..end {
                    buffer[index] = (index - start) as u8;
                }
            }
            FillMode::Random => {
                for index in start..end {
                    // multiply a prime number and applies an XOR operation.
                    buffer[index] = ((index * 13) ^ 0xAA) as u8;
                }
            }
        }
    }

    #[test]
    fn test_is_disk_efi_signed() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        dummy_buffer[GptHeaderOffsets::SIG].copy_from_slice(&GptSpecification::EFI_SIGN);
        assert_eq!(is_disk_efi_signed(&dummy_buffer), true);
    }

    #[test]
    fn test_disk_is_not_efi_signed() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, 8, FillMode::Random);
        assert_eq!(is_disk_efi_signed(&dummy_buffer), false);
    }

    #[test]
    fn test_validate_crc32_header_checksum() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        let header_size = LIMITS.header_size;

        fill_test_data(&mut dummy_buffer, 0, header_size, FillMode::Sequential);

        // Puts at 0 CRC field before trying to calculate (important)
        dummy_buffer[GptHeaderOffsets::CRC].fill(0);
        let compute_crc = calculate_crc32(&dummy_buffer[..header_size]);
        dummy_buffer[GptHeaderOffsets::CRC].copy_from_slice(&compute_crc.to_le_bytes());

        assert_eq!(
            validate_crc32_header_checksum(&dummy_buffer, header_size as u32)
                .expect("Function returned error instead of true."),
            true,
            "Function returned false instead of true."
        );
    }

    #[test]
    fn test_validate_crc32_header_checksum_invalid() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        assert_eq!(
            validate_crc32_header_checksum(&dummy_buffer, LIMITS.header_size as u32)
                .expect("Function returned error instead of false."),
            false,
            "Function returned true instead of false."
        )
    }

    #[test]
    fn test_validate_crc32_partition_array_checksum_valid() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(
            &mut dummy_buffer,
            0,
            LIMITS.buffer_size,
            FillMode::Sequential,
        );

        let expected_crc =
            calculate_crc32(&dummy_buffer[LIMITS.lba_sector_size..LIMITS.buffer_size]);
        dummy_buffer[GptHeaderOffsets::PART_CRC].copy_from_slice(&expected_crc.to_le_bytes());

        assert_eq!(
            validate_crc32_partition_array_checksum(
                &dummy_buffer,
                LIMITS.gpt_partitions as u32,
                LIMITS.partition_entry_size as u32,
                2,
                LIMITS.lba_sector_size as u64
            )
            .expect("Function returned error instead of true."),
            true,
            "Function returned false instead of true."
        );
    }

    #[test]
    fn test_validate_crc32_partition_array_checksum_invalid() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(
            &mut dummy_buffer,
            LIMITS.lba_sector_size,
            LIMITS.buffer_size,
            FillMode::Random,
        );
        assert_eq!(
            validate_crc32_partition_array_checksum(
                &dummy_buffer,
                LIMITS.gpt_partitions as u32,
                LIMITS.partition_entry_size as u32,
                2,
                LIMITS.lba_sector_size as u64
            )
            .expect("Function returned error instead of false."),
            false,
            "Function returned true instead of false."
        )
    }

    #[test]
    fn test_get_esp_guid() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);

        let sector_size: u64 = LIMITS.lba_sector_size as u64;
        let part_array_start: u64 = 2;

        let partition_index = 0;
        let base_offset = (sector_size * (part_array_start - 1)) as usize;
        let entry_start = base_offset + (partition_index * LIMITS.partition_entry_size);
        let expected_unique_guid: [u8; 16] = [0xBB; 16];

        let type_guid_range = (entry_start + GptEntryOffsets::TYPE_GUID.start)
            ..(entry_start + GptEntryOffsets::TYPE_GUID.end);
        let unique_guid_range = (entry_start + GptEntryOffsets::UNIQUE_GUID.start)
            ..(entry_start + GptEntryOffsets::UNIQUE_GUID.end);

        dummy_buffer[type_guid_range].copy_from_slice(&GptSpecification::ESP_GUID_SIG);
        dummy_buffer[unique_guid_range].copy_from_slice(&expected_unique_guid);

        let esp = get_esp_guid(
            &dummy_buffer,
            LIMITS.gpt_partitions as u32,
            LIMITS.partition_entry_size as u32,
            sector_size,
            part_array_start,
        )
        .expect("Function returned error instead of Ok()");
        assert_eq!(
            esp,
            Some(expected_unique_guid),
            "Test should have returned Some not None."
        );
    }

    #[test]
    fn test_get_esp_guid_not_found() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);

        let esp = get_esp_guid(
            &dummy_buffer,
            LIMITS.gpt_partitions as u32,
            LIMITS.partition_entry_size as u32,
            LIMITS.lba_sector_size as u64,
            2,
        )
        .expect("Function returned error instead of Ok(None)");

        assert_eq!(esp, None, "Test should have returned None not Some.")
    }

    #[test]
    fn test_get_esp_guid_out_of_bounds() {
        let dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        let corrupted_entry_size = 99999;
        let esp = get_esp_guid(
            &dummy_buffer,
            LIMITS.gpt_partitions as u32,
            corrupted_entry_size,
            LIMITS.lba_sector_size as u64,
            2,
        );
        assert!(
            esp.is_err(),
            "Function should have returned an InvalidBufferOverflow error"
        );
    }

    #[test]
    fn test_get_gpt_header_size() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let size: [u8; 4] = u32::to_le_bytes(LIMITS.header_size as u32);
        dummy_buffer[GptHeaderOffsets::SIZE].copy_from_slice(&size);
        assert_eq!(
            get_gpt_header_size(&dummy_buffer).expect("The function returned Err instead of Ok"),
            LIMITS.header_size as u32
        )
    }

    #[test]
    fn test_get_gpt_header_size_corrupt() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let size: [u8; 4] = u32::to_le_bytes((LIMITS.header_size + 1) as u32);
        dummy_buffer[GptHeaderOffsets::SIZE].copy_from_slice(&size);
        assert!(get_gpt_header_size(&dummy_buffer).is_err())
    }

    #[test]
    fn test_get_max_gpt_partition() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let max_gpt_part: [u8; 4] = u32::to_le_bytes(LIMITS.gpt_partitions as u32);
        dummy_buffer[GptHeaderOffsets::PART_COUNT].copy_from_slice(&max_gpt_part);
        assert_eq!(
            get_max_gpt_partition(&dummy_buffer).expect("The function returned Err instead of Ok"),
            LIMITS.gpt_partitions as u32
        )
    }

    #[test]
    fn test_get_max_gpt_partition_corrupt() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let max_gpt_part: [u8; 4] = u32::to_le_bytes((LIMITS.gpt_partitions + 1) as u32);
        dummy_buffer[GptHeaderOffsets::PART_COUNT].copy_from_slice(&max_gpt_part);
        assert!(get_max_gpt_partition(&dummy_buffer).is_err())
    }

    #[test]
    fn test_get_partition_array_start() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let partition_array_start: [u8; 8] = u64::to_le_bytes(LIMITS.header_part_lba as u64);
        dummy_buffer[GptHeaderOffsets::PART_LBA].copy_from_slice(&partition_array_start);
        assert_eq!(
            get_partition_array_start(&dummy_buffer)
                .expect("The function returned Err instead of Ok"),
            LIMITS.header_part_lba as u64
        )
    }

    #[test]
    fn test_get_partition_array_start_corrupt() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let partition_array_start: [u8; 8] = u64::to_le_bytes((LIMITS.header_part_lba + 1) as u64);
        dummy_buffer[GptHeaderOffsets::PART_LBA].copy_from_slice(&partition_array_start);
        assert!(get_partition_array_start(&dummy_buffer).is_err())
    }

    #[test]
    fn test_get_partition_max_size() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let partition_max_size: [u8; 4] = u32::to_le_bytes(LIMITS.header_part_size as u32);
        dummy_buffer[GptHeaderOffsets::PART_SIZE].copy_from_slice(&partition_max_size);
        assert_eq!(
            get_partition_max_size(&dummy_buffer).expect("The function returned Err instead of Ok"),
            LIMITS.header_part_size as u32
        )
    }

    #[test]
    fn test_get_partition_max_size_corrupt() {
        let mut dummy_buffer: [u8; LIMITS.buffer_size] = [0u8; LIMITS.buffer_size];
        fill_test_data(&mut dummy_buffer, 0, LIMITS.buffer_size, FillMode::Random);
        let partition_max_size: [u8; 4] = u32::to_le_bytes((LIMITS.header_part_size + 1) as u32);
        dummy_buffer[GptHeaderOffsets::PART_SIZE].copy_from_slice(&partition_max_size);
        assert!(get_partition_max_size(&dummy_buffer).is_err())
    }

    #[test]
    fn test_get_gpt_structure() {
        let mut fake_disk = vec![0u8; 100];
        let offset = 50;
        let expected_data = [0xAA, 0xBB, 0xCC, 0xDD];
        fake_disk[offset..(offset + 4)].copy_from_slice(&expected_data);

        let disk = Cursor::new(fake_disk);
        let mut buffer = [0u8; 4];
        get_gpt_structure(offset as u64, disk, &mut buffer)
            .expect("The function returned Err instead of Ok");
        assert_eq!(buffer, expected_data)
    }

    #[test]
    fn test_get_gpt_structure_out_of_bounds() {
        let fake_disk = vec![0u8; 100];
        let disk = Cursor::new(fake_disk);
        let mut buffer = [0u8; 4];
        let offset = 150;
        assert!(get_gpt_structure(offset as u64, disk, &mut buffer).is_err());
    }
}
