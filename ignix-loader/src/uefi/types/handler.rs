use core::ffi::c_void;

pub type Handle = *mut c_void;
// SPDX-License-Identifier: GPL-3.0-only
/* Page 167 UEFI spec 2.11, it's not my fault, it's an enum just with one value.*/
#[repr(C)]
pub enum InterfaceType {
    Native = 0,
}

#[repr(C)]
pub enum SearchType {
    AllHandles = 0,
    ByRegisterNotify = 1,
    ByProtocol = 2,
}

#[repr(transparent)]
pub struct OpenProtocol(pub u32);
impl OpenProtocol {
    pub const BY_HANDLE_PROTOCOL: u32 = 0x00000001;
    pub const GET_PROTOCOL: u32 = 0x00000002;
    pub const TEST_PROTOCOL: u32 = 0x00000004;
    pub const BY_CHILD_CONTROLER: u32 = 0x00000008;
    pub const BY_DRIVER: u32 = 0x00000010;
    pub const EXCLUSIVE: u32 = 0x00000020;
}

#[repr(C)]
pub struct OpenProtocolInformationEntry {
    pub agent_handle: Handle,
    pub controller_handle: Handle,
    pub attributes: u32,
    pub open_count: u32,
}
