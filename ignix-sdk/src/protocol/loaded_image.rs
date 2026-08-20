// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    protocol::DevicePathProtocol, table::SystemTable, types::{Guid, Handle, MemoryType, Uuid}
};
use core::{ffi::c_void, ptr::NonNull};
/* You can find this struct defined in page 255, UEFI spec revision 2.11 PDF*/
#[allow(unused)]
#[repr(C)]
/// Can be used on any image handle to obtain information about the loaded handle
pub struct LoadedImageProtocol {
    revision: u32,
    pub parent_handle: Handle,
    pub system_table: *mut SystemTable,
    // Source location of the image
    pub device_handle: Handle,
    pub file_path: *mut DevicePathProtocol,
    pub reserved: *mut c_void,

    // Image's load options
    pub load_options_size: u32,
    pub load_options: *mut c_void,

    // Location (in RAM) where the image was loaded.
    pub image_base: *mut c_void,
    pub image_size: u64,
    pub image_code_type: MemoryType,
    pub image_data_type: MemoryType,
    unload: *mut c_void, // Don't use since there is already a RAII pattern for the LoadImage function
}

impl LoadedImageProtocol {
    pub fn device_handle(&self) -> Handle {
        self.device_handle
    }
    pub fn set_load_options(&mut self, cmdline: &[u16]) {
        self.load_options = cmdline.as_ptr() as *mut c_void;
        self.load_options_size = core::mem::size_of_val(cmdline) as u32;
    }
}

impl Uuid for LoadedImageProtocol {
    const GUID: Guid = Guid::new(
        0x5B1B31A1,
        0x9562,
        0x11d2,
        [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
    );
}
