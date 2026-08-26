// SPDX-License-Identifier: GPL-3.0-only
#![no_std]
#![no_main]
use core::time::Duration;
#[allow(unused)]
mod config;
#[allow(unused)]
mod filesystem;
#[allow(unused)]
use config::*;
use filesystem::*;
use ignix_sdk::{
    init::{HANDLE, SYSTEM_TABLE},
    println,
    services::boot::misc::stall,
    str_utf16,
    table::SystemTable,
    types::{Handle, IgnixError, Status},
};

#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    SYSTEM_TABLE.set(system_table).unwrap();
    HANDLE.set(image_handle).unwrap();
    if let Err(e) = run() {
        println!("ERROR: {}", e);
        stall(Duration::from_secs(10)).unwrap();
        return e.as_status();
    }
    Status::SUCCESS
}

fn run() -> Result<(), IgnixError> {
    let mut fs = open_root_fs()?;
    /*let loader_config: LoaderConfig = read_config(&mut fs)?;
    let timeout_ms = loader_config.timeout * 1000;
    let mut elapsed_ms = 0;
    /* Since UEFI is single threaded, need to detect input, refresh entries etc at 100ms (I mean, I
     * could have used events but... this is easier and less unstable)*/
    while elapsed_ms < timeout_ms {
        stall(Duration::from_millis(STEP_MS.into()))?;
        elapsed_ms += STEP_MS as usize;
    }*/
    /* As the same as the cmdline string, this is hardcoded at this time to 'work on my machine™©' */
    
    load_initrds(&str_utf16!("initramfs-linux.img"), &mut fs)?;
    load_kernel(&str_utf16!("vmlinuz-linux"),&mut fs)?;
    Ok(())
}

#[allow(unused)]
fn detect_key_stroke() -> Result<Option<char>, IgnixError> {
    let stdin = SYSTEM_TABLE.get().get_stdin().unwrap();
    let key = stdin.read_key_stroke()?;
    Ok(None)
}
