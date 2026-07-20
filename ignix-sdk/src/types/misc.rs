// SPDX-License-Identifier: GPL-3.0-only
use core::{ffi::c_void, marker::PhantomData};

use crate::{
    init::SYSTEM_TABLE,
    types::{Boolean, Handle},
};
pub type Char16 = u16;
pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;

/*
 * Okay so I know this is a protocol in theory, but it's going to remain here until i actually
 * reach this part reading the UEFI spec and can complete more to deserve it's own file.
 * */
#[repr(C)]
pub struct DevicePathProtocol {
    pub dp_type: u8,
    pub sub_type: u8,
    pub length: [u8; 2],
}
// This is a marker trait, to make possible use some boot services functions that depends on it.
pub trait Table {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

pub trait Uuid {
    const GUID: Guid;
}
#[repr(C)]
pub struct IgnixImage<'a> {
    pub handle: Option<Handle>,
    pub _m: PhantomData<&'a c_void>,
}

impl<'a> Drop for IgnixImage<'a> {
    fn drop(&mut self) {
        if let Some(image_handle) = self.handle {
            let _ = SYSTEM_TABLE
                .get()
                .unwrap()
                .get_boot_services()
                .unwrap()
                .unload_image(image_handle);
        }
    }
}
