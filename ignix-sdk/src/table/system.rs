// SPDX-License-Identifier: GPL-3.0-only
use crate::protocol::console::{
    SimpleTextInputProtocol, SimpleTextInputProtocolWrapper, SimpleTextOutputProtocol,
    SimpleTextOutputProtocolWrapper,
};
use crate::table::Header;
use crate::table::boot::{BootServices, BootServicesWrapper};
use crate::table::runtime::{RuntimeServices, RuntimeServicesWrapper};
use crate::types::{Handle, Table};
use core::ffi::c_void;
// Code that is with '*mut c_void' is for structure normally. Don't even think of trying them!
#[allow(unused)]
// All structs that are here, needs the parameter #[repr(C)]
#[repr(C)]
pub struct SystemTable {
    hdr: Header,
    firmware_vendor: *mut u16,
    firmware_revision: u32,
    console_in_handle: Handle,
    con_in: *mut SimpleTextInputProtocol,
    console_out_handle: Handle,
    con_out: *mut SimpleTextOutputProtocol,
    standard_error_handle: Handle,
    std_err: *mut SimpleTextOutputProtocol,
    runtime_services: *mut RuntimeServices,
    boot_services: *mut BootServices,
    number_of_table_entries: usize,
    configuration_table: *mut c_void,
}

impl Table for SystemTable {}

impl SystemTable {
    pub fn get_stdout(&self) -> Option<SimpleTextOutputProtocolWrapper> {
        if self.con_out.is_null() {
            return None;
        }
        Some(unsafe { SimpleTextOutputProtocolWrapper::new(self.con_out) })
    }
    pub fn get_stdin(&self) -> Option<SimpleTextInputProtocolWrapper> {
        if self.con_in.is_null() {
            return None;
        }
        Some(unsafe { SimpleTextInputProtocolWrapper::new(self.con_in) })
    }
    pub fn get_stderr(&self) -> Option<SimpleTextOutputProtocolWrapper> {
        if self.std_err.is_null() && self.con_out.is_null() {
            return None;
        }
        Some(unsafe { SimpleTextOutputProtocolWrapper::new(self.std_err) })
    }
    pub fn get_boot_services(&self) -> Option<BootServices> {
        if self.boot_services.is_null() {
            return None;
        }
        Some(unsafe { self.boot_services })
    }
    pub fn get_runtime_services(&self) -> Option<RuntimeServicesWrapper> {
        if self.runtime_services.is_null() {
            return None;
        }
        Some(unsafe { RuntimeServicesWrapper::new(self.runtime_services) })
    }
}
