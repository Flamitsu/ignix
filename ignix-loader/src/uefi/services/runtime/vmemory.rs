use core::ffi::c_void;

#[repr(C)]
pub struct Vmemory{
    set_virtual_address_map: *mut c_void,
    convert_pointer: *mut c_void
}
