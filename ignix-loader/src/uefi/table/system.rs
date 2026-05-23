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
use crate::uefi::table::Header;
use crate::uefi::protocol::console::SimpleTextOutputProtocol;
use core::ffi::c_void;
// Code that is with '*mut c_void' is for structure normally. Don't even think of trying them!
#[allow(unused)]
#[repr(C)]
pub struct SystemTable{
    hdr: Header,
    // structure 
    firmware_vendor: *mut u16,
    // structure
    firmware_revision: u32,
    // structure
    console_in_handle: *mut c_void,
    // structure
    con_in: *mut c_void,
    // structure
    console_out_handle: *mut c_void,
    // structure
    pub con_out: *mut SimpleTextOutputProtocol,
    // structure
    standard_error_handle: *mut c_void,
    // structure
    std_err: *mut c_void,
    // structure
    runtime_services: *mut c_void,
    // structure
    boot_services: *mut c_void,
    // structure
    number_of_table_entries: usize,
    // structure
    configuration_table: *mut c_void,
}
