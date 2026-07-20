// SPDX-License-Identifier: GPL-3.0-only
/*
 * Big disclaimer. If someone wants to do */
use crate::{
    table::header::Header,
    types::{Boolean, DebugDisposition, MemoryDescriptor, Status, Table, Time, TimeCapabilities},
};
use core::ffi::c_void;
#[allow(unused)]
#[repr(C)]
pub struct RuntimeServices {
    hdr: Header,

    // Time services
    pub get_time: unsafe extern "efiapi" fn(
        time: *mut Time,
        time_capabilities: *mut TimeCapabilities,
    ) -> Status,
    pub set_time: unsafe extern "efiapi" fn(time: *const Time) -> Status,
    pub get_wakeup_time: unsafe extern "efiapi" fn(
        enabled: *mut Boolean,
        pending: *mut Boolean,
        time: *mut Time,
    ) -> Status,
    pub set_wakeup_time: unsafe extern "efiapi" fn(enable: bool, time: *const Time) -> Status,

    // Virtual memory services
    pub set_virtual_address_map: unsafe extern "efiapi" fn(
        memory_map_size: usize,
        descriptor_size: usize,
        descriptor_version: u32,
        virtual_map: *mut MemoryDescriptor,
    ) -> Status,
    pub convert_pointer: unsafe extern "efiapi" fn(
        debug_position: DebugDisposition,
        addres: *mut *mut c_void,
    ) -> Status,

    // Variable services
    get_variable: *mut c_void,
    get_next_variable_name: *mut c_void,
    set_variable: *mut c_void,

    // Miscellaneous services
    get_next_high_monotonic_count: *mut c_void,
    reset_system: *mut c_void,

    // UEFI 2.0 Capsule services
    update_capsule: *mut c_void,
    query_capsule_capabilities: *mut c_void,

    // Miscellaneous UEFI 2.0 Service
    query_variable_info: *mut c_void,
}

impl Table for RuntimeServices {}

#[derive(Clone, Copy)]
#[allow(unused)]
pub struct RuntimeServicesWrapper {
    function: *mut RuntimeServices,
}
#[allow(unused)]
impl RuntimeServicesWrapper {
    pub unsafe fn new(function: *mut RuntimeServices) -> Self {
        Self { function }
    }
    pub fn get_method(&self) -> Option<&RuntimeServices> {
        if self.function.is_null() {
            return None;
        }
        Some(unsafe { &*self.function })
    }
}
