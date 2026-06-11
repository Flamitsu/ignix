// SPDX-License-Identifier: GPL-3.0-only
use core::ptr::NonNull;
use crate::uefi::{
    table::boot::BootServicesWrapper,
    types::{AllocateType, MemoryType, PhysicalAddress, Status},
};
impl BootServicesWrapper {
    pub fn allocate_pages(
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
    ) -> Result<NonNull<u8>, Status>  {
        Err(Status::NOT_FOUND)
    }
    pub fn free_pages() -> Status {
        Status::NOT_FOUND
    }
    pub fn get_memory_map() -> Status {
        Status::NOT_FOUND
    }
    pub fn allocate_pool() -> Status {
        Status::NOT_FOUND
    }
    pub fn free_pool() -> Status {
        Status::NOT_FOUND
    }
}
