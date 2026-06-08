// SPDX-License-Identifier: GPL-3.0-only
use core::ffi::c_void;

// This file is for the SimpleTextOutputProtocol and others for the console.
use crate::uefi::types::Status;
#[repr(C)]
#[allow(unused)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(this: *mut Self, extended: bool) -> Status,
    pub output_string: unsafe extern "efiapi" fn(this: *mut Self, string: *const u16) -> Status,
    pub test_string: unsafe extern "efiapi" fn(this: *mut Self, string: *const u16) -> Status,
    pub query_mode: unsafe extern "efiapi" fn(
        this: *mut Self,
        mode: usize,
        columns: *mut usize,
        rows: *mut usize,
    ) -> Status,
    pub set_mode: unsafe extern "efiapi" fn(this: *mut Self, mode: usize) -> Status,
    pub set_attribute: unsafe extern "efiapi" fn(this: *mut Self, attribute: usize) -> Status,
    pub clear_screen: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub set_cursor_position:
        unsafe extern "efiapi" fn(this: *mut Self, column: usize, rows: usize) -> Status,
    pub enable_cursor: unsafe extern "efiapi" fn(this: *mut Self, visible: bool) -> Status,
    pub mode: *mut c_void,
}

#[allow(unused)]
// This is the wrapper that allows to use SimpleTextOutputProtocol withouth unsafe lines
pub struct SimpleTextOutput {
    protocol: *mut SimpleTextOutputProtocol,
}

#[allow(unused)]
impl SimpleTextOutput{
    
    pub unsafe fn new(protocol: *mut SimpleTextOutputProtocol) -> Self {
        Self { protocol }
    }

    fn get_protocol(&self) -> Option<&SimpleTextOutputProtocol>{
        if self.protocol.is_null(){
            return None;
        }
        unsafe { Some(&*self.protocol) }
    }

    pub fn reset(&mut self, extended: bool) -> Status{
        if let Some(protocol) = self.get_protocol(){
            return unsafe {(protocol.reset)(self.protocol, extended)};
        }
        Status::INVALID_PARAMETER
    }

    pub fn output_string(&mut self, string: *const u16) -> Status{
        if let Some(protocol) = self.get_protocol(){
            return unsafe {(protocol.output_string)(self.protocol, string)};
        }
        Status::INVALID_PARAMETER
    }

    pub fn test_string(self, string: *const u16) -> Status{
        if let Some(protocol) = self.get_protocol(){
            return unsafe {(protocol.test_string)(self.protocol, string)}
        }
        Status::INVALID_PARAMETER
    }
}
