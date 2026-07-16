// SPDX-License-Identifier: GPL-3.0-only
use crate::protocol::console::{SimpleTextOutput, SimpleTextOutputProtocol};
use crate::table::Header;
use crate::table::boot::{BootServices, BootServicesWrapper};
use crate::table::runtime::RuntimeServices;
use crate::types::Table;
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
    std_err: *mut SimpleTextOutputProtocol,
    runtime_services: *mut RuntimeServices,
    boot_services: *mut BootServices,
    number_of_table_entries: usize,
    configuration_table: *mut c_void,
}

impl Table for SystemTable {}

impl SystemTable {
    pub fn get_stdout(&self) -> Option<SimpleTextOutput> {
        if self.con_out.is_null() {
            return None;
        }
        Some(unsafe { SimpleTextOutput::new(self.con_out) })
    }
    #[allow(unused)]
    pub fn get_stderr(&self) -> Option<SimpleTextOutput> {
        if self.std_err.is_null() && self.con_out.is_null() {
            return None;
        }
        Some(unsafe { SimpleTextOutput::new(self.std_err) })
    }
    #[allow(unused)]
    pub fn get_boot_services(&self) -> Option<BootServicesWrapper> {
        if self.boot_services.is_null() {
            return None;
        }
        Some(unsafe { BootServicesWrapper::new(self.boot_services) })
    }
}
