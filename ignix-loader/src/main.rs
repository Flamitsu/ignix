/*
 * Copyright (C) 2026 Flamitsu
 *
 * This file is part of Ignix.
 *
 * Ignix is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * Ignix is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
#![no_std]
#![no_main]
mod uefi;
use uefi::table::SystemTable;
use uefi::types::Status;
use uefi::init::SYSTEM_TABLE;
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    _image_handle: *mut core::ffi::c_void,
    system_table: *mut SystemTable,
) -> Status {
    // This will put the system table in the static variable
    SYSTEM_TABLE.set(system_table).unwrap();
    if let Err(e) = run() {
        println!("ERROR: {}", e);
    }
    Status::SUCCESS
}

fn run() -> Result<(), Status> {
    Ok(())
}
