// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    table::boot::BootServicesWrapper,
    types::{
        AllocateType, IgnixError, MemoryDescriptor, MemoryMap, MemoryType, PAGE_SIZE,
        PhysicalAddress, Status,
    },
};
use core::ptr::NonNull;
impl BootServicesWrapper {
    /// Allocate pages from the system.
    /// In general, OS loaders should allocate memory (and pool) of type EfiLoaderData.
    ///
    /// AllocateAnyPages allocates any range of pages that satisfies the request. The address on
    /// input is ignored.
    ///
    /// AllocateMaxAddress allocates a range of pages whose uppermost address is less
    /// than or equal to the address pointed to by Memory on input.
    ///
    /// AllocateAddress allocate pages at the address pointed to by Memory on input.
    ///
    /// RETURN CODES
    /// EFI_OUT_OF_RESOURCEST The pages could not be allocated.
    /// EFI_INVALID_PARAMETER Type is not AllocateAnyPages or AllocateMaxAddress or AllocateAddress
    /// EFI_INVALID_PARAMETER MemoryType is in the range EfiMaxMemoryType..0x6FFFFFFF.
    /// EFI_INVALID_PARAMETER MemoryType is EfiPersistentMemoryType or EfiUnacceptedMemory.
    /// EFI_INVALID_PARAMETER Memory is NULL.
    /// EFI_NOT_FOUND The requested pages could not be found
    pub fn allocate_pages(
        self,
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
    ) -> Result<NonNull<u8>, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("allocate_pages"))?
        };
        let mut addr: PhysicalAddress = 0;
        let status =
            unsafe { (function.allocate_pages)(allocate_type, memory_type, pages, &mut addr) };

        if status.is_success() {
            let ptr = addr as *mut u8;
            if let Some(not_null) = NonNull::new(ptr) {
                return Ok(not_null);
            }

            Err(Status::OUT_OF_RESOURCES.context("allocate_pages"))?
        }

        Err(status.context("allocate_pages"))?
    }
    /// Returns the memory allocated by AllocatePages to the firmware.
    /// RETURN CODES:
    ///
    /// EFI_NOT_FOUND The requested memory pages were not allocated with AllocatePages().
    /// EFI_INVALID_PARAMETER Memory is not a page-aligned address or Pages is invalid.
    pub fn free_pages(self, memory: PhysicalAddress, pages: usize) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("free_pages"))?
        };
        let status = unsafe { (function.free_pages)(memory, pages) };

        if status.is_success() {
            return Ok(());
        }
        Err(status.context("free_pages"))?
    }
    /// Returns a copy of the current memory map.
    ///
    /// RETURN CODES:
    /// EFI_BUFFER_TOO_SMALL The MemoryMap buffer was too small. The current buffer size needed to
    /// hold the memory map is returned in MemoryMapSize.
    /// EFI_INVALID_PARAMETER MemoryMapSize is NULL.
    /// EFI_INVALID_PARAMETER The MemoryMap buffer is not too small and MemoryMap is NULL.
    pub fn get_memory_map(self) -> Result<MemoryMap, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("get_memory_map"))?
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
                Err(first_execution.context("get_memory_map"))?
            }
        }

        /* This is mandatory. Whenever you need to allocate more memory,
         * the map size since the last call will increase.
         * Sometimes some bullshit implementations do too much
         * fragmentation so 8 is a good margin for me I think */
        mem_map.map_size += (mem_map.descriptor_size * 8);
        /* I know this is a war crime, but rust forced me to do this fix as that function.
         * However since this is a cleaner method, I prefer to keep it.
         * old: let pages_needed = (mem_map.map_size + PAGE_SIZE - 1) / PAGE_SIZE*/
        let pages_needed = mem_map.map_size.div_ceil(PAGE_SIZE);

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
        Err(status.context("get_memory_map"))
    }
    /// Allocates pool memory.
    /// Allocates a memory region of Size bytes from memory of type PoolType and returns the
    /// address of the allocated memory in the location referenced by Buffer. This function
    /// allocates pages from EfiConventionalMemory as needed to grow the requested pool type
    /// pool_type needs to be either OEM reserved use or UEFI OS Loaders.
    ///
    /// RETURN CODES
    /// EFI_OUT_OF_RESOURCES The pool requested could not be allocated.
    /// EFI_INVALID_PARAMETER PoolType is in the range EfiMaxMemoryType..0x6FFFFFFF.
    /// EFI_INVALID_PARAMETER PoolType is EfiPersistentMemory.
    /// EFI_INVALID_PARAMETER Buffer is NULL.
    pub fn allocate_pool(
        self,
        pool_type: MemoryType,
        size: usize,
    ) -> Result<NonNull<u8>, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("allocate_pool"))?
        };

        let mut raw_ptr: *mut u8 = core::ptr::null_mut();
        let status = unsafe { (function.allocate_pool)(pool_type, size, &mut raw_ptr) };

        if status.is_success() {
            let Some(nn) = NonNull::new(raw_ptr) else {
                Err(Status::OUT_OF_RESOURCES.context("allocate_pool"))?
            };
            return Ok(nn);
        }
        Err(status.context("allocate_pool"))
    }
    /// Returns pool memory to the firmware.
    /// The FreePool() function returns the memory specified by Buffer to the system. On return,
    /// the memory’s type is EfiConventionalMemory.
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Buffer was invalid.
    pub fn free_pool(self, buffer: NonNull<u8>) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("free_pool"))?
        };
        let status = unsafe { (function.free_pool)(buffer.as_ptr()) };
        if status.is_success() {
            return Ok(());
        }
        Err(status.context("free_pool"))
    }
}
