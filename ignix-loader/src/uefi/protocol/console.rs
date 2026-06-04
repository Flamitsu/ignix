/*
 * Copyright (C) 2026 Flamitsu
 *
 * This file is part of Ignix.
 *
 * Ignix is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * Ignix is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
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

    pub fn reset(&mut self, extended: bool) -> Status{
        if self.protocol.is_null(){
            return Status::INVALID_PARAMETER
        }
        let protocol = unsafe {
            &*self.protocol
        };
        unsafe {
            (protocol.reset)(self.protocol, extended)
        }
    }

    pub fn output_string(&mut self, string: *const u16) -> Status{
        if self.protocol.is_null(){
            return Status::INVALID_PARAMETER
        }
        let protocol = unsafe {
            &*self.protocol
        };
        unsafe {
            (protocol.output_string)(self.protocol, string)
        }
    }
}
