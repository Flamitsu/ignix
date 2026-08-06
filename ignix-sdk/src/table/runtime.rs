// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    table::header::Header,
    types::{
        Boolean, DebugDisposition, Guid, MemoryDescriptor, ResetType, Status, Table, Time,
        TimeCapabilities, VariableAttributes,
    },
};
use core::{ffi::c_void, ptr::NonNull};
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
    pub get_variable: unsafe extern "efiapi" fn(
        variable_name: *const u16,
        vendor_guid: *const Guid,
        attributes: *mut VariableAttributes,
        data_size: *mut usize,
        data: *mut c_void,
    ) -> Status,
    pub get_next_variable_name: unsafe extern "efiapi" fn(
        variable_name_size: *mut usize,
        variable_name: *mut u16,
        vendor_guid: *mut Guid,
    ) -> Status,
    pub set_variable: unsafe extern "efiapi" fn(
        variable_name: *const u16,
        vendor_guid: *const Guid,
        attributes: VariableAttributes,
        data_size: usize,
        data: *const c_void,
    ) -> Status,

    // Miscellaneous services
    pub get_next_high_monotonic_count: unsafe extern "efiapi" fn(high_count: *mut u32) -> Status,
    pub reset_system: unsafe extern "efiapi" fn(
        reset_type: ResetType,
        reset_status: Status,
        data_size: usize,
        reset_data: *const u16,
    ),

    // UEFI 2.0 Capsule services
    // I don't think I need this services at all, so I'm not mapping them (flashing firmware)
    update_capsule: *mut c_void,
    query_capsule_capabilities: *mut c_void,

    // Miscellaneous UEFI 2.0 Service
    pub query_variable_info: unsafe extern "efiapi" fn(
        attributes: VariableAttributes,
        maximum_variables_storage_size: *mut u64,
        remaining_variable_storage_size: *mut u64,
        maximum_variable_size: *mut u64,
    ) -> Status,
}

impl Table for RuntimeServices {}

#[derive(Clone, Copy)]
#[allow(unused)]
pub struct RuntimeServicesWrapper {
    function: NonNull<RuntimeServices>,
}
#[allow(unused)]
impl RuntimeServicesWrapper {
    pub unsafe fn new(function: *mut RuntimeServices) -> Self {
        let non_null = NonNull::new(function).expect("Runtime Services pointer cannot be null");
        Self { function: non_null }
    }
    #[inline(always)]
    pub fn get_method(&self) -> &RuntimeServices {
        unsafe { self.function.as_ref() }
    }
}
