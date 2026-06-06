use core::ffi::c_void;
#[repr(C)]
pub struct Misc{
    set_watch_dog_timer: *mut c_void,
    stall: *mut c_void,
    copy_mem: *mut c_void,
    set_mem: *mut c_void,
    get_next_monotonic_count: *mut c_void,
    install_configuration_table: *mut c_void,
    calculate_crc32: *mut c_void
}
