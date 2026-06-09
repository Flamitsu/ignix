// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    config::{LinuxTags, Routes},
    errors::{IgnixError, cmd},
};
use std::{
    fs::{self, File, read_to_string},
    io::{Read, Write},
};
/* entropy_source needs to be mutable, because if you read something you are "modifying" it.
In theory it is just modifying the cursor position.*/
pub fn get_random<SOURCE: Read>(
    mut entropy_source: SOURCE,
    buffer: &mut [u8],
) -> Result<(), IgnixError> {
    entropy_source.read_exact(buffer)?;
    Ok(())
}

pub struct SystemInfo {
    pub options: String,
    pub title: String,
    pub sort_key: String,
    pub machine_id: String,
}

impl SystemInfo {
    pub fn new() -> Result<SystemInfo, IgnixError> {
        let options: String = Self::filter_cmdline(&read_to_string(Routes::CMDLINE)?);
        let mut title: Option<String> = None;
        let mut sort_key: Option<String> = None;
        let file: String = read_to_string(Routes::OS_RELEASE)?;
        let machine_id: String = setup_machine_id()?;
        for line in file.lines() {
            if let Some(line_title) = line.strip_prefix(LinuxTags::PRETTY_NAME) {
                title = Some(line_title.to_string());
            }
            if let Some(line_sort_key) = line.strip_prefix(LinuxTags::OS_ID) {
                sort_key = Some(line_sort_key.to_string());
            }
        }
        let title = match title {
            Some(value) => value,
            None => Err(cmd::Error::KeyValueMissing(
                "title".into(),
                Routes::OS_RELEASE.into(),
            ))?,
        };
        let sort_key = match sort_key {
            Some(value) => value,
            None => Err(cmd::Error::KeyValueMissing(
                "sort_key".into(),
                Routes::OS_RELEASE.into(),
            ))?,
        };
        Ok(SystemInfo {
            options,
            title,
            sort_key,
            machine_id,
        })
    }
    pub fn filter_cmdline(cmdline_buffer: &str) -> String {
        const INITRD_ARGUMENT: &str = "initrd=";
        let cmdline = cmdline_buffer.split_whitespace();
        let mut filtered_cmdline = String::new();
        for arg in cmdline {
            if arg.starts_with(INITRD_ARGUMENT) {
                continue;
            } else {
                // I put this so the first push doesn't get an extra unneeded char.
                if !filtered_cmdline.is_empty() {
                    filtered_cmdline.push(' ');
                }
                filtered_cmdline.push_str(arg);
            }
        }
        filtered_cmdline
    }
}
/// First tries to get the machine id with ETC_MACHINE_ID or DBUS_MACHINE_ID, if isn't succesful,
/// it will try to create it by it's own.
pub fn setup_machine_id() -> Result<String, IgnixError> {
    let mut machine_id = String::with_capacity(33);
    let paths = [Routes::ETC_MACHINE_ID, Routes::DBUS_MACHINE_ID];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            machine_id.push_str(&content);
            if !machine_id.is_empty() {
                return Ok(machine_id.trim().into());
            }
        }
    }
    {
        let mut buffer: [u8; 16] = [0u8; 16];
        let rng = File::open(Routes::RNG_SOURCE)?;
        get_random(rng, &mut buffer)?;
        // Iters over the string and format it, and then fuses them into one string.
        let id_hex: String = buffer.iter().map(|b| format!("{:02x}", b)).collect();
        machine_id.push_str(&id_hex);
    }
    {
        let mut atomic_etc_machine_id = File::create(Routes::AT_ETC_MACHINE_ID)?;
        // I use as_bytes() because it is in hexadecimal
        atomic_etc_machine_id.write_all(format!("{}\n", machine_id).as_bytes())?;
        atomic_etc_machine_id.sync_all()?;

        fs::rename(Routes::AT_ETC_MACHINE_ID, Routes::ETC_MACHINE_ID)?;
        // I do this because after the rename, needs to sync again to save the new name.
        let dir = File::open("/etc")?;
        dir.sync_all()?;
    }
    Ok(machine_id)
}

/// This function formats from little endian bytes to a String
pub fn format_uuid(guid: &[u8; 16]) -> Result<String, IgnixError> {
    let data1 = u32::from_le_bytes(guid[0..4].try_into()?);
    let data2 = u16::from_le_bytes(guid[4..6].try_into()?);
    let data3 = u16::from_le_bytes(guid[6..8].try_into()?);
    Ok(
        // If the field isn't big enough the format says to add padding. x is for small letters.
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            data1,
            data2,
            data3,
            guid[8],
            guid[9],
            guid[10],
            guid[11],
            guid[12],
            guid[13],
            guid[14],
            guid[15]
        ),
    )
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn test_format_uuid() {
        let mut dummy_buffer: [u8; 16] = [0u8; 16];
        fill_test_data(&mut dummy_buffer, 0, 16, FillMode::Random);
        let data1 = u32::from_le_bytes(dummy_buffer[0..4].try_into().expect("Conversion error"));
        let data2 = u16::from_le_bytes(dummy_buffer[4..6].try_into().expect("Conversion error"));
        let data3 = u16::from_le_bytes(dummy_buffer[6..8].try_into().expect("Conversion error"));
        let result = format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            data1,
            data2,
            data3,
            dummy_buffer[8],
            dummy_buffer[9],
            dummy_buffer[10],
            dummy_buffer[11],
            dummy_buffer[12],
            dummy_buffer[13],
            dummy_buffer[14],
            dummy_buffer[15]
        );
        assert_eq!(format_uuid(&dummy_buffer).expect("Function error"), result)
    }

    #[test]
    fn test_format_uuid_failed() {
        let mut dummy_buffer: [u8; 16] = [0u8; 16];
        fill_test_data(&mut dummy_buffer, 0, 16, FillMode::Random);
        let data1 = u32::from_le_bytes(dummy_buffer[0..4].try_into().expect("Conversion error"));
        let data2 = u16::from_le_bytes(dummy_buffer[4..6].try_into().expect("Conversion error"));
        let data3 = u16::from_le_bytes(dummy_buffer[6..8].try_into().expect("Conversion error"));
        let result = format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            data1,
            data2,
            data3,
            dummy_buffer[8],
            dummy_buffer[9],
            dummy_buffer[10],
            dummy_buffer[11],
            dummy_buffer[12],
            dummy_buffer[13],
            dummy_buffer[14],
            dummy_buffer[15]
        );
        fill_test_data(&mut dummy_buffer, 0, 16, FillMode::Sequential);
        assert_ne!(format_uuid(&dummy_buffer).expect("Function error"), result)
    }
}
