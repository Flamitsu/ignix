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
    types::{AllocateType, Handle, IgnixError, MemoryType, OpenProtocolAttributes, Status, Uuid},
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
    bt.stall(Duration::from_secs(1))?;
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
    let file_name = str_utf16!("vmlinuz-linux");
    let mut file = root_dir.open(file_name.as_slice(), OpenModes::READ, FileAttributes::NONE)?;

    let file_size_bytes = file.get_info()?.file_size;
    let num_pages = (file_size_bytes + 4095) / 4096;
    let buffer_size = num_pages * 4096;
    let mut source_buffer = bt.allocate_pages(
        AllocateType::AllocateAnyPages,
        MemoryType::EfiLoaderData,
        num_pages.try_into().unwrap(),
    )?;
    let kernel_slice = unsafe {
        core::slice::from_raw_parts_mut(source_buffer.as_ptr(), buffer_size.try_into().unwrap())
    };
    file.read(kernel_slice)?;
    let kernel_handle = bt.load_image(false, handle, None, Some(kernel_slice))?;

    let loaded_kernel = bt.open_protocol::<LoadedImageProtocol>(
        kernel_handle.handle.unwrap(),
        &LoadedImageProtocol::GUID,
        handle,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;
    let cmdline = str_utf16!("root=/dev/sda rw");
    unsafe {
        let interface = loaded_kernel.interface;
        (*interface).load_options = cmdline.as_ptr() as *mut core::ffi::c_void;
        (*interface).load_options_size = (cmdline.len() * 2) as u32
    }
    bt.start_image(kernel_handle).map_err(|(err, _image)| err)?;
    Ok(())
}
