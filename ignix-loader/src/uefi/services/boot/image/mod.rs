use core::ffi::c_void;

mod exit_boot_services;
#[allow(unused)]
#[repr(C)]
pub struct Image{
    load_image: *mut c_void,
    start_image: *mut c_void,
    unload_image: *mut c_void,
    efi_image_entry_point: *mut c_void,
    exit: *mut c_void,
    exit_boot_services: *mut c_void
}
