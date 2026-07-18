// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    services::boot::memory,
    table::boot::BootServicesWrapper,
    types::{DevicePathProtocol, Handle, IgnixError, IgnixImage, Status},
};
use core::{ffi::c_void, marker::PhantomData};

impl BootServicesWrapper {
    /// Loads an EFI image into memory.
    ///
    /// The image can be loaded in two ways:
    /// - Memory-to-Memory: If source_buffer is not None, the function copies the image
    /// from that buffer. device_path is optional but recommended because of security policies.
    /// - File System: If source_buffer is None, it attempts to load the image from the
    /// provided device_path using EFI_SIMPLE_FILE_SYSTEM_PROTOCOL (or load file protocols).
    ///
    /// RETURN CODES:
    /// EFI_NOT_FOUND Both SourceBuffer and DevicePath are NULL.
    /// EFI_INVALID_PARAMETER One of the parameters has an invalid value.
    /// EFI_INVALID_PARAMETER ImageHandle is NULL.
    /// EFI_INVALID_PARAMETER ParentImageHandle is NULL.
    /// EFI_UNSUPPORTED The image type is not supported.
    /// EFI_OUT_OF_RESOURCES Image was not loaded due to insufficient resources.
    /// EFI_LOAD_ERROR Image was not loaded because the image format was corrupt or not understood.
    /// EFI_DEVICE_ERROR Image was not loaded because the device returned a read error.
    /// EFI_ACCESS_DENIED Image was not loaded because the platform policy prohibits the image from
    /// being loaded. NULL is returned in ImageHandle.
    /// EFI_SECURITY_VIOLATION Image was loaded and an ImageHandle was created with a valid
    /// EFI_LOADED_IMAGE_PROTOCOL. However, the current platform policy specifies that the
    /// image should not be started
    pub fn load_image<'a>(
        &self,
        boot_policy: bool,
        parent_image_handle: Handle,
        device_path: Option<&DevicePathProtocol>,
        source_buffer: Option<&[u8]>,
    ) -> Result<IgnixImage<'a>, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("load_image"))?
        };

        let mut handle: Handle = core::ptr::null_mut();

        let device_path_ptr = match device_path {
            Some(path) => path as *const DevicePathProtocol,
            None => core::ptr::null(),
        };

        let (src_buff_ptr, src_size) = match source_buffer {
            Some(slice) => (slice.as_ptr() as *const c_void, slice.len()),
            None => (core::ptr::null(), 0),
        };

        let status = unsafe {
            (function.load_image)(
                boot_policy,
                parent_image_handle,
                device_path_ptr,
                src_buff_ptr,
                src_size,
                &mut handle,
            )
        };

        if status.is_error() {
            Err(status.context("load_image"))?
        }

        Ok(IgnixImage {
            handle: Some(handle),
            _m: PhantomData,
        })
    }

    /// Transfer control to a loaded image's entry point.
    /// When the started image returns or calls exit function, the control returns here and
    /// exit_data will be populated if image provided any.
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER ImageHandle is either an invalid image handle or the image
    /// has already been initialized with StartImage
    /// Exit code from image Exit code from image.
    /// EFI_SECURITY_VIOLATION The current platform policy specifies that the image should not be
    /// started.
    pub fn start_image<'a>(
        &self,
        mut image: IgnixImage<'a>,
    ) -> Result<(), (IgnixError, IgnixImage<'a>)> {
        let Some(function) = self.get_method() else {
            return Err((Status::BST_POINTER_MISSING.context("start_image"), image));
        };

        let handle: Handle = match image.handle {
            Some(ptr) => ptr,
            None => core::ptr::null_mut(),
        };

        let status =
            unsafe { (function.start_image)(handle, core::ptr::null_mut(), core::ptr::null_mut()) };

        if status.is_error() {
            image.handle = Some(handle);
            return Err((status.context("start_image"), image));
        }
        image.handle = None;
        Ok(())
    }

    /// Unloads an image
    ///
    /// If the image’s unload function returns EFI_SUCCESS, the image is unloaded;
    /// otherwise, the error returned by the image’s unload function is returned to the caller
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORTED The image has been started, and does not support unload.
    /// EFI_INVALID_PARAMETER ImageHandle is not a valid image handle.
    /// Exit code from Unload handler Exit code from the image’s unload function.
    pub fn unload_image(&self, efi_handle: Handle) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("unload_image"))?
        };
        let status = unsafe { (function.unload_image)(efi_handle) };
        if status.is_error() {
            Err(status.context("unload_image"))?
        }
        Ok(())
    }

    /// Ends a loaded EFI image and returns control to boot services.
    /// This function may not be called if the image has already returned from its entry point
    /// ( EFI_IMAGE_ENTRY_POINT ) or if it has loaded any child images that have not exited
    /// (all child images must exit before this image can exit).
    /// Warning: You should not be using this function in the first place.
    /// The firmware already exits the binary whenever its needed (end of the code)
    ///
    /// RETURN CODES:
    /// EFI_SUCCESS The image specified by ImageHandle was unloaded. This condition only
    /// occurs for images that have been loaded with LoadImage() but have not been
    /// started with StartImage().
    /// (Does not return.) Image exit. Control is returned to the StartImage() call that invoked
    /// the image specified by ImageHandle.
    /// EFI_INVALID_PARAMETER The image specified by ImageHandle has been loaded and started
    /// with LoadImage() and StartImage(), but the image is not the currently executing image.
    pub fn exit(&self, efi_handle: Handle, efi_status: Status) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("exit"))?
        };
        let status = unsafe { (function.exit)(efi_handle, efi_status, 0, core::ptr::null_mut()) };
        if status.is_error() {
            Err(status.context("exit"))?
        }
        Ok(())
    }

    /// Terminates all boot services.
    /// The handle argument is the one the UEFI gives to the binary whenever it's executed
    pub fn exit_boot_services(&self, efi_handle: Handle) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("exit_boot_services"))?
        };
        let memory_map = self.get_memory_map()?;
        let status = unsafe { (function.exit_boot_services)(efi_handle, memory_map.key) };
        if status.is_error() {
            Err(status.context("exit_boot_services"))?
        }
        Ok(())
    }
}
