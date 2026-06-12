// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::{
    table::boot::BootServicesWrapper,
    types::{
        AllocateType, MemoryDescriptor, MemoryMap, MemoryType, PAGE_SIZE, PhysicalAddress, Status,
    },
};
use core::ptr::NonNull;
impl BootServicesWrapper {
    pub fn allocate_pages(
        self,
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
    ) -> Result<NonNull<u8>, Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let mut addr: PhysicalAddress = 0;
        let status =
            unsafe { (function.allocate_pages)(allocate_type, memory_type, pages, &mut addr) };

        if status.is_success() {
            let ptr = addr as *mut u8;
            if let Some(not_null) = NonNull::new(ptr) {
                return Ok(not_null);
            }

            Err(Status::OUT_OF_RESOURCES)?
        }

        Err(status)?
    }

    pub fn free_pages(self, memory: PhysicalAddress, pages: usize) -> Result<(), Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let status = unsafe { (function.free_pages)(memory, pages) };

        if status.is_success() {
            return Ok(());
        }
        Err(status)?
    }

    pub fn get_memory_map(self) -> Result<MemoryMap, Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };

        let mut mem_map = MemoryMap::new_empty();

        {
            let first_execution = unsafe {
                (function.get_memory_map)(
                    &mut mem_map.map_size,
                    core::ptr::null_mut(),
                    &mut mem_map.key,
                    &mut mem_map.descriptor_size,
                    &mut mem_map.descriptor_version,
                )
            };

            if first_execution != Status::BUFFER_TOO_SMALL {
                Err(first_execution)?
            }
        }

        /* This is mandatory. Whenever you need to allocate more memory,
         * the map size since the last call will increase.
         * Sometimes some bullshit implementations do too much
         * fragmentation so 8 is a good margin for me I think */
        mem_map.map_size += (mem_map.descriptor_size * 8);
        // I know this is a war crime, but rust forced me to do '/ PAGE_SIZE' as that function.
        let pages_needed = (mem_map.map_size + PAGE_SIZE - 1).div_ceil(PAGE_SIZE);

        let buffer_ptr = self.allocate_pages(
            AllocateType::AllocateAnyPages,
            MemoryType::EfiLoaderData,
            pages_needed,
        )?;
        let status = unsafe {
            (function.get_memory_map)(
                &mut mem_map.map_size,
                buffer_ptr.as_ptr() as *mut MemoryDescriptor,
                &mut mem_map.key,
                &mut mem_map.descriptor_size,
                &mut mem_map.descriptor_version,
            )
        };

        if status.is_success() {
            // This converts the descriptor into NonNull<MemoryDescriptor>
            mem_map.descriptor = Some(buffer_ptr.cast());
            return Ok(mem_map);
        }
        self.free_pages(buffer_ptr.as_ptr() as PhysicalAddress, pages_needed)?;
        Err(status)
    }
    /// pool_type needs to be either OEM reserved use or UEFI OS Loaders.
    pub fn allocate_pool(self, pool_type: MemoryType, size: usize) -> Result<NonNull<u8>, Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };

        let mut raw_ptr: *mut u8 = core::ptr::null_mut();
        let status = unsafe { (function.allocate_pool)(pool_type, size, &mut raw_ptr) };

        if status.is_success() {
            let Some(nn) = NonNull::new(raw_ptr) else {
                Err(Status::OUT_OF_RESOURCES)?
            };
            return Ok(nn);
        }
        Err(status)
    }

    pub fn free_pool(self, buffer: NonNull<u8>) -> Result<(), Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let status = unsafe { (function.free_pool)(buffer.as_ptr()) };
        if status.is_success() {
            return Ok(());
        }
        Err(status)
    }
}
