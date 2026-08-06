// SPDX-License-Identifier: GPL-3.0-only
use crate::types::{Char16, Guid, IgnixError, Status, Time, Uuid};
use core::{
    ffi::c_void,
    ptr::{NonNull, null_mut},
};
#[repr(C)]
pub struct FileProtocol {
    revision: u64,
    pub open: unsafe extern "efiapi" fn(
        this: *mut Self,
        new_handle: *mut *mut FileProtocol,
        file_name: *const u16,
        open_mode: OpenModes,
        attr: FileAttributes,
    ) -> Status,
    pub close: unsafe extern "efiapi" fn(this: *mut Self), // This also returns an status but I wont
    // include it, since you can't call it
    // under normal circustances
    pub delete: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub read: unsafe extern "efiapi" fn(
        this: *mut Self,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status,
    pub write: unsafe extern "efiapi" fn(
        this: *mut Self,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status,
    pub get_position: unsafe extern "efiapi" fn(this: *mut Self, position: *mut u64) -> Status,
    pub set_position: unsafe extern "efiapi" fn(this: *mut Self, position: *mut u64) -> Status,
    pub get_info: unsafe extern "efiapi" fn(
        this: *mut Self,
        information_type: *const Guid,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status,
    pub set_info: unsafe extern "efiapi" fn() -> Status,
    pub flush: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    // Those extended protocols were added in revision 0x00020000
    open_ex: *mut c_void,
    read_ex: *mut c_void,
    write_ex: *mut c_void,
    file_ex: *mut c_void,
}

pub struct FileProtocolWrapper {
    protocol: NonNull<FileProtocol>,
}
impl FileProtocolWrapper {
    /// # Safety
    /// So this function is safe to use because if you don't have this protocol mapped
    /// in your firmware you're screwd anyways so it gives a panic and reduces code smell
    /// (Need to apply this pattern for others) so it's completely secure to use
    pub unsafe fn new(protocol: *mut FileProtocol) -> Self {
        let non_null = NonNull::new(protocol).expect("FileProtocol pointer cannot be null");
        Self { protocol: non_null }
    }
    // Safety:
    // The previous assert in the new function must be ALWAYS PRESENT so this unsafe code doesn't
    // blow everything up.
    #[inline(always)]
    fn get_protocol(&self) -> &FileProtocol {
        unsafe { self.protocol.as_ref() }
    }
    /// Opens a new file relative to the source directory's location.
    ///
    /// RETURN CODES:
    /// EFI_SUCCESS The file was opened.
    /// EFI_NOT_FOUND The specified file could not be found on the device.
    /// EFI_NO_MEDIA The device has no medium.
    /// EFI_MEDIA_CHANGED The device has a different medium in it or the medium is no longer supported.
    /// EFI_DEVICE_ERROR The device reported an error.
    /// EFI_VOLUME_CORRUPTED The file system structures are corrupted.EFI_WRITE_PROTECTED An attempt was made to create a file, or open a file for write when the media is write-protected.
    /// EFI_ACCESS_DENIED The service denied access to the file.
    /// EFI_OUT_OF_RESOURCES Not enough resources were available to open the file.
    /// EFI_VOLUME_FULL The volume is full.
    /// EFI_INVALID_PARAMETER This refers to a regular file, not a directory.
    pub fn open(
        &mut self,
        filename: &[Char16],
        open_mode: OpenModes,
        attr: FileAttributes,
    ) -> Result<FileProtocolWrapper, IgnixError> {
        let mut new_handle: *mut FileProtocol = null_mut();
        let status = unsafe {
            (self.get_protocol().open)(
                self.protocol.as_ptr(),
                &mut new_handle,
                filename.as_ptr(),
                open_mode,
                attr,
            )
        };
        if status.is_error() {
            Err(status.context("FileProtocol.open"))?
        }
        Ok(unsafe { FileProtocolWrapper::new(new_handle) })
    }
    /// Closes and deletes a file.
    ///
    /// RETURN CODES:
    /// EFI_WARN_DELETE_FAILURE The handle was closed, but the file was not deleted.
    pub fn delete(&mut self) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().delete)(self.protocol.as_ptr()) };
        // Since it will close the File alone, don't need the RAII pattern to do it anymore.
        if status.is_error() {
            Err(status.context("FileProtocol.delete"))?
        }
        Ok(())
    }
    /// Reads data from a file.
    ///
    /// If 'Self' is not a directory, the function reads a requested number of bytes from
    /// the file at the file's current position and returns them in the buffer.
    /// If read goes beyond the end of the file, the read length is truncated to
    /// the end of the file.
    /// The file's current position is increased by the number of bytes returned.
    /// If This is a directory, the function reads the directory entry at the file’s current position and returns the entry in Buffer.
    /// If the Buffer is not large enough to hold the current directory entry, then EFI_BUFFER_TOO_SMALL is returned and the current file position is not updated.
    ///
    /// RETURN CODES:
    /// EFI_NO_MEDIA The device has no medium.
    /// EFI_DEVICE_ERROR The device reported an error.
    /// EFI_DEVICE_ERROR An attempt was made to read from a deleted file.
    /// EFI_DEVICE_ERROR On entry, the current file position is beyond the end of the file.
    /// EFI_VOLUME_CORRUPTED The file system structures are corrupted.
    /// EFI_BUFFER_TOO_SMALL The BufferSize is too small to read the current directory entry. BufferSize has been updated with the size needed to complete the request.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, IgnixError> {
        let mut size = buffer.len();
        let status = unsafe {
            (self.get_protocol().read)(
                self.protocol.as_ptr(),
                &mut size,
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_error() {
            Err(status.context("FileProtocol.read"))?
        }
        Ok(size)
    }
    /// Writes data to a file.
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORT Writes to open directory files are not supported.
    /// EFI_NO_MEDIA The device has no medium.
    /// EFI_DEVICE_ERROR The device reported an error.
    /// EFI_DEVICE_ERROR An attempt was made to write to a deleted file.
    /// EFI_VOLUME_CORRUPTED The file system structures are corrupted.
    /// EFI_WRITE_PROTECTED The file or medium is write-protected.
    /// EFI_VOLUME_FULL The volume is full.
    pub fn write(&mut self, buffer: &mut [u8]) -> Result<(), IgnixError> {
        let mut size: usize = buffer.len();
        let status = unsafe {
            (self.get_protocol().write)(
                self.protocol.as_ptr(),
                &mut size,
                buffer.as_mut_ptr().cast(),
            )
        };
        if status.is_error() {
            Err(status.context("FileProtocol.write"))?
        }
        Ok(())
    }
    /// Sets a file's current position.
    /// The SetPosition() function sets the current file position for the handle to the position
    /// supplied. With the exception of seeking to position 0xFFFFFFFFFFFFFFFF, only absolute
    /// positioning is supported, and seeking past the end of the file is allowed (a subsequent
    /// write would grow the file). Seeking to position 0xFFFFFFFFFFFFFFFF causes the current
    /// position to be set to the end of the file.
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORTED The seek request for nonzero is not valid on open directories.
    /// EFI_DEVICE_ERROR An attempt was made to set the position of a deleted file
    pub fn set_position(&mut self, mut position: u64) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().set_position)(self.protocol.as_ptr(), &mut position) };
        if status.is_error() {
            Err(status.context("FileProtocol.set_position"))?
        }
        Ok(())
    }
    /// Returns the current file position for the file handle. For directories, the current file
    /// position has no meaning outside of the file system driver and as such the operation is not
    /// supported. An error is returned if 'Self' is a directory.
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORTED The request is not valid on open directories.
    /// EFI_DEVICE_ERROR An attempt was made to get the position from a deleted file.
    pub fn get_position(&mut self) -> Result<u64, IgnixError> {
        let mut position: u64 = 0;
        let status =
            unsafe { (self.get_protocol().get_position)(self.protocol.as_ptr(), &mut position) };
        Ok(position)
    }
    /// Gets the info from a file
    /// RETURN CODES:
    /// EFI_UNSUPPORTED The InformationType is not known.
    /// EFI_NO_MEDIA The device has no medium.
    /// EFI_DEVICE_ERROR The device reported an error.
    /// EFI_VOLUME_CORRUPTED The file system structures are corrupted.
    /// EFI_BUFFER_TOO_SMALL The BufferSize is too small to read the current directory entry.
    /// BufferSize has been updated with the size needed to complete the request.
    pub fn get_info(&mut self) -> Result<FileInfo, IgnixError> {
        /* This buffer size is fixed to: 80 bytes header + 256 * 2 bytes that a file name can be in
         * FAT32 */
        let mut buffer = [0u8; 592];
        let mut buffer_size = buffer.len();
        let status = unsafe {
            (self.get_protocol().get_info)(
                self.protocol.as_ptr(),
                &FileInfo::GUID,
                &mut buffer_size,
                buffer.as_mut_ptr() as *mut c_void,
            )
        };
        if status.is_error() {
            Err(status.context("get_info"))?
        }
        let raw = unsafe { &*(buffer.as_ptr() as *const FileInfo) };
        Ok(FileInfo {
            size: raw.size,
            file_size: raw.file_size,
            physical_size: raw.physical_size,
            create_time: raw.create_time,
            last_accesed: raw.last_accesed,
            modification_time: raw.modification_time,
            attr: raw.attr,
            file_name: raw.file_name,
        })
    }
    /// Flushes all modified data associated with a file to a device
    ///
    /// RETURN CODES:
    /// EFI_NO_MEDIA The device has no medium.
    /// EFI_DEVICE_ERROR The device reported an error.
    /// EFI_VOLUME_CORRUPTED The file system structures are corrupted.
    /// EFI_WRITE_PROTECTED The file or medium is write-protected.
    /// EFI_ACCESS_DENIED The file was opened read-only.
    /// EFI_VOLUME_FULL The volume is full.
    pub fn flush(&mut self) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().flush)(self.protocol.as_ptr()) };
        if status.is_error() {
            Err(status.context("FileProtocol.flush"))?
        }
        Ok(())
    }
}

impl Drop for FileProtocolWrapper {
    fn drop(&mut self) {
        unsafe { (self.get_protocol().close)(self.protocol.as_ptr()) }
    }
}
#[repr(C)]
pub struct OpenModes(u64);
impl OpenModes {
    pub const READ: Self = Self(0x0000000000000001);
    pub const WRITE: Self = Self(0x0000000000000002);
    pub const CREATE: Self = Self(0x8000000000000000);
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FileAttributes(u64);
impl FileAttributes {
    pub const NONE: Self = Self(0x0000000000000000);
    pub const READ_ONLY: Self = Self(0x0000000000000001);
    pub const HIDDEN: Self = Self(0x0000000000000002);
    pub const FILE_SYSTEM: Self = Self(0x0000000000000004);
    pub const RESERVED: Self = Self(0x0000000000000008);
    pub const DIRECTORY: Self = Self(0x0000000000000010);
    pub const ARCHIVE: Self = Self(0x0000000000000020);
    pub const VALID_ATTR: Self = Self(0x0000000000000037);

    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn has_flag(&self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}
#[repr(C)]
pub struct FileInfo {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: Time,
    pub last_accesed: Time,
    pub modification_time: Time,
    pub attr: FileAttributes,
    pub file_name: [u16; 1],
}

impl Uuid for FileInfo {
    const GUID: Guid = Guid::new(
        0x09576e92,
        0x6d3f,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
