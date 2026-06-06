use core::ffi::c_void;

pub mod stall;
#[allow(unused)]
#[repr(C)]
pub struct Misc{
    pub set_watch_dog_timer: *mut c_void,
    pub stall: *mut c_void,
    pub copy_mem: *mut c_void,
    pub set_mem: *mut c_void,
    pub get_next_monotonic_count: *mut c_void,
    pub install_configuration_table: *mut c_void,
    pub calculate_crc32: *mut c_void
}
