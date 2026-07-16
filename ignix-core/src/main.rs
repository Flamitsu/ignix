// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
use core::time::Duration;
use ignix_sdk::*;

use ignix_sdk::init::SYSTEM_TABLE;
use ignix_sdk::table::SystemTable;
use ignix_sdk::types::Handle;
use ignix_sdk::types::Status;

use ignix_sdk::types::AllocateType;
use ignix_sdk::types::MemoryType;
use ignix_sdk::types::PhysicalAddress;
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
    let st = SYSTEM_TABLE.get().unwrap().get_boot_services().unwrap();
    for n in 1..=100 {
        SYSTEM_TABLE
            .get()
            .unwrap()
            .get_stdout()
            .unwrap()
            .reset(true);
        let buffer = st
            .allocate_pages(AllocateType::AllocateAnyPages, MemoryType::EfiLoaderData, 2)
            .unwrap();
        let status = st.free_pages(buffer.as_ptr() as PhysicalAddress, 2);
        let buff_alloc = st.allocate_pool(MemoryType::EfiLoaderData, 900);
        let st_buf_alloc = st.free_pool(buff_alloc.unwrap());

        println!("{:?}", buffer);
        println!("{:?}", status);
        println!("{:?}", buff_alloc);
        println!("{:?}", st_buf_alloc);
        println!("{}", n);
        st.stall(Duration::from_secs(1)).unwrap();
    }
    Ok(())
}
