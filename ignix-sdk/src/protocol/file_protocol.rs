// SPDX-License-Identifier: GPL-3.0-only
use core::ffi::c_void;
pub struct FileProtocol {
    revision: u64,
    pub open: *mut c_void,
    pub close: *mut c_void,
    pub delete: *mut c_void,
    pub read: *mut c_void,
    pub write: *mut c_void,
    pub get_position: *mut c_void,
    pub set_position: *mut c_void,
    pub get_info: *mut c_void,
    pub set_info: *mut c_void,
    // Those extended protocols were added in revision 0x00020000
    pub open_ex: *mut c_void,
    pub read_ex: *mut c_void,
    pub write_ex: *mut c_void,
    pub file_ex: *mut c_void,
}

pub struct FileProtocolWrapper {
    protocol: *mut FileProtocol,
}
impl FileProtocolWrapper {
    pub unsafe fn new(protocol: *mut FileProtocol) -> Self {
        Self { protocol }
    }
    fn get_protocol(&self) -> Option<&FileProtocol> {
        if self.protocol.is_null() {
            return None;
        }
        unsafe { Some(&*self.protocol) }
    }
}
