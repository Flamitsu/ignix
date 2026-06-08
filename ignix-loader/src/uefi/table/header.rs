// SPDX-License-Identifier: GPL-3.0-only
#[repr(C)]
pub struct Header {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}
