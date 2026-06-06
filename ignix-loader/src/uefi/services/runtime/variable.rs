use core::ffi::c_void;
#[repr(C)]
pub struct Variable{
    get_variable: *mut c_void,
    get_next_variable_name: *mut c_void,
    set_variable: *mut c_void,
    query_variable_info: *mut c_void
}
