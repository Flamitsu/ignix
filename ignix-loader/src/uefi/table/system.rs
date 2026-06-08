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
use crate::uefi::protocol::console::{SimpleTextOutput, SimpleTextOutputProtocol};
use crate::uefi::table::Header;
use crate::uefi::table::boot::{BootServices, BootServicesWrapper};
use crate::uefi::table::runtime::RuntimeServices;
use core::ffi::c_void;
// Code that is with '*mut c_void' is for structure normally. Don't even think of trying them!
#[allow(unused)]
// All structs that are here, needs the parameter #[repr(C)]
#[repr(C)]
pub struct SystemTable {
    hdr: Header,
    firmware_vendor: *mut u16,
    firmware_revision: u32,
    console_in_handle: *mut c_void,
    con_in: *mut c_void,
    console_out_handle: *mut c_void,
    con_out: *mut SimpleTextOutputProtocol,
    standard_error_handle: *mut c_void,
    std_err: *mut c_void,
    runtime_services: *mut RuntimeServices,
    boot_services: *mut BootServices,
    number_of_table_entries: usize,
    configuration_table: *mut c_void,
}

impl SystemTable{
    pub fn get_stdout(&self) -> Option<SimpleTextOutput>{
        if self.con_out.is_null(){
            return None;
        }
        Some(unsafe {
            SimpleTextOutput::new(self.con_out)
        })
    }
    #[allow(unused)]
    pub fn get_stderr(&self) -> Option<SimpleTextOutput> {
        if self.std_err.is_null() && self.con_out.is_null(){
            return None
        }
        Some( unsafe {
            SimpleTextOutput::new(self.con_out)
        })
    }
    #[allow(unused)]
    pub fn get_boot_services(&self) -> Option<BootServicesWrapper>{
        if self.boot_services.is_null() {
            return None
        }
        Some( unsafe {
            BootServicesWrapper::new(self.boot_services)
        })
    }
}
