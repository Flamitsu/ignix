use core::ffi::c_void;
#[repr(C)]
pub struct Memory{
    pub allocate_pages: *mut c_void,
    pub free_pages: *mut c_void,
    pub get_memory_map: *mut c_void,
    pub allocate_pool: *mut c_void,
    pub free_pool: *mut c_void
}
