use core::ffi::c_void;

#[allow(unused)]
#[repr(C)]
pub struct Memory{
    allocate_pages: *mut c_void,
    free_pages: *mut c_void,
    get_memory_map: *mut c_void,
    allocate_pool: *mut c_void,
    free_pool: *mut c_void
}
