// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::{
    table::header::Header,
    types::{
        AllocateType, EfiEventGroup, Event, EventNotifyFn, EventType, MemoryDescriptor, MemoryType,
        PhysicalAddress, Status, TimerDelay, Tpl,
    },
};
use core::ffi::c_void;
/*
 * This structure can be found in page 92 UEFI spec 2.11
 */
#[allow(unused)]
#[repr(C)]
pub struct BootServices {
    hdr: Header,

    // Task priority services (for more info refer to page 136 UEFI spec 2.11)
    pub raise_tpl: unsafe extern "efiapi" fn(new_tlp: Tpl) -> Tpl,
    pub restore_tpl: unsafe extern "efiapi" fn(old_tpl: Tpl),

    // Memory services
    pub allocate_pages: unsafe extern "efiapi" fn(
        alloc_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        memory: *mut PhysicalAddress,
    ) -> Status,
    pub free_pages: unsafe extern "efiapi" fn(memory: PhysicalAddress, pages: usize) -> Status,
    pub get_memory_map: unsafe extern "efiapi" fn(
        memory_map_size: *mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> Status,
    pub allocate_pool: unsafe extern "efiapi" fn(
        pool_type: MemoryType,
        size: usize,
        buffer: *mut *mut u8,
    ) -> Status,
    pub free_pool: unsafe extern "efiapi" fn(buffer: *mut u8) -> Status,

    // Event and Timer Services
    pub create_event: unsafe extern "efiapi" fn(
        event_type: EventType,
        tpl: Tpl,
        notify_function: Option<EventNotifyFn>, /* There is no problem with calling it with
                                                an option, just if someone was
                                                as stupid as I was wondering it*/
        notify_context: *mut c_void,
        out_event: *mut Event,
    ) -> Status,
    set_timer: unsafe extern "efiapi" fn(event: Event, timer_delay: TimerDelay, trigger_time: u64),
    wait_for_event: unsafe extern "efiapi" fn(
        number_of_events: usize,
        event: *mut Event,
        index: *mut usize,
    ) -> Status,
    signal_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    close_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    check_event: unsafe extern "efiapi" fn(event: Event) -> Status,

    // Protocol handler services
    install_protocol_interface: *mut c_void,
    reinstall_protocol_interface: *mut c_void,
    uninstall_protocol_interface: *mut c_void,
    reserved: *mut c_void,
    handle_protocol: *mut c_void,
    register_protocol_notify: *mut c_void,
    locate_handle: *mut c_void,
    locate_device_path: *mut c_void,
    install_configuration_table: *mut c_void,

    // Image services
    load_image: *mut c_void,
    start_image: *mut c_void,
    exit: *mut c_void,
    unload_image: *mut c_void,
    exit_boot_services: *mut c_void,

    // Miscellaneous services
    get_next_monotonic_count: *mut c_void,
    pub stall: unsafe extern "efiapi" fn(microseconds: usize) -> Status,
    set_watch_dog_timer: *mut c_void,

    // Driver support services
    connect_controller: *mut c_void,
    disconnect_controller: *mut c_void,

    // Open and Close protocol services
    open_protocol: *mut c_void,
    close_protocol: *mut c_void,
    open_protocol_information: *mut c_void,

    // library services
    protocols_per_handle: *mut c_void,
    locate_handle_buffer: *mut c_void,
    locate_protocol: *mut c_void,
    install_multiple_protocol_interfaces: *mut c_void,
    uninstall_multiple_protocol_interfaces: *mut c_void,

    // CRC32 services
    calculate_crc32: *mut c_void,

    /* Miscellaneous services
     * I KNOW THERE IS ALSO MISCELLANEOUS UP THERE IN THE FILE, but this
     * is how its implemented in the UEFI spec 2.11 page 94
     */
    copy_mem: *mut c_void,
    set_mem: *mut c_void,
    // Event services
    pub create_event_ex: unsafe extern "efiapi" fn(
        event_type: EventType,
        tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        notify_context: *const c_void,
        event_group: *const EfiEventGroup,
        efi_event: *mut Event,
    ) -> Status,
}
#[derive(Clone, Copy)]
pub struct BootServicesWrapper {
    function: *mut BootServices,
}

impl BootServicesWrapper {
    pub unsafe fn new(function: *mut BootServices) -> Self {
        Self { function }
    }
    pub fn get_method(&self) -> Option<&BootServices> {
        if self.function.is_null() {
            return None;
        }
        Some(unsafe { &*self.function })
    }
}
