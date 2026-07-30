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
        file_protocol::FileProtocolWrapper,
        loaded_image::LoadedImageProtocol,
        simple_file_system_protocol::{SimpleFileSystemProtocol, SimpleFileSystemProtocolWrapper},
    },
    table::SystemTable,
    types::{Handle, IgnixError, OpenProtocolAttributes, Status, Uuid},
};
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    SYSTEM_TABLE.set(system_table).unwrap();
    println!("efi_main ram loaded address: {:p}", efi_main as *const ());
    if let Err(e) = run(image_handle) {
        println!("ERROR: {}", e);
        return e.as_status();
    }
    Status::SUCCESS
}

#[allow(unused)]
fn run(handle: Handle) -> Result<(), IgnixError> {
    let bt = SYSTEM_TABLE.get().unwrap().get_boot_services().unwrap();
    let fs = open_root_fs(handle)?;
    let loader_config: LoaderConfig = read_config(&fs);
    
    let mut seconds: f64 = 0.0;
    /* Since UEFI is single threaded, need to detect input, refresh entries etc at 100ms */
    while seconds < loader_config.timeout as f64 {
        bt.stall(Duration::from_secs_f64(0.1))?;
        seconds += 0.1;
    }

    load_kernel()?;
    Ok(())
}

#[allow(unused)]
fn read_config(file_system: &FileProtocolWrapper) -> LoaderConfig {
    let timeout: usize = 0;
    LoaderConfig::new()
}
#[allow(unused)]
fn open_file() {}

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
    let fs_guard = bt.open_protocol::<SimpleFileSystemProtocol>(
        image_guard.handle,
        &SimpleFileSystemProtocol::GUID,
        handle,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;
    let mut sfsp = unsafe { SimpleFileSystemProtocolWrapper::new(fs_guard.interface) };
    Ok(sfsp.open_volume()?)
}
