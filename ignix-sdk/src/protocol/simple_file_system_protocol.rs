// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    protocol::file_protocol::{FileProtocol, FileProtocolFFI},
    types::{Guid, IgnixError, Status, Uuid},
};
use core::{
    ffi::c_void,
    ptr::{NonNull, null_mut},
};
#[repr(C)]
pub struct SimpleFileSystemProtocolFFI {
    revision: u64,
    open_volume:
        unsafe extern "efiapi" fn(this: *mut Self, file_protocol: *mut *mut FileProtocolFFI) -> Status,
}

pub struct SimpleFileSystemProtocol {
    protocol: NonNull<SimpleFileSystemProtocolFFI>,
}

impl SimpleFileSystemProtocol {
    /// # Safety
    /// This function is unsafe because the protocol itself is a pointer.
    /// But it's secure to use since the interface that you will be using has an idiomatic Option
    /// (get_protocol) and deref the pointer checking if it is null or not.
    pub unsafe fn new(protocol: *mut SimpleFileSystemProtocolFFI) -> Self {
        let non_null =
            NonNull::new(protocol).expect("SimpleFileSystemProtocol pointer cannot be null");
        Self { protocol: non_null }
    }
    fn get_protocol(&self) -> &SimpleFileSystemProtocolFFI {
        unsafe { self.protocol.as_ref() }
    }
    pub fn open_volume(&mut self) -> Result<FileProtocol, IgnixError> {
        let mut root: *mut FileProtocolFFI = null_mut();
        let status =
            unsafe { (self.get_protocol().open_volume)(self.protocol.as_ptr(), &mut root) };
        if status.is_error() {
            Err(status.context("open_volume"))?
        }
        // Safety
        // if everything was okay, creates an instance of secure FileProtocolWrapper
        // It's mandatory to check if it gave any errors before this.
        Ok(unsafe { FileProtocol::new(root) })
    }
}

impl Uuid for SimpleFileSystemProtocol {
    const GUID: Guid = Guid::new(
        0x0964e5b22,
        0x6459,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
