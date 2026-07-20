use core::{ffi::c_void, ptr::null_mut};

use crate::{
    init::SYSTEM_TABLE,
    table::runtime::RuntimeServicesWrapper,
    types::{DebugDisposition, IgnixError, MemoryDescriptor, MemoryMap, Status},
};
impl RuntimeServicesWrapper {
    /// Changes the runtime addressing mode of EFI firmware from physical to virtual.
    ///
    /// You can only execute this function ONCE. If you execute it more than one, it will return
    /// EFI_UNSUPPORTED.
    /// You should only call it when you already have call ExitBootServices(). This is exclusive to
    /// the handle that called the ExitBootServices function.
    /// You CAN'T call this function if the CPU is already using the VirtualMemory
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORTED EFI firmware is not at runtime, or the EFI firmware is already in virtual
    /// address mapped mode.
    /// EFI_INVALID_PARAMETER DescriptorSize or DescriptorVersion is invalid.
    /// EFI_NO_MAPPING A virtual address was not supplied for a range in the memory map that requires a mapping.
    /// EFI_NOT_FOUND A virtual address was supplied for an address that is not found in the memory map.
    /// EFI_UNSUPPORTED This call is not supported by this platform at the time the call is made.
    /// The platform should describe this runtime service as unsupported at runtime via an
    /// EFI_RT_PROPERTIES_TABLE configuration table.
    pub fn set_virtual_address_map(&self) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("set_virtual_address_map"))?
        };
        let memory_map = SYSTEM_TABLE
            .get()
            .ok_or(Status::ST_POINTER_MISSING.context("set_virtual_address_map"))?
            .get_boot_services()
            .ok_or(Status::BST_POINTER_MISSING.context("set_virtual_address_map"))?
            .get_memory_map()?;
        let memory_map_descriptor = match memory_map.descriptor {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr.as_ptr(),
        };
        let status = unsafe {
            (function.set_virtual_address_map)(
                memory_map.map_size,
                memory_map.descriptor_size,
                memory_map.descriptor_version,
                memory_map_descriptor,
            )
        };

        if status.is_error() {
            Err(status.context("set_virtual_address_map"))?
        }

        Ok(())
    }
    /// Determines the new virtual address that is to be used on subsequent memory access.
    ///
    /// This function should be ONLY USED WHILE the event EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE is
    /// executing.
    /// Need to use it with physical address while executing set_virtual_address_map().
    ///
    /// RETURN CODES:
    /// EFI_NOT_FOUND The pointer pointed to by Address was not found to be part of the current
    /// memory map. This is normally fatal.
    /// EFI_INVALID_PARAMETER Address is NULL.
    /// EFI_INVALID_PARAMETER *Address is NULL and DebugDisposition does not have the
    /// EFI_OPTIONAL_PTR bit set.
    /// EFI_UNSUPPORTED This call is not supported by this platform at the time the call is made.
    /// The platform should describe this runtime service as unsupported at runtime via an
    /// EFI_RT_PROPERTIES_TABLE configuration table.
    pub fn convert_pointer(
        &self,
        debug_disposition: DebugDisposition,
        address: &mut *mut c_void,
    ) -> Result<(), IgnixError> {
        assert!(
            debug_disposition != DebugDisposition::OPTIONAL_PTR && address.is_null(),
            "Address cannot be NULL unless OPTIONAL_PTR is set"
        );
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("convert_pointer"))?
        };
        let addr_ptr: *mut *mut c_void = address as *mut *mut c_void;
        let status = unsafe { (function.convert_pointer)(debug_disposition, addr_ptr) };
        Ok(())
    }
}
