// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
use core::time::Duration;
use ignix_sdk::{
    init::SYSTEM_TABLE,
    println,
    protocol::{
        file_protocol::{FileAttributes, OpenModes},
        loaded_image::LoadedImageProtocol,
        simple_file_system_protocol::{SimpleFileSystemProtocol, SimpleFileSystemProtocolWrapper},
    },
    str_utf16,
    table::SystemTable,
    types::{Handle, IgnixError, OpenProtocolAttributes, Status, Uuid},
};
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    // This will put the system table in the static variable
    SYSTEM_TABLE.set(system_table).unwrap();
    if let Err(e) = run(image_handle) {
        println!("ERROR: {}", e);
        return e.as_status();
    }
    Status::SUCCESS
}

fn run(handle: Handle) -> Result<(), IgnixError> {
    let bt = SYSTEM_TABLE.get().unwrap().get_boot_services().unwrap();
    let loaded_image_guard = bt.open_protocol::<LoadedImageProtocol>(
        handle,
        &LoadedImageProtocol::GUID,
        handle,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;

    let device_handle = unsafe { (*loaded_image_guard.interface).device_handle };
    let guard = bt.open_protocol::<SimpleFileSystemProtocol>(
        device_handle,
        &SimpleFileSystemProtocol::GUID,
        handle,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;

    let mut sfsp = unsafe { SimpleFileSystemProtocolWrapper::new(guard.interface) };
    let mut root_dir = sfsp.open_volume()?;

    let file_name = str_utf16!("hello.txt");
    let mut file = root_dir.open(file_name.as_slice(), OpenModes::READ, FileAttributes::NONE)?;
    let mut buffer: [u8; 512] = [0u8; 512];
    let bytes_read = file.read(&mut buffer)?;

    println!("{}", bytes_read);

    if let Ok(text) = core::str::from_utf8(&buffer[..bytes_read]) {
        println!("{}", text);
    } else {
        println!("The file doesn't have utf-8 valid characters");
    }

    bt.stall(Duration::from_secs(2))?;
    Ok(())
}
