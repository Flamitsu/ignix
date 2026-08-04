// SPDX-License-Identifier: GPL-3.0-only
use core::ffi::c_void;
use crate::types::{Boolean, Event, Guid, IgnixError, Status, Uuid};
#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode: u32,
    pub mode: u32,
    pub attribute: u32,
    pub cursor_col: u32,
    pub cursor_row: u32,
    pub cursor_visible: Boolean,
}

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
    pub mode: *mut SimpleTextOutputMode,
}

impl Uuid for SimpleTextOutputProtocol {
    const GUID: Guid = Guid::new(
        0x387477c2,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

// This is the wrapper that allows to use SimpleTextOutputProtocol withouth unsafe lines
pub struct SimpleTextOutput {
    protocol: *mut SimpleTextOutputProtocol,
}

impl SimpleTextOutput {
    /// # Safety
    /// The protocol pointer should always be valid and not null, point to an instance
    /// initialized by the UEFI to 'SimpleTextOutputProtocol'
    pub unsafe fn new(protocol: *mut SimpleTextOutputProtocol) -> Self {
        Self { protocol }
    }

    fn get_protocol(&self) -> Option<&SimpleTextOutputProtocol> {
        if self.protocol.is_null() {
            return None;
        }
        unsafe { Some(&*self.protocol) }
    }

    pub fn reset(&mut self, extended: bool) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND.context("SimpleTextOutputProtocol.reset"))?
        };
        let status = unsafe { (protocol.reset)(self.protocol, extended) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.reset"))?
        }
        Ok(())
    }

    pub fn output_string(&mut self, string: &[u16]) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND
                .context("SimpleTextOutputProtocol.output_string"))?
        };
        let status = unsafe { (protocol.output_string)(self.protocol, string.as_ptr()) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.output_string"))?
        }
        Ok(())
    }

    pub fn test_string(&mut self, string: &[u16]) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND.context(""))?
        };
        let status = unsafe { (protocol.test_string)(self.protocol, string.as_ptr()) };
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
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND.context("SimpleTextOutputProtocol.query_mode"))?
        };
        let status = unsafe { (protocol.query_mode)(self.protocol, mode, columns, rows) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.query_mode"))?
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: usize) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND.context("SimpleTextOutputProtocol.set_mode"))?
        };
        let status = unsafe { (protocol.set_mode)(self.protocol, mode) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_mode"))?
        }
        Ok(())
    }

    pub fn set_attribute(&mut self, attribute: usize) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND
                .context("SimpleTextOutputProtocol.set_attribute"))?
        };
        let status = unsafe { (protocol.set_attribute)(self.protocol, attribute) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_attribute"))?
        }
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND.context("SimpleTextOutputProtocol.clear_screen"))?
        };
        let status = unsafe { (protocol.clear_screen)(self.protocol) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.clear_screen"))?
        }
        Ok(())
    }
    pub fn set_cursor_position(&mut self, column: usize, row: usize) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND
                .context("SimpleTextOutputProtocol.set_cursor_position"))?
        };
        let status = unsafe { (protocol.set_cursor_position)(self.protocol, column, row) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_cursor_position"))?
        }
        Ok(())
    }

    pub fn enable_cursor(&mut self, visible: bool) -> Result<(), IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND
                .context("SimpleTextOutputProtocol.enable_cursor"))?
        };
        let status = unsafe { (protocol.enable_cursor)(self.protocol, visible) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.enable_cursor"))?
        }
        Ok(())
    }
}
#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub reset: *mut c_void,
    pub read_key_stroke: *mut c_void,
    wait_for_key: Event,
}

impl Uuid for SimpleTextInputProtocol {
    const GUID: Guid = Guid::new(
        0x387477c1,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

pub struct SimpleTextInputProtocolWrapper {

}
