// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    table::boot::BootServicesWrapper,
    types::{
        DevicePath, DevicePathProtocol, Event, FixedHandleList, Guid, Handle, IgnixError,
        IgnixImage, IgnixProtocol, IgnixProtocolNotification, InterfaceType, SearchType, Status,
    },
};
use core::{ffi::c_void, marker::PhantomData};
impl BootServicesWrapper {
    /// Installs a protocol interface on a device handle. If the handle doesn't exist, it's created
    /// and added to the handle list in the system.
    /// Handle: handle is the parameter of the device handle that is going to get the new interface
    /// The same GUID cannot be installed more than once onto the same handle.
    ///
    /// RETURN CODES:
    /// EFI_OUT_OF_RESOURCES Space for a new handle could not be allocated.
    /// EFI_INVALID_PARAMETER HandLe is NULL
    /// EFI_INVALID_PARAMETER ProtocoL is NULL.
    /// EFI_INVALID_PARAMETER InterfaceType is not EFI_NATIVE_INTERFACE.
    /// EFI_INVALID_PARAMETER ProtocoL is already installed on the handle specified by HandLe
    pub fn install_protocol_interface<'p, 'i: 'p>(
        &self,
        image: &'p mut IgnixImage<'i>,
        guid: &Guid,
        interface_type: InterfaceType,
        interface: Option<*mut c_void>,
    ) -> Result<IgnixProtocol<'p, 'i>, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("install_protocol_interface"))?
        };

        let Some(mut handle) = image.handle else {
            Err(Status::HANDLE_DEVICE_IS_NULL.context("install_protocol_interface"))?
        };

        let interface_ptr = match interface {
            Some(ptr) => ptr,
            None => core::ptr::null_mut(),
        };

        let status = unsafe {
            (function.install_protocol_interface)(
                &mut handle,
                guid as *const Guid,
                interface_type,
                interface_ptr,
            )
        };
        if status.is_error() {
            Err(status.context("install_protocol_interface"))?
        }

        Ok(IgnixProtocol {
            image,
            guid: *guid,
            interface: Some(interface_ptr),
        })
    }
    /// Removes a protocol interface from a device handle.
    /// The caller is responsible for ensuring that there are no references to a protocol interface
    /// that has been removed. In some cases, outstanding reference information is not available in
    /// the protocol, so the protocol, once added, cannot be removed. Examples include Console I/O,
    /// Block I/O, Disk I/O, and (in general) handles to device protocols.
    /// Warning: You should not call this function on your own, since it implements the
    /// Drop function, that will execute uninstall_protocol_interface function automatically.
    ///
    /// RETURN CODES:
    /// EFI_NOT_FOUND The interface was not found.
    /// EFI_ACCESS_DENIED The interface was not removed because the interface is still being used by a driver.
    /// EFI_INVALID_PARAMETER HandLe is NULL.
    /// EFI_INVALID_PARAMETER ProtocoL is NULL.
    pub fn uninstall_protocol_interface(
        &self,
        handle: Handle,
        protocol: &Guid,
        interface: Option<*mut c_void>,
    ) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("uninstall_protocol_interface"))?
        };
        let interface_ptr = match interface {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr,
        };
        let status =
            unsafe { (function.uninstall_protocol_interface)(handle, protocol, interface_ptr) };
        if status.is_error() {
            Err(status.context("uninstall_protocol_interface"))?
        }
        Ok(())
    }
    /// Reinstalls a protocol interface on a device handle.
    /// The ReinstallProtocolInterface() function reinstalls a protocol interface on a device handle.
    ///
    /// RETURN CODES:
    /// EFI_NOT_FOUND The OldInterface on the handle was not found.
    /// EFI_ACCESS_DENIED The protocol interface could not be reinstalled, because OldInterface is still being used by a driver that will not release it.
    /// EFI_INVALID_PARAMETER HandLe is NULL.
    /// EFI_INVALID_PARAMETER ProtocoL is NULL.
    pub fn reinstall_protocol_interface(
        &self,
        protocol: &mut IgnixProtocol,
        new_interface: &c_void,
    ) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("reinstall_protocol_interface"))?
        };
        let handle_ptr = match protocol.image.handle {
            Some(ptr) => ptr,
            None => Err(Status::HANDLE_DEVICE_IS_NULL.context("reinstall_protocol_interface"))?,
        };
        let old_interface = match protocol.interface {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr,
        };
        let status = unsafe {
            (function.reinstall_protocol_interface)(
                handle_ptr,
                &protocol.guid,
                old_interface,
                new_interface as *const c_void,
            )
        };
        Ok(())
    }

    /// Creates an event that is signaled whenever a new interface is installed for a specified
    /// protocol.
    /// Once the event is signaled you can call locate_handle() to identify the newly installed,
    /// or reinstalled, handles that support Protocol.
    ///
    /// RETURN CODES:
    /// EFI_OUT_OF_RESOURCES Space for the notification event could not be allocated.
    /// EFI_INVALID_PARAMETER Protocol is NULL.
    /// EFI_INVALID_PARAMETER Event is NULL.
    /// EFI_INVALID_PARAMETER Registration is NULL.
    pub fn register_protocol_notify<'a>(
        &self,
        protocol: &Guid,
        event: Event,
    ) -> Result<IgnixProtocolNotification<'a>, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("register_protocol_notify"))?
        };
        let mut search_key = core::ptr::null_mut();
        let status = unsafe {
            (function.register_protocol_notify)(protocol as *const Guid, event, &mut search_key)
        };
        if status.is_error() {
            Err(status.context("register_protocol_notify"))?
        }
        Ok(IgnixProtocolNotification {
            search_key,
            event,
            _m: PhantomData,
        })
    }

    /// Returns an array of handles that support the specified protocol and the SearchType request.
    /// This uses a fixed array with const generics.
    /// DO NOT try to use it ABOVE of 128. If you put above 16KB the memory (2048) will overflow and
    /// corrupt. Please, be careful while doing this type of stuff, and please try to use 'AllHandles
    /// the minimum necessary, since it's really probably you're going to run into a
    /// EFI_BUFFER_TOO_SMALL error, or not, that depends on the hardware plugging in to your machine
    ///
    /// RETURN CODES:
    /// EFI_NOT_FOUND No handles match the search.
    /// EFI_BUFFER_TOO_SMALL The BufferSize is too small for the result. BufferSize has been updated with the size needed to complete the request.
    /// EFI_INVALID_PARAMETER SearchType is not a member of EFI_LOCATE_SEARCH_TYPE.
    /// EFI_INVALID_PARAMETER SearchType is ByRegisterNotify and SearchKey is NULL.
    /// EFI_INVALID_PARAMETER SearchType is ByProtocol and ProtocoL is NULL.
    /// EFI_INVALID_PARAMETER One or more matches are found and BufferSize is NULL.
    /// EFI_INVALID_PARAMETER BufferSize is large enough for the result and Buffer is NULL.
    pub fn locate_handle<const N: usize>(
        &self,
        search_type: SearchType,
        protocol: Option<&Guid>,
        search_key: Option<&c_void>,
    ) -> Result<FixedHandleList<N>, IgnixError> {
        assert!(N > 128);
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("locate_handle"))?
        };

        let protocol_ptr = match protocol {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr as *const Guid,
        };

        let search_key_ptr = match search_key {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr as *const c_void,
        };

        let mut result = FixedHandleList {
            storage: [core::ptr::null_mut(); N],
            len: 0,
        };

        let mut buffer_size = N * core::mem::size_of::<*mut c_void>();

        let status = unsafe {
            (function.locate_handle)(
                search_type,
                protocol_ptr,
                search_key_ptr,
                &mut buffer_size,
                result.storage.as_mut_ptr(),
            )
        };

        if status.is_error() {
            Err(status.context("locate_handle"))?
        }

        result.len = buffer_size / core::mem::size_of::<*mut c_void>();
        Ok(result)
    }
    
    /// Queries a handle to determine if it supports a specified protocol
    /// Interface will be None if the protocol doesn't have any.
    /// It's recommended to use OpenProtocol function instead of this one that is for compatibility
    ///
    /// RETURN CODES:
    /// EFI_UNSUPPORTED The device does not support the specified protocol.
    /// EFI_INVALID_PARAMETER Handle is NULL.
    /// EFI_INVALID_PARAMETER Protocol is NULL.
    /// EFI_INVALID_PARAMETER Interface is NULL.
    pub fn handle_protocol<'p, 'i: 'p>(
        &self,
        handle: &'p mut IgnixImage<'i>,
        protocol: &Guid,
    ) -> Result<IgnixProtocol<'p, 'i>, IgnixError> {
        let Some(handle_ptr) = handle.handle else {
            Err(Status::HANDLE_DEVICE_IS_NULL.context("handle_protocol"))?
        };

        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("handle_protocol"))?
        };
        // Explicit type declaration is needed so it doesn't break IgnixProtocol interface field
        let mut interface: *mut c_void = core::ptr::null_mut();

        let status = unsafe {
            (function.handle_protocol)(handle_ptr, protocol, interface as *mut *mut c_void)
        };

        if status.is_error() {
            Err(status.context("handle_protocol"))?
        }
        let interface_option = if interface.is_null() {
            None
        } else {
            Some(interface)
        };
        Ok(IgnixProtocol {
            image: handle,
            guid: *protocol,
            interface: interface_option,
        })
    }
    
    /// Locates the handle to a device on the device path provided that supports specified protocol
    /// Device path is on input, a pointer to a pointer to the device path. On output, the device
    /// path pointer is modified to point to the remaining part of the device path–that is, when the
    /// function finds the closest handle, it splits the device path into two parts, stripping off
    /// the front part, and returning the remaining portion.
    ///
    /// RETURN CODES:
    /// EFI_NOT_FOUND No handles matched the search.
    /// EFI_INVALID_PARAMETER Protocol is NULL
    /// EFI_INVALID_PARAMETER DevicePath is NULL.
    /// EFI_INVALID_PARAMETER A handle matched the search and Device is NULL.
    pub fn locate_device_path(
        &self,
        protocol: &Guid,
        mut device_path: DevicePathProtocol,
    ) -> Result<DevicePath, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("locate_device_path"))?
        };
        let mut handle: Handle = core::ptr::null_mut();
        let mut dp_ptr = &device_path as *const DevicePathProtocol;
        let status = unsafe {
            (function.locate_device_path)(
                protocol,
                &mut dp_ptr as *mut *const DevicePathProtocol,
                &mut handle,
            )
        };
        Ok(DevicePath {
            device_path: dp_ptr,
            handle,
        })
    }

    pub fn open_protocol(&self) {}

    pub fn close_protocol(self) {}

    pub fn open_protocol_information(&self) {}

    pub fn connect_controller(&self) {}

    pub fn disconnect_controller(&self) {}

    pub fn protocols_per_handle(&self) {}

    pub fn locate_handle_buffer(&self) {}

    pub fn locate_protocol(&self) {}
}
