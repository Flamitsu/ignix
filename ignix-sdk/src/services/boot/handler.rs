// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    table::boot::BootServicesWrapper,
    types::{
        DevicePath, DevicePathProtocol, Event, FixedHandleList, Guid, Handle, HandleBuffer,
        IgnixError, IgnixImage, IgnixProtocol, IgnixProtocolNotification, InterfaceType,
        OpenProtocolAttributes, OpenProtocolInformation, OpenProtocolInformationEntry,
        ProtocolGuard, ProtocolsPerHandle, SearchType, Status, Uuid,
    },
};
use core::{
    ffi::c_void,
    marker::PhantomData,
    ptr::{NonNull, null_mut},
};
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
        if status.is_error() {
            Err(status.context("reinstall_protocol_interface"))?
        }
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
            (function.handle_protocol)(handle_ptr, protocol, &mut interface as *mut *mut c_void)
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

    /// Extended version of handle_protocol.
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Protocol is NULL.
    /// EFI_INVALID_PARAMETER Interface is NULL, and Attributes is not TEST_PROTOCOL.
    /// EFI_INVALID_PARAMETER Handle is NULL.
    /// EFI_UNSUPPORTED Handle does not support Protocol.
    /// EFI_INVALID_PARAMETER Attributes is not a legal value.
    /// EFI_INVALID_PARAMETER Attributes is BY_CHILD_CONTROLLER and AgentHandle is NULL.
    /// EFI_INVALID_PARAMETER Attributes is BY_DRIVER and AgentHandle is NULL.
    /// EFI_INVALID_PARAMETER Attribute is BY_DRIVEREXCLUSIVE and AgentHandle is NULL.
    /// EFI_INVALID_PARAMETER Attributes is EXCLUSIVE and AgentHandle is NULL.
    /// EFI_INVALID_PARAMETER Attributes is BY_CHILD_CONTROLLER and ControllerHandle is NULL.
    /// EFI_INVALID_PARAMETER Attributes is BY_DRIVER and ControllerHandle is NULL.
    /// EFI_INVALID_PARAMETER Attributes is BY_DRIVEREXCLUSIVE and ControllerHandle is NULL.
    /// EFI_INVALID_PARAMETER Attributes is BY_CHILD_CONTROLLER and Handle is identical to ControllerHandle.
    /// EFI_ACCESS_DENIED Attributes is BY_DRIVER and there is an item on the open list with an attribute of BY_DRIVEREXCLUSIVE or EXCLUSIVE.
    /// EFI_ACCESS_DENIED Attributes is BY_DRIVEREXCLUSIVE and there is an item on the open list with an attribute of EXCLUSIVE.
    /// EFI_ACCESS_DENIED Attributes is EXCLUSIVE and there is an item on the open list with an attribute of BY_DRIVEREXCLUSIVE or EXCLUSIVE.
    /// EFI_ALREADY_STARTED Attributes is BY_DRIVER and there is an item on the open list with an attribute of BY_DRIVER whose agent handle is the same as AgentHandle.
    /// EFI_ACCESS_DENIED Attributes is BY_DRIVER and there is an item on the open list with an attribute of BY_DRIVER whose agent handle is different than AgentHandle.
    /// EFI_ALREADY_STARTED Attributes is BY_DRIVEREXCLUSIVE and there is an item on the open list with an attribute of BY_DRIVEREXCLUSIVE whose agent handle is the same as AgentHandle.
    /// EFI_ACCESS_DENIED Attributes is BY_DRIVEREXCLUSIVE and there is an item on the open list with an attribute of BY_DRIVEREXCLUSIVE whose agent handle is different than AgentHandle.
    /// EFI_ACCESS_DENIED Attributes is BY_DRIVEREXCLSUIVE or EXCLUSIVE and there are items in the open list with an attribute of BY_DRIVER that could not be removed when EFI_BOOT_SERVICES.DisconnectController() was called for that open item.
    pub fn open_protocol<'a, T>(
        &self,
        handle: Handle,
        protocol: &'a Guid,
        agent_handle: Handle,
        attr: OpenProtocolAttributes,
    ) -> Result<ProtocolGuard<'a, T>, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("open_protocol"))?
        };
        let mut interface: *mut c_void = core::ptr::null_mut();
        let status = unsafe {
            (function.open_protocol)(
                handle,
                protocol,
                &mut interface,
                agent_handle,
                core::ptr::null_mut(),
                attr,
            )
        };

        if status.is_error() {
            Err(status.context("open_protocol"))?
        }

        Ok(ProtocolGuard {
            handle,
            protocol,
            agent_handle,
            interface: interface as *mut T,
            attr,
            _m: PhantomData,
        })
    }
    /// Closes a protocol on a handle open by open_protocol function.
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Handle is NULL.
    /// EFI_INVALID_PARAMETER AgentHandle is NULL.
    ///
    /// This status code is here to complaint to the UEFI spec, but isn't needed for this usecase:
    /// EFI_INVALID_PARAMETER ControllerHandle is not NULL and ControllerHandle is NULL.
    ///
    ///
    /// EFI_INVALID_PARAMETER Protocol is NULL.
    /// EFI_NOT_FOUND Handle does not support the protocol specified by Protocol.
    /// EFI_NOT_FOUND The protocol interface specified by Handle and Protocol is not currently open
    /// by AgentHandle and ControllerHandle
    pub fn close_protocol(
        &self,
        handle: Handle,
        protocol: &Guid,
        agent_handle: Handle,
    ) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("close_protocol"))?
        };
        let status = unsafe {
            (function.close_protocol)(handle, protocol, agent_handle, core::ptr::null_mut())
        };
        if status.is_error() {
            Err(status.context("close_protocol"))?
        }
        Ok(())
    }

    /// Allocates and returns a buffer of EFI_OPEN_PROTOCOL_INFORMATION_ENTRY structures.
    /// The buffer is return in entry_buffer and the number of entries with entry_count
    ///
    /// If the interface specified by Protocol is supported by the handle specified by Handle, then
    /// EntryBuffer is allocated with the boot service AllocatePool() , and EntryCount is set to the
    /// number of entries in EntryBuffer. Each entry of EntryBuffer is filled in with the image
    /// handle, controller handle, and attributes that were passed to OpenProtocol() when the
    /// protocol interface was opened. The field OpenCount shows the number of times that the
    /// protocol interface has been opened by the agent specified by ImageHandle, ControllerHandle,
    /// and Attributes.
    ///
    /// RETURN CODES:
    /// EFI_NOT_FOUND Handle does not support the protocol specified by Protocol.
    /// EFI_OUT_OF_RESOURCES There are not enough resources available to allocate EntryBuffer.
    pub fn open_protocol_information<T>(
        &self,
        protocol: ProtocolGuard<T>,
    ) -> Result<OpenProtocolInformation, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("open_protocol_information"))?
        };

        let mut buffer: *mut OpenProtocolInformationEntry = core::ptr::null_mut();

        let mut entry_buffer: usize = 0;

        let status = unsafe {
            (function.open_protocol_information)(
                protocol.handle,
                protocol.protocol as *const Guid,
                &mut buffer as *mut *mut OpenProtocolInformationEntry,
                &mut entry_buffer,
            )
        };

        if status.is_error() {
            Err(status.context("open_protocol_information"))?
        }

        let ptr = NonNull::new(buffer).unwrap();

        Ok(OpenProtocolInformation {
            count: entry_buffer,
            ptr,
        })
    }
    /* I'm going to left those functions withouth completing for now. Because I don't see them as
     * urgent as the others are.
    pub fn connect_controller(&self) {}
    pub fn disconnect_controller(&self) {}*/

    /// Retrieves a list of protocol interface GUIDs that are installed on a handle in a buffer
    /// allocated from pool (I don't want to do another type struct with RAII but they forced me
    /// with that last one)
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Handle is NULL.
    /// EFI_INVALID_PARAMETER ProtocolBuffer is NULL.
    /// EFI_INVALID_PARAMETER ProtocolBufferCount is NULL.
    /// EFI_OUT_OF_RESOURCES There is not enough pool memory to store the results.
    pub fn protocols_per_handle(&self, handle: Handle) -> Result<ProtocolsPerHandle, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("protocols_per_handle"))?
        };

        let protocol_buffer: *mut *mut *const Guid = core::ptr::null_mut();
        let mut protocol_count: usize = 0;
        let status = unsafe {
            (function.protocols_per_handle)(handle, protocol_buffer, &mut protocol_count)
        };

        if status.is_error() {
            Err(status.context("protocols_per_handle"))?
        }
        let protocol_buff_ptr = NonNull::new(protocol_buffer.cast()).unwrap();
        Ok(ProtocolsPerHandle {
            handle,
            protocol_buffer: protocol_buff_ptr,
            buffer_size: protocol_count,
        })
    }

    /// Returns an array of handles that support the requested protocol in a buffer
    /// allocated from pool
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER NoHandles is NULL
    /// EFI_INVALID_PARAMETER Buffer is NULL
    /// EFI_NOT_FOUND No handles match the search.
    /// EFI_OUT_OF_RESOURCES There is not enough pool memory to store the matching results.
    pub fn locate_handle_buffer(
        &self,
        search_type: SearchType,
        protocol: Option<&Guid>,
        search_key: Option<&c_void>,
    ) -> Result<HandleBuffer, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("locate_handle_buffer"))?
        };
        let protocol_ptr = match protocol {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr as *const Guid,
        };
        let search_key_ptr = match search_key {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr as *const c_void,
        };
        let mut num_handles: usize = 0;
        let mut buffer_handlers: Handle = core::ptr::null_mut();
        let status = unsafe {
            (function.locate_handle_buffer)(
                search_type,
                protocol_ptr,
                search_key_ptr,
                &mut num_handles,
                &mut buffer_handlers as *mut Handle as *mut *mut Handle,
            )
        };

        if status.is_error() {
            Err(status.context("locate_handle_buffer"))?
        }
        let buffer_handlers_ptr = NonNull::new(&mut buffer_handlers).unwrap();
        Ok(HandleBuffer {
            buffer_handlers: buffer_handlers_ptr,
            num_handles,
        })
    }

    /// Returns the first protocol instance that matches the given protocol.
    ///
    /// SAFETY: the compiler can't guarantee the interface that returns isn't null.
    /// Please, be careful using this function and use it with Uuids and structs already registered
    /// in this SDK (that's really the only caution you need to have).
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Interface is NULL. Protocol is NULL.
    /// EFI_NOT_FOUND No protocol instances were found that match Protocol and Registration
    pub fn locate_protocol<T: Uuid>(
        &self,
        register: Option<*const c_void>,
    ) -> Result<&'static T, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("locate_protocol"))?
        };
        let mut interface: *mut c_void = core::ptr::null_mut();
        let register_ptr = match register {
            None => core::ptr::null_mut(),
            Some(ptr) => ptr,
        };
        let status = unsafe {
            (function.locate_protocol)(&T::GUID as *const Guid, register_ptr, &mut interface)
        };
        if status.is_error() {
            Err(status.context("locate_protocol"))?
        }
        unsafe { Ok(&*(interface as *const T)) }
    }

    /*I'm not doing those last two, the devil made them (varargs in C)
    pub fn install_multiple_protocol_interfaces(&self)
    pub fn uninstall_multiple_protocol_interfaces(&self) */
}
