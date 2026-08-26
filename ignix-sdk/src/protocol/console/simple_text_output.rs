// SPDX-License-Identifier: GPL-3.0-only
use crate::types::{Boolean, Event, Guid, Handle, IgnixError, Status, Uuid};
use core::{ffi::c_void, ptr::NonNull};

#[repr(C)]
#[allow(unused)]
pub struct SimpleTextOutputProtocolFFI {
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
    pub mode: *mut SimpleTextOutputMode,
}

// This is the wrapper that allows to use SimpleTextOutputProtocol withouth unsafe lines
pub struct SimpleTextOutputProtocol {
    protocol: NonNull<SimpleTextOutputProtocolFFI>,
}

impl SimpleTextOutputProtocol {
    /// # Safety
    /// The protocol should point to a valid instance assigned by the UEFI, and not null
    pub unsafe fn new(protocol: *mut SimpleTextOutputProtocolFFI) -> Self {
        let not_null =
            NonNull::new(protocol).expect("SimpleTextOutputProtocol pointer cannot be null");
        Self { protocol: not_null }
    }

    fn get_protocol(&self) -> &SimpleTextOutputProtocolFFI {
        unsafe { self.protocol.as_ref() }
    }

    pub fn reset(&mut self) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().reset)(self.protocol.as_ptr(), true) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.reset"))?
        }
        Ok(())
    }

    pub fn output_string(&mut self, string: &[u16]) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().output_string)(self.protocol.as_ptr(), string.as_ptr()) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.output_string"))?
        }
        Ok(())
    }

    pub fn test_string(&mut self, string: &[u16]) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().test_string)(self.protocol.as_ptr(), string.as_ptr()) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.test_string"))?
        }
        Ok(())
    }

    pub fn query_mode(
        &mut self,
        mode: usize,
        columns: &mut usize,
        rows: &mut usize,
    ) -> Result<(), IgnixError> {
        let status = unsafe {
            (self.get_protocol().query_mode)(self.protocol.as_ptr(), mode, columns, rows)
        };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.query_mode"))?
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: usize) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().set_mode)(self.protocol.as_ptr(), mode) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_mode"))?
        }
        Ok(())
    }

    pub fn set_attribute(&mut self, attribute: usize) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().set_attribute)(self.protocol.as_ptr(), attribute) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_attribute"))?
        }
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().clear_screen)(self.protocol.as_ptr()) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.clear_screen"))?
        }
        Ok(())
    }
    pub fn set_cursor_position(&mut self, column: usize, row: usize) -> Result<(), IgnixError> {
        let status = unsafe {
            ((self.get_protocol().set_cursor_position)(self.protocol.as_ptr(), column, row))
        };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_cursor_position"))?
        }
        Ok(())
    }

    pub fn enable_cursor(&mut self, visible: bool) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().enable_cursor)(self.protocol.as_ptr(), visible) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.enable_cursor"))?
        }
        Ok(())
    }
}

impl Uuid for SimpleTextOutputProtocol {
    const GUID: Guid = Guid::new(
        0x387477c2,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode: u32,
    pub mode: u32,
    pub attribute: u32,
    pub cursor_col: u32,
    pub cursor_row: u32,
    pub cursor_visible: Boolean,
}
