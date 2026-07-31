// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    protocol::file_protocol::{FileProtocol, FileProtocolWrapper},
    types::{Guid, IgnixError, Status, Uuid},
};
use core::{ffi::c_void, ptr::null_mut};
#[repr(C)]
pub struct SimpleFileSystemProtocol {
    revision: u64,
    open_volume:
        unsafe extern "efiapi" fn(this: *mut Self, file_protocol: *mut *mut FileProtocol) -> Status,
}
impl Uuid for SimpleFileSystemProtocol {
    const GUID: Guid = Guid::new(
        0x0964e5b22,
        0x6459,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
pub struct SimpleFileSystemProtocolWrapper {
    protocol: *mut SimpleFileSystemProtocol,
}
impl SimpleFileSystemProtocolWrapper {
    /// # Safety
    /// This function is unsafe because the protocol itself is a pointer.
    /// But it's secure to use since the interface that you will be using has an idiomatic Option
    /// (get_protocol) and deref the pointer checking if it is null or not.
    pub unsafe fn new(protocol: *mut SimpleFileSystemProtocol) -> Self {
        Self { protocol }
    }
    fn get_protocol(&self) -> Option<&SimpleFileSystemProtocol> {
        if self.protocol.is_null() {
            return None;
        }
        unsafe { Some(&*self.protocol) }
    }
    pub fn open_volume(&mut self) -> Result<FileProtocolWrapper, IgnixError> {
        let Some(protocol) = self.get_protocol() else {
            Err(Status::PROTOCOL_POINTER_NOT_FOUND.context("open_volume"))?
        };
        let mut root: *mut FileProtocol = null_mut();
        let status = unsafe { (protocol.open_volume)(self.protocol, &mut root) };
        if status.is_error() {
            Err(status.context("open_volume"))?
        }
        // Safety
        // if everything was okay, creates an instance of secure FileProtocolWrapper
        // It's mandatory to check if it gave any errors before this.
        Ok(unsafe { FileProtocolWrapper::new(root) })
    }
}
