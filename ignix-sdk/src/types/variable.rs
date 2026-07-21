use crate::types::Guid;
use core::ffi::c_void;

// SPDX-License-Identifier: GPL-3.0-only
pub struct Variable<'a, const N: usize> {
    pub variable_name: &'a [u16],
    pub vendor_guid: *const Guid,
    pub attr: VariableAttributes,
    pub data_size: usize,
    pub data: [u8; N],
}
impl<'a, const N: usize> Variable<'a, N> {
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.data_size]
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VariableAttributes(pub u32);
impl VariableAttributes {
    pub const NON_VOLATILE: Self = Self(0x00000001);
    pub const BOOTSERVICE_ACCESS: Self = Self(0x00000002);
    pub const RUNTIME_ACCESS: Self = Self(0x00000004);
    pub const HARDWARE_ERROR_RECORD: Self = Self(0x00000008);
    /*There are some more that I'm not going to include, because they're secure boot related
     * but since I'm doing my own version of secure boot this doesn't matter at all*/
}
#[repr(C)]
pub struct NextVariableName<const N: usize> {
    pub variable_name_size: usize,
    pub variable_name: [u16; N],
    pub vendor_guid: *const Guid,
}

pub struct NonVolatileRamStatus {
    pub attr: VariableAttributes,
    pub maximum_variable_storage_size: u64,
    pub remaining_variable_storage_size: u64,
    pub maximum_variable_size: u64,
}
