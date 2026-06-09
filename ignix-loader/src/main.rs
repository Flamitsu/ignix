// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
mod uefi;
use core::time::Duration;

use uefi::init::SYSTEM_TABLE;
use uefi::table::SystemTable;
use uefi::types::Handle;
use uefi::types::Status;
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(_image_handle: *mut Handle, system_table: *mut SystemTable) -> Status {
    // This will put the system table in the static variable
    SYSTEM_TABLE.set(system_table).unwrap();
    if let Err(e) = run() {
        println!("ERROR: {}", e);
    }
    Status::SUCCESS
}

fn run() -> Result<(), Status> {
    for n in 1..=10 {
        let status = SYSTEM_TABLE
            .get()
            .unwrap()
            .get_boot_services()
            .unwrap()
            .stall(Duration::from_secs(1));
        println!("{}: STATUS: {:?}", n, status);
    }
    Ok(())
}
