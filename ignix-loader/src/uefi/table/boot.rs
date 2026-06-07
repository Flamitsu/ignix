use core::ffi::c_void;
use crate::uefi::types::Status;
#[allow(unused)]
#[repr(C)]
pub struct BootServices{
    // EVENT MANAGMENT
    pub create_event: *mut c_void,
    pub create_event_ex: *mut c_void,
    pub close_event: *mut c_void,
    pub signal_event: *mut c_void,
    pub wait_for_event: *mut c_void,
    pub check_event: *mut c_void,
    pub set_timer: *mut c_void,
    pub raise_tpl: *mut c_void,
    pub restore_tpl: *mut c_void,
    
    // MEMORY MANAGMENT
    pub allocate_pages: *mut c_void,
    pub free_pages: *mut c_void,
    pub get_memory_map: *mut c_void,
    pub allocate_pool: *mut c_void,
    pub free_pool: *mut c_void,
    
    // PROTOCOL HANDLER
    pub install_protocol_interface: *mut c_void,
    pub uninstall_protocol_interface: *mut c_void,
    pub reinstall_protocol_interface: *mut c_void,
    pub register_protocol_notify: *mut c_void,
    pub locate_handle: *mut c_void,
    pub handle_protocol: *mut c_void,
    pub locate_device_path: *mut c_void,
    pub open_protocol: *mut c_void,
    pub close_protocol: *mut c_void,
    pub open_protocol_information: *mut c_void,
    pub connect_controller: *mut c_void,
    pub disconnect_controller: *mut c_void,
    pub protocols_per_handle: *mut c_void,
    pub locate_handle_buffer: *mut c_void,
    pub locate_protocol: *mut c_void,
    pub install_multiple_protocol_interfaces: *mut c_void,
    pub uninstall_multiple_protocol_interfaces: *mut c_void,
    
    // IMAGE MANAGING
    pub load_image: *mut c_void,
    pub start_image: *mut c_void,
    pub unload_image: *mut c_void,
    pub efi_image_entry_point: *mut c_void,
    pub exit: *mut c_void,
    pub exit_boot_services: *mut c_void,

    // MISCELANEOUS SERVICES
    pub set_watch_dog_timer: *mut c_void,
    pub stall: unsafe extern "efiapi" fn(microseconds: usize) -> Status,
    pub copy_mem: *mut c_void,
    pub set_mem: *mut c_void,
    pub get_next_monotonic_count: *mut c_void,
    pub install_configuration_table: *mut c_void,
    pub calculate_crc32: *mut c_void
}
