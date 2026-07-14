use core::ffi::c_void;
pub type Handle = *mut c_void;
pub type Char16 = u16;
pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;

#[repr(C)]
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
