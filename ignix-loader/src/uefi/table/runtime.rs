use core::ffi::c_void;

use crate::uefi::table::header::Header;
#[allow(unused)]
#[repr(C)]
pub struct RuntimeServices{
    hdr: Header,
    // Time services 
    get_time: *mut c_void,
    set_time: *mut c_void,
    get_wakeup_time: *mut c_void,
    set_wakeup_time: *mut c_void,
    // Virtual memory services
    set_virtual_address_map: *mut c_void,
    convert_pointer: *mut c_void,
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
    query_variable_info: *mut c_void
}
pub struct RuntimeServicesWrapper{}
