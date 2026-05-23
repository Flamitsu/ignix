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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
#![no_std]
#![no_main]
mod panic_handler;
mod uefi;
use uefi::types::Status;
use uefi::table::SystemTable;
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    _image_handle: *mut core::ffi::c_void,
    system_table: *mut SystemTable,
) -> Status {
    const LOOPS: usize = 1000;
    if system_table.is_null() {
        return Status(0x1);
    }
    let con_out = unsafe { (*system_table).con_out};
    if con_out.is_null() {
        return Status(0x2);
    }
    // As UEFI uses UTF-16 and i don't have a macro yet, i need to convert it manually.
    let message = ['H' as u16, 'e' as u16, 'l' as u16, 'l' as u16, 'o' as u16,
    ' ' as u16, 'w' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, '!' as u16, 
    // '\r' is neccesary so the pointer goes to the first char of the line. This created a funny
    // bug when wasn't here the print just look like it was tabbed out lmao 
    '\r' as u16, '\n' as u16, 0];
    let mut result = Status::SUCCESS;
    for _ in 1..=LOOPS{
        unsafe { 
            result = ((*con_out).output_string)(con_out, message.as_ptr())
        }
    }
    result
}
