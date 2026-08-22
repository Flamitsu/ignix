use crate::{
    init::INITRD_FILES,
    protocol::{DevicePathProtocol, device_path::VendorDevicePathNode},
    services::boot::memory::allocate_pool,
    types::{AllocateType, DevicePath, Guid, IgnixError, MemoryType, PoolBuffer, Status, Uuid},
};
use core::{
    ffi::c_void,
    ptr::{NonNull, copy_nonoverlapping, null_mut},
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};
/* I know I'm repeating here, but I haven't found any way to compact it into one main helper
 * function.
 * Its just 2 functions any way*/
#[repr(C)]
pub struct LoadFileFFI {
    pub load_file: unsafe extern "efiapi" fn(
        this: *mut Self,
        file_path: *const DevicePathProtocol,
        boot_policy: bool,
        buff_size: *mut usize,
        buff: *mut c_void,
    ) -> Status,
}
/// This protocol is used to retrieve boot files from arbitrary devices (Like PXE boot or boot from
/// USB to the main disk)
pub struct LoadFile {
    pub protocol: NonNull<LoadFileFFI>,
}

impl LoadFile {
    #[inline(always)]
    fn get_protocol(&self) -> &LoadFileFFI {
        unsafe { self.protocol.as_ref() }
    }

    /// Causes the driver to load a specified file
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORTED The device does not support the provided BootPolicy.
    /// EFI_INVALID_PARAMETER FilePath is not a valid device path, or BufferSize is NULL.
    /// EFI_NO_MEDIA No medium was present to load the file.
    /// EFI_DEVICE_ERROR The file was not loaded due to a device error.
    /// EFI_NO_RESPONSE The remote system did not respond.
    /// EFI_NOT_FOUND The file was not found.
    /// EFI_ABORTED The file load process was manually cancelled.
    /// EFI_BUFFER_TOO_SMALL The BufferSize is too small to read the current directory entry. BufferSize
    /// has been updated with the size needed to complete the request.
    /// EFI_WARN_FILE_SYSTEM The resulting Buffer contains UEFI-compliant file system.
    pub fn load_file(
        &mut self,
        file_path: &DevicePathProtocol,
        boot_policy: bool,
    ) -> Result<PoolBuffer, IgnixError> {
        let mut buff_size = 0;
        let mut buff: *mut c_void = null_mut();
        let status = unsafe {
            (self.get_protocol().load_file)(
                self.protocol.as_ptr(),
                file_path,
                boot_policy,
                &mut buff_size,
                buff,
            )
        };

        if status.is_error() && status != Status::BUFFER_TOO_SMALL {
            Err(status.context("LoadFileProtocol.load_file"))?
        }

        let pool_buffer = allocate_pool(MemoryType::EfiLoaderData, buff_size)?;
        buff = pool_buffer.ptr.as_ptr() as *mut c_void;

        let status = unsafe {
            (self.get_protocol().load_file)(
                self.protocol.as_ptr(),
                file_path,
                boot_policy,
                &mut buff_size,
                buff,
            )
        };

        if status.is_error() {
            Err(status.context("LoadFileProtocol.load_file"))?
        }

        Ok(pool_buffer)
    }
}

impl Uuid for LoadFile {
    const GUID: Guid = Guid::new(
        0x56EC3091,
        0x954C,
        0x11d2,
        [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

pub struct LoadFile2FFI {
    pub load_file: unsafe extern "efiapi" fn(
        this: *mut Self,
        file_path: *const DevicePathProtocol,
        boot_policy: bool,
        buff_size: *mut usize,
        buff: *mut c_void,
    ) -> Status,
}

/// Used to obtain files from arbitrary devices but are not used as boot options
pub struct LoadFile2 {
    pub protocol: NonNull<LoadFile2FFI>,
}

impl LoadFile2 {
    #[inline(always)]
    fn get_protocol(&self) -> &LoadFile2FFI {
        unsafe { self.protocol.as_ref() }
    }

    /// Causes the driver to load a specified file
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORTED BootPolicy is TRUE.
    /// EFI_INVALID_PARAMETER FilePath is not a valid device path, or BufferSize is NULL.
    /// EFI_NO_MEDIA No medium was present to load the file.
    /// EFI_DEVICE_ERROR The file was not loaded due to a device error.
    /// EFI_NO_RESPONSE The remote system did not respond.
    /// EFI_NOT_FOUND The file was not found.
    /// EFI_ABORTED The file load process was manually cancelled.
    /// EFI_BUFFER_TOO_SMALL The BufferSize is too small to read the current directory entry. BufferSize
    /// has been updated with the size needed to complete the request.
    pub fn load_file(&mut self, file_path: &DevicePathProtocol) -> Result<PoolBuffer, IgnixError> {
        let mut buff_size = 0;
        let mut buff: *mut c_void = null_mut();
        let status = unsafe {
            (self.get_protocol().load_file)(
                self.protocol.as_ptr(),
                file_path,
                false,
                &mut buff_size,
                buff,
            )
        };

        if status.is_error() && status != Status::BUFFER_TOO_SMALL {
            Err(status.context("LoadFile2Protocol.load_file"))?
        }

        let pool_buffer = allocate_pool(MemoryType::EfiLoaderData, buff_size)?;
        buff = pool_buffer.ptr.as_ptr() as *mut c_void;

        let status = unsafe {
            (self.get_protocol().load_file)(
                self.protocol.as_ptr(),
                file_path,
                false,
                &mut buff_size,
                buff,
            )
        };

        if status.is_error() {
            Err(status.context("LoadFile2Protocol.load_file"))?
        }

        Ok(pool_buffer)
    }
}

impl Uuid for LoadFile2 {
    const GUID: Guid = Guid::new(
        0x4006c0c1,
        0xfcb3,
        0x403e,
        [0x99, 0x6d, 0x4a, 0x6c, 0x87, 0x24, 0xe0, 0x6d],
    );
}

// This part is exclusive for the Linux kernel to use.
// You can find this GUID in https://github.com/torvalds/linux/blob/master/include/linux/efi.h line 420
pub const LINUX_EFI_INITRD_MEDIA_GUID: Guid = Guid::new(
    0x5568e427,
    0x68fc,
    0x4f3d,
    [0xac, 0x74, 0xca, 0x55, 0x52, 0x31, 0xcc, 0x68],
);

pub extern "efiapi" fn initrd_load_file(
    this: *mut LoadFile2FFI,
    file_path: *const DevicePathProtocol,
    boot_policy: bool,
    buff_size: *mut usize,
    buff: *mut c_void,
) -> Status {
    if boot_policy {
        return Status::UNSUPPORTED;
    }

    if buff_size.is_null() {
        return Status::INVALID_PARAMETER;
    }

    let mut buff_size_needed = INITRD_FILES.len();
    let buff_size_parameter = unsafe { *buff_size };

    if buff.is_null() || buff_size_needed > buff_size_parameter {
        unsafe {
            *buff_size = buff_size_needed;
        }
        return Status::BUFFER_TOO_SMALL;
    }

    unsafe {
        copy_nonoverlapping(INITRD_FILES.as_ptr(), buff as *mut u8, buff_size_needed);
    }

    Status::SUCCESS
}
