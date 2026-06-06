use core::ffi::c_void;
#[repr(C)]
pub struct Time{
    get_time: *mut c_void,
    set_time: *mut c_void,
    get_wakeup_time: *mut c_void,
    set_wakeup_time: *mut c_void
}
