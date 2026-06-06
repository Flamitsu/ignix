use core::ffi::c_void;
#[repr(C)]
pub struct Image{
    pub load_image: *mut c_void,
    pub start_image: *mut c_void,
    pub unload_image: *mut c_void,
    pub efi_image_entry_point: *mut c_void,
    pub exit: *mut c_void,
    pub exit_boot_services: *mut c_void
}
