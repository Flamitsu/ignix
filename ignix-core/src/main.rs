// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
use core::time::Duration;
#[allow(unused)]
mod config;
#[allow(unused)]
use config::*;
use ignix_sdk::{
    init::SYSTEM_TABLE,
    println,
    protocol::{
        file_protocol::{FileAttributes, FileProtocolWrapper, OpenModes},
        loaded_image::LoadedImageProtocol,
        simple_file_system_protocol::{SimpleFileSystemProtocol, SimpleFileSystemProtocolWrapper},
    },
    str_utf16,
    table::SystemTable,
    types::{Handle, IgnixError, OpenProtocolAttributes, Status, Uuid},
};

#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    SYSTEM_TABLE.set(system_table).unwrap();
    if let Err(e) = run(image_handle) {
        println!("ERROR: {}", e);
        return e.as_status();
    }
    Status::SUCCESS
}

#[allow(unused)]
fn run(handle: Handle) -> Result<(), IgnixError> {
    let bt = SYSTEM_TABLE.get().unwrap().get_boot_services().unwrap();
    let mut fs = open_root_fs(handle)?;

    let loader_config: LoaderConfig = read_config(&mut fs)?;
    println!("Timeout time: {}", loader_config.timeout);

    let timeout_ms = loader_config.timeout * 1000;
    let mut elapsed_ms = 0;
    /* Since UEFI is single threaded, need to detect input, refresh entries etc at 100ms (I mean, I
     * could have used events but... this is easier and less unstable)*/
    while elapsed_ms < timeout_ms {
        println!("{}", elapsed_ms);
        bt.stall(Duration::from_millis(STEP_MS.into()))?;
        elapsed_ms += STEP_MS as usize;
    }
    bt.stall(Duration::from_secs(30))?;
    load_kernel()?;
    Ok(())
}

#[allow(unused)]
fn read_config(fs: &mut FileProtocolWrapper) -> Result<LoaderConfig, IgnixError> {
    let mut timeout: usize = 0;

    let file_name = str_utf16!(CONFIG_ROUTE);
    let mut conf_file = fs.open(&file_name, OpenModes::READ, FileAttributes::NONE)?;
    let mut buffer: [u8; 512] = [0u8; 512];
    let valid_bytes = conf_file.read(&mut buffer)?;
    let mut bytes = &buffer[..valid_bytes];

    while !bytes.is_empty() {
        if bytes[0] == b'#' {
            while !bytes.is_empty() && bytes[0] != b'\n' {
                bytes = &bytes[1..];
            }

            if !bytes.is_empty() {
                bytes = &bytes[1..]; // This is basically so \n is skipped
            }

            continue;
        }

        if bytes.starts_with(ConfigKeywords::TIMEOUT) {
            bytes = &bytes[ConfigKeywords::TIMEOUT.len()..];
            if !bytes.is_empty() && bytes[0] == b' ' {
                bytes = &bytes[1..];
            }

            let mut parsed_value: usize = 0;
            let mut found_digit = false;
            while !bytes.is_empty() && bytes[0].is_ascii_digit() {
                let digit = (bytes[0] - b'0') as usize;
                if let Some(number) = parsed_value
                    .checked_mul(10)
                    .and_then(|v| v.checked_add(digit))
                {
                    parsed_value = number;
                    found_digit = true;
                }
                bytes = &bytes[1..];
            }
            if found_digit {
                timeout = parsed_value;
            }
            continue;
        }
        bytes = &bytes[1..]
    }
    Ok(LoaderConfig { timeout })
}
#[allow(unused)]
fn load_entries<'a>(// fs: &mut FileProtocolWrapper
) -> Result<LoaderData<'a>, IgnixError> {
    // let entries = fs.open(&str_utf16!(ENTRIES_DIR), OpenModes::READ, FileAttributes::DIRECTORY)?;
    Ok(LoaderData::new())
}

fn load_kernel() -> Result<(), IgnixError> {
    Ok(())
}

fn open_root_fs(handle: Handle) -> Result<FileProtocolWrapper, IgnixError> {
    let bt = SYSTEM_TABLE.get().unwrap().get_boot_services().unwrap();
    let image_guard = bt.open_protocol::<LoadedImageProtocol>(
        handle,
        &LoadedImageProtocol::GUID,
        handle,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;
    let device_handle = image_guard.device_handle;
    let fs_guard = bt.open_protocol::<SimpleFileSystemProtocol>(
        device_handle,
        &SimpleFileSystemProtocol::GUID,
        handle,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;
    let mut sfsp = unsafe { SimpleFileSystemProtocolWrapper::new(fs_guard.interface) };
    sfsp.open_volume()
}
