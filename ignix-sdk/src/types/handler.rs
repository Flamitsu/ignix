// SPDX-License-Identifier: GPL-3.0-only
use core::ffi::c_void;
use crate::{init::SYSTEM_TABLE, types::{Guid, IgnixImage}};
pub type Handle = *mut c_void;
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

#[repr(C)]
pub struct IgnixProtocol<'a> {
    pub image: &'a mut IgnixImage<'a>,
    pub guid: Guid,
    pub interface: *mut c_void,
}

impl<'a> Drop for IgnixProtocol<'a> {
    fn drop(&mut self) {
        if let Some(image_handle) = self.image.handle {
            let _ = SYSTEM_TABLE
                .get()
                .unwrap()
                .get_boot_services()
                .unwrap()
                .uninstall_protocol_interface(
                    image_handle,
                    &self.guid,
                    self.interface as *const c_void,
                );
        }
    }
}

