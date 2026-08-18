// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    table::header::Header,
    types::{
        AllocateType, DevicePathProtocol, Event, EventGroup, EventNotifyFn, EventType, Guid,
        Handle, InterfaceType, MemoryDescriptor, MemoryType, OpenProtocolAttributes,
        OpenProtocolInformationEntry, PhysicalAddress, SearchType, Status, Table, TimerDelay, Tpl,
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
    pub raise_tpl: unsafe extern "efiapi" fn(new_tpl: Tpl) -> Tpl,
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
    pub set_timer: unsafe extern "efiapi" fn(
        event: Event,
        timer_delay: TimerDelay,
        trigger_time: u64,
    ) -> Status,
    pub wait_for_event: unsafe extern "efiapi" fn(
        number_of_events: usize,
        event: *const Event,
        out_index: *mut usize,
    ) -> Status,
    pub signal_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    pub close_event: unsafe extern "efiapi" fn(event: Event) -> Status,
    pub check_event: unsafe extern "efiapi" fn(event: Event) -> Status,

    // Protocol handler services
    pub install_protocol_interface: unsafe extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *const Guid,
        interface_type: InterfaceType,
        interface: *const c_void,
    ) -> Status,
    pub reinstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        old_interface: *const c_void,
        new_interface: *const c_void,
    ) -> Status,
    pub uninstall_protocol_interface: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        interface: *const c_void,
    ) -> Status,
    // DON'T TOUCH THIS POINTER THIS SHIT TOOK ME 13 HOURS TO REALIZE IT WAS MISALIGNED BY 8 BYTES
    reserved: *mut c_void,
    pub handle_protocol: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        interface: *mut *mut c_void,
    ) -> Status,
    pub register_protocol_notify: unsafe extern "efiapi" fn(
        protocol: *const Guid,
        event: Event,
        registration: *mut *mut c_void,
    ) -> Status,
    pub locate_handle: unsafe extern "efiapi" fn(
        search_type: SearchType,
        protocol: *const Guid,
        search_key: *const c_void,
        buffer_size: *mut usize,
        buffer: *mut Handle,
    ) -> Status,
    pub locate_device_path: unsafe extern "efiapi" fn(
        protocol: *const Guid,
        device_path: *mut *const DevicePathProtocol,
        device: *mut Handle,
    ) -> Status,

    // Miscellaneous services
    pub install_configuration_table:
        unsafe extern "efiapi" fn(guid: *const Guid, table: *const c_void) -> Status,

    // Image services
    pub load_image: unsafe extern "efiapi" fn(
        boot_policy: bool,
        parent_image_handle: Handle,
        device_path: *const DevicePathProtocol,
        source_buffer: *const c_void,
        source_size: usize,
        image_handle: *mut Handle,
    ) -> Status,
    pub start_image: unsafe extern "efiapi" fn(
        image_handle: Handle,
        exit_data_size: *mut usize,
        exit_data: *mut *mut u16,
    ) -> Status,
    pub exit: unsafe extern "efiapi" fn(
        image_handle: Handle,
        exit_status: Status,
        exit_data_size: usize,
        exit_data: *mut u16,
    ) -> Status,
    pub unload_image: unsafe extern "efiapi" fn(image_handle: Handle) -> Status,
    pub exit_boot_services:
        unsafe extern "efiapi" fn(image_handle: Handle, map_key: usize) -> Status,

    // Miscellaneous services
    pub get_next_monotonic_count: unsafe extern "efiapi" fn(count: *mut u64) -> Status,
    pub stall: unsafe extern "efiapi" fn(microseconds: usize) -> Status,
    pub set_watch_dog_timer: unsafe extern "efiapi" fn(
        timeout: usize,
        watchdog_code: u64,
        data_size: usize,
        watchdog_data: *const u16,
    ) -> Status,

    // Driver support services
    pub connect_controller: unsafe extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: *const Handle,
        remaining_device_path: *const DevicePathProtocol,
        recursive: bool,
    ) -> Status,
    pub disconnect_controller: unsafe extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: Handle,
        child_handle: Handle,
    ) -> Status,

    // Open and Close protocol services
    pub open_protocol: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        interface: *mut *mut c_void,
        agent_handle: Handle,
        controller_handle: Handle,
        attributes: OpenProtocolAttributes,
    ) -> Status,
    pub close_protocol: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        agent_handle: Handle,
        controller_handle: Handle,
    ) -> Status,
    pub open_protocol_information: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol: *const Guid,
        entry_buffer: *mut *mut OpenProtocolInformationEntry,
        entry_count: *mut usize,
    ) -> Status,

    // library services
    pub protocols_per_handle: unsafe extern "efiapi" fn(
        handle: Handle,
        protocol_buffer: *mut *mut *const Guid,
        protocol_buffer_count: *mut usize,
    ) -> Status,
    pub locate_handle_buffer: unsafe extern "efiapi" fn(
        search_type: SearchType,
        protocol: *const Guid,
        search_key: *const c_void,
        no_handles: *mut usize,
        buffer: *mut *mut Handle,
    ) -> Status,
    pub locate_protocol: unsafe extern "efiapi" fn(
        protocol: *const Guid,
        registration: *const c_void,
        interface: *mut *mut c_void,
    ) -> Status,

    /*
     * Okay so let's talk about this bullshit. These two services are the worst services
     * made by a human, in fact, I believe it was made by a psychopath.
     * If you go to the page 194 of the UEFI spec 2.11, you will find these motherfuckers.
     * Those monsters are literally crawling in every single fucking CPU because one
     * Intel engineer didn't wanted to use a fucking variable size array in that moment.
     * Now those functions have "VARIABLE ARGUMENTS", what does this mean?, you may ask. You're
     * too innocent to be reading this... That's all I have to say and that's why it's not
     * implemented.
     */
    install_multiple_protocol_interfaces: *mut c_void,
    uninstall_multiple_protocol_interfaces: *mut c_void,

    // CRC32 services
    pub calculate_crc32:
        unsafe extern "efiapi" fn(data: *const c_void, data_size: usize, crc32: *mut u32) -> Status,

    // Miscellaneous services
    pub copy_mem:
        unsafe extern "efiapi" fn(destination: *const c_void, source: *const c_void, length: usize),
    pub set_mem: unsafe extern "efiapi" fn(buffer: *const c_void, size: usize, value: u8),

    // Event services
    pub create_event_ex: unsafe extern "efiapi" fn(
        event_type: EventType,
        tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        notify_context: *const c_void,
        event_group: *const EventGroup,
        efi_event: *mut Event,
    ) -> Status,
}

impl Table for BootServices {}
