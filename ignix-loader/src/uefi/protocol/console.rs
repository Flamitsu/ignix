// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::types::{Boolean, Guid, Status, Uuid};
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
pub(crate) struct SimpleTextOutputProtocol {
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
    pub unsafe fn new(protocol: *mut SimpleTextOutputProtocol) -> Self {
        Self { protocol }
    }

    fn get_protocol(&self) -> Option<&SimpleTextOutputProtocol> {
        if self.protocol.is_null() {
            return None;
        }
        unsafe { Some(&*self.protocol) }
    }

    pub fn reset(&mut self, extended: bool) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.reset)(self.protocol, extended) };
        }
        Status::NOT_FOUND
    }

    pub fn output_string(&mut self, string: *const u16) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.output_string)(self.protocol, string) };
        }
        Status::NOT_FOUND
    }

    pub fn test_string(&mut self, string: *const u16) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.test_string)(self.protocol, string) };
        }
        Status::INVALID_PARAMETER
    }

    pub fn query_mode(&mut self, mode: usize, columns: *mut usize, rows: *mut usize) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.query_mode)(self.protocol, mode, columns, rows) };
        }
        Status::NOT_FOUND
    }

    pub fn set_mode(&mut self, mode: usize) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.set_mode)(self.protocol, mode) };
        }
        Status::NOT_FOUND
    }

    pub fn set_attribute(&mut self, attribute: usize) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.set_attribute)(self.protocol, attribute) };
        }
        Status::NOT_FOUND
    }

    pub fn clear_screen(&mut self) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.clear_screen)(self.protocol) };
        }
        Status::NOT_FOUND
    }

    pub fn set_cursor_position(&mut self, column: usize, row: usize) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.set_cursor_position)(self.protocol, column, row) };
        }
        Status::NOT_FOUND
    }

    pub fn enable_cursor(&mut self, visible: bool) -> Status {
        if let Some(protocol) = self.get_protocol() {
            return unsafe { (protocol.enable_cursor)(self.protocol, visible) };
        }
        Status::NOT_FOUND
    }
}
