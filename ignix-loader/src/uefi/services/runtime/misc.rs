use core::ffi::c_void;
#[repr(C)]
pub struct Misc{
    get_next_high_monotonic_count: *mut c_void,
    reset_system: *mut c_void,
    update_capsule: *mut c_void,
    query_capsule_capabilities: *mut c_void
}
