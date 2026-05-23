use core::ffi::c_void;

// This file is for the SimpleTextOutputProtocol and others for the console.
use crate::uefi::types::Status;
#[repr(C)]
#[allow(unused)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(this: *mut Self, extended: bool) -> Status,
    pub output_string: unsafe extern "efiapi" fn(this: *mut Self, string: *const u16) -> Status,
    /// This function tests if the string is compatible or not
    pub test_string: unsafe extern "efiapi" fn(this: *mut Self, string: *const u16) -> Status,
    pub query_mode: unsafe extern "efiapi" fn (this: *mut Self, mode: usize,
        columns: *mut usize, rows: *mut usize) -> Status,
    pub set_mode: unsafe extern "efiapi" fn (this: *mut Self, mode: usize) -> Status,
    pub set_attribute: unsafe extern "efiapi" fn (this: *mut Self, attribute: usize) -> Status,
    pub clear_screen: unsafe extern "efiapi" fn (this: *mut Self) -> Status,
    pub set_cursor_position: unsafe extern "efiapi" fn (this: *mut Self, column: 
        usize, rows: usize) -> Status,
    pub enable_cursor: unsafe extern "efiapi" fn (this: *mut Self, visible: bool) -> Status,
    pub mode: *mut c_void
}
