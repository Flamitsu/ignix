// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    init::SYSTEM_TABLE,
    types::{PhysicalAddress, VirtualAddress},
};
use core::{
    ffi::c_void,
    marker::PhantomData,
    ops::RangeInclusive,
    ptr::NonNull,
    slice::{from_raw_parts, from_raw_parts_mut},
};

/* Those numbers are how enums are interpreted in C.
 * Basically a fancy way to represent an int, but since
 * I don't want rustc do anything funny with it, I prefer explicitly
 * saying which value is associated*/
#[allow(unused)]
#[repr(u32)]
pub enum MemoryType {
    /// Not usable memory
    EfiReservedMemory = 0,

    /// Chunk used for a loaded UEFI application
    EfiLoaderCode = 1,

    /// Chunk used for loaded UEFI application and the default
    /// data allocation type used by the UEFI Boot Services Driver.
    EfiLoaderData = 2,

    /// Chunk of code used by a loaded UEFI boot service driver.
    EfiBootServicesCode = 3,

    /// Chunk used for loaded UEFI boot service driver and the default
    /// data allocation type used by the UEFI boot service drivers to allocate memory
    EfiBootServicesData = 4,

    /// Chunk portion of a loaded UEFI runtime driver
    EfiRuntimeServicesCode = 5,

    /// Chunk used for loaded UEFI runtime driver and the default
    /// data allocation type used by the UEFI runtime driver to allocate memory
    EfiRuntimeServicesData = 6,

    /// Free memory (unallocated)
    EfiConventionalMemory = 7,

    /// Memory in which errors have been detected.
    EfiUnusableMemory = 8,

    /// Memory that holds the ACPI tables
    EfiACPIReclaimMemory = 9,

    /// Address the space reserved use by the firmware
    EfiACPIMemoryNVS = 10,

    /// Used by the system firmware to request MMIO region be mapped by the OS to a virtual address
    /// so it can be used by the EFI runtime services.
    EfiMemoryMappedIO = 11,

    /// System MMIO region that is used to translate memory cycles to IO cycles by the CPU
    EfiMemoryMappedIOPortSpace = 12,

    /// Address space reserved by the firmware for code that is part of the CPU
    EfiPalCode = 13,

    /// Memory region that operates as EfiConventionalMemory but it also supports
    /// byte-addreseable non-volatility
    EfiPersistentMemory = 14,

    /// Memory region that is unaccepted memory, must be accepted by the boot target
    /// before it can be used. Unless otherwise noted, all other EFI memory types are accepted.
    /// For platforms that support unaccepted memory, all unacepted memory will be reported as
    /// unacepted memory map. Unreported physical address range must be treated as not-present
    /// memory.
    EfiUnacceptedMemory = 15,

    EfiMaxMemory = 16,
}

impl MemoryType {
    /// NOT USABLE MEMORY FOR THE BOOT LOADER
    pub const RESERVED_OEM_MEMORY: RangeInclusive<u32> = 0x70000000..=0x7FFFFFFF;
    /// NOT USABLE MEMORY FOR THE BOOT LOADER
    pub const RESERVED_OS_LOADER_MEMORY: RangeInclusive<u32> = 0x80000000..=0xFFFFFFFF;
}

#[repr(u32)]
pub enum AllocateType {
    /// Allocates any pages that satisfies the request.
    /// On input, the address pointed to by Memory is ignored
    AllocateAnyPages = 0,

    /// allocate any aviable range of pages whose uppermost address
    /// is less than or equal to the address pointed to and by Memory on input
    AllocateMaxAddress = 1,

    /// Allocate pages at the address pointed by Memory on input
    AllocateAddress = 2,

    MaxAllocate = 3,
}

/// Size in bytes of UEFI Memory page (NOT CPU PAGES)
pub const PAGE_SIZE: usize = 4096;

#[repr(C)]
pub struct MemoryDescriptor {
    pub tp: MemoryType,
    pad: u32,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub page_count: u64,
    pub attributes: MemoryAttributes,
}
// All the info for this codes and descriptions can be found in
// the UEFI Spec 2.11 page 158 to 159
#[repr(C)]
pub struct MemoryAttributes(pub u64);
impl MemoryAttributes {
    /// The memory region supports being configured as not cacheable
    pub const UNCACHABLE: Self = Self(1 << 0);

    /// The memory region supports being configured as write combined
    pub const WRITE_COMBINED: Self = Self(1 << 1);

    /// The memory region supports being configured as cacheable
    /// with a "write through" policy. Writes that hit in the cache will
    /// also be written to main memory.
    pub const WRITE_THROUGH: Self = Self(1 << 2);

    /// The memory region supports being configured as cacheable
    /// with a "write back" policy. Reads and writes that hit in the
    /// cache do not propagate to main memory. Dirty data is written
    /// back into main memory when a new cache line is allocated.
    pub const WRITE_BACK: Self = Self(1 << 3);

    /// The memory region supports being configured as not cacheable,
    /// exported and supports the "fetch and add" semaphore mechanism
    pub const UNCACHABLE_EXPORTED: Self = Self(1 << 4);

    /// Physical memory protection: the memory region supports being configured
    /// as write-protected, and supports the "fetch and add" semaphore mechanism
    pub const WRITE_PROTECTED: Self = Self(1 << 12);

    /// Physical memory protection: The memory region supports being configured as
    /// read protected by the system hardware
    pub const READ_PROTECTED: Self = Self(1 << 13);

    /// Physical memory protection: The memory region supports being configured so
    /// it is protected by the system from executing code
    pub const EXECUTION_PROTECTED: Self = Self(1 << 14);

    /// Runtime memory attribute: The memory region refers to persistent memory
    pub const NO_VOLATILE: Self = Self(1 << 15);

    /// Memory region provides higher reliability relative to other memory. If
    /// all memory has the same reliability, then this bit is not used.
    pub const MORE_RELIABLE: Self = Self(1 << 16);

    /// Physical memory protection: The memory region supports making
    /// this memory range read-only by system hardware
    pub const READ_ONLY: Self = Self(1 << 17);

    /// Specific purpose memory (SPM). The memory is emarked for specific purpouses such as
    /// for specific device drivers or applications. The SPM attribute serves
    /// as a hint to the OS to avoid allocating this memory for code OS data or
    /// code that can't be relocated. Prolonged use of this memory for purposes other
    /// than the intended purpose may result in suboptimal platform performance
    pub const SPECIFIC_PURPOSE: Self = Self(1 << 18);

    /// If this flag is set, the memory region is capable of being protected with the
    /// CPU’s memory cryptographic capabilities. If this flag is clear,
    /// the memory region is not capable of being protected with the CPU’s memory
    /// cryptographic capabilities or the CPU does not support
    /// CPU memory cryptographic capabilities.
    pub const CPU_CRYPTO: Self = Self(1 << 19);

    /// If this flag is set, the memory region is present and capable of having memory
    /// dynamically removed from the platform. This attribute serves as a hint
    /// to the OS prior to its ACPI subsystem initialization to avoid allocating this
    /// memory for core OS data or code that cannot be dynamically relocated at runtime.
    /// If this flag is clear, the memory region is not capable of being dynamically
    /// removed from the platform at runtime.
    pub const HOT_PLUGGABLE: Self = Self(1 << 20);

    /// Runtime memory attribute: The memory region needs to be given a virtual
    /// mapping by the operating system when SetVirtualAddressMap()
    pub const RUNTIME: Self = Self(1 << 63);

    /// If this flag is set, the memory region is described with additional
    /// ISA-specific memory attributes as specified in EFI_MEMORY_ISA_MASK.
    pub const ISA_VALID: Self = Self(1 << 62);

    /// Defines the bits reserved for describing optional ISA-specific cacheability
    /// attributes that are not covered by the standard UEFI Memory Attributes
    /// cacheability bits (EFI_MEMORY_UC, EFI_MEMORY_WC, EFI_MEMORY_WT, EFI_MEMORY_WB
    /// and EFI_MEMORY_UCE).
    pub const ISA_MASK: Self = Self(0x0FFFF00000000000);
}

pub struct MemoryMap {
    pub map_size: usize,
    pub descriptor: Option<NonNull<MemoryDescriptor>>,
    pub key: usize,
    pub descriptor_size: usize,
    pub descriptor_version: u32,
}

impl MemoryMap {
    pub fn new_empty() -> Self {
        Self {
            map_size: 0,
            descriptor: None,
            key: 0,
            descriptor_size: 0,
            descriptor_version: 0,
        }
    }
}
#[repr(C)]
#[derive(PartialEq)]
pub struct DebugDisposition(pub usize);
impl DebugDisposition {
    pub const OPTIONAL_PTR: Self = Self(0x00000001);
}

pub struct PagesBuffer<'a> {
    pub ptr: NonNull<u8>,
    pub num_pages: usize,
    pub _m: PhantomData<&'a c_void>,
}
impl<'a> PagesBuffer<'a> {
    pub fn as_mut_slice(&mut self, len: usize) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr.as_ptr(), len) }
    }
    pub fn as_slice(&mut self, len: usize) -> &[u8] {
        unsafe { from_raw_parts(self.ptr.as_ptr(), len) }
    }
}
impl<'a> Drop for PagesBuffer<'a> {
    fn drop(&mut self) {
        let _ = SYSTEM_TABLE
            .get()
            .unwrap()
            .get_boot_services()
            .unwrap()
            .free_pages(self.ptr.as_ptr() as u64, self.num_pages);
    }
}

pub struct PoolBuffer<'a> {
    pub ptr: NonNull<u8>,
    pub num_bytes: usize,
    pub _m: PhantomData<&'a c_void>,
}

impl<'a> PoolBuffer<'a> {
    pub fn as_mut_slice(&mut self, len: usize) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr.as_ptr(), len) }
    }
    pub fn as_slice(&mut self, len: usize) -> &[u8] {
        unsafe { from_raw_parts(self.ptr.as_ptr(), len) }
    }
}

impl<'a> Drop for PoolBuffer<'a> {
    fn drop(&mut self) {
        let _ = SYSTEM_TABLE
            .get()
            .unwrap()
            .get_boot_services()
            .unwrap()
            .free_pool(self.ptr);
    }
}
