// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    init::SYSTEM_TABLE,
    protocol::DevicePathProtocol,
    services::boot::{
        event::close_event,
        handler::{close_protocol, uninstall_protocol_interface},
        memory::free_pool,
    },
    types::{Event, Guid, IgnixImage},
};
use core::{
    ffi::c_void,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};
pub type Handle = *mut c_void;
/* Page 167 UEFI spec 2.11, it's not my fault, it's an enum just with one value.*/
#[repr(C)]
pub enum InterfaceType {
    Native = 0,
}

#[repr(C)]
#[derive(Clone)]
pub enum SearchType {
    AllHandles = 0,
    ByRegisterNotify = 1,
    ByProtocol = 2,
}

#[repr(transparent)]
#[derive(PartialEq, Clone, Copy)]
pub struct OpenProtocolAttributes(pub u32);
impl OpenProtocolAttributes {
    pub const BY_HANDLE_PROTOCOL: Self = Self(0x00000001);
    pub const GET_PROTOCOL: Self = Self(0x00000002);
    pub const TEST_PROTOCOL: Self = Self(0x00000004);
    pub const BY_CHILD_CONTROLER: Self = Self(0x00000008);
    pub const BY_DRIVER: Self = Self(0x00000010);
    pub const EXCLUSIVE: Self = Self(0x00000020);
}

#[repr(C)]
pub struct OpenProtocolInformationEntry {
    pub agent_handle: Handle,
    pub controller_handle: Handle,
    pub attributes: OpenProtocolAttributes,
    pub open_count: u32,
}

pub struct OpenProtocolInformation {
    pub ptr: NonNull<OpenProtocolInformationEntry>,
    pub count: usize,
}

impl Drop for OpenProtocolInformation {
    fn drop(&mut self) {
        free_pool(self.ptr.cast());
    }
}

/// I'm going to be crystal clear with this, IgnixProtocol only must be used with protocols
/// that are only and specifically made for Ignix. If you do it with another one, you're going
/// to remove the protocol from the database of the whole motherboard, and good luck trying to
/// figure it out, because now every single Handle will say they don't know what protocol are you
/// referring to
#[repr(C)]
pub struct IgnixProtocol<'p, 'i> {
    pub image: &'p mut IgnixImage<'i>,
    pub guid: Guid,
    pub interface: Option<*mut c_void>,
}

impl<'p> Drop for IgnixProtocol<'p, '_> {
    fn drop(&mut self) {
        if let Some(image_handle) = self.image.handle {
            uninstall_protocol_interface(image_handle, &self.guid, self.interface);
        }
    }
}

pub struct IgnixProtocolNotification<'a> {
    pub search_key: *mut c_void,
    pub event: Event,
    pub _m: PhantomData<&'a c_void>,
}

impl<'a> Drop for IgnixProtocolNotification<'a> {
    fn drop(&mut self) {
        close_event(self.event);
    }
}

/* Why this struct? see, the locate_handle function wants to allocate memory but i don't.
 * I used an static buffer, should be enough for most cases. However, the whole buffer was return.
 * So to fix that and don't return bullshit (buffers with null pointers at the end of the data) it's
 * better to have an struct that handles that for you*/
pub struct FixedHandleList<const N: usize> {
    pub storage: [*mut c_void; N],
    pub len: usize,
}
impl<const N: usize> FixedHandleList<N> {
    pub fn as_slice(&self) -> &[*mut c_void] {
        &self.storage[..self.len]
    }
}

pub struct DevicePath {
    pub handle: Handle,
    pub device_path: *const DevicePathProtocol,
}

pub struct ProtocolGuard<'a, T> {
    pub handle: Handle,
    pub protocol: &'a Guid,
    pub interface: *mut T,
    pub agent_handle: Handle,
    pub attr: OpenProtocolAttributes,
    pub _m: PhantomData<&'a c_void>,
}

impl<'a, T> Drop for ProtocolGuard<'a, T> {
    fn drop(&mut self) {
        /*This is because the UEFI spec says if the attribute is get_protocol or test_protocol,
         * you don't need to call to close protocol.*/
        if self.attr == OpenProtocolAttributes::GET_PROTOCOL
            || self.attr == OpenProtocolAttributes::TEST_PROTOCOL
        {
            return;
        }
        close_protocol(self.handle, self.protocol, self.agent_handle);
    }
}

impl<'a, T> Deref for ProtocolGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.interface }
    }
}

impl<'a, T> DerefMut for ProtocolGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.interface }
    }
}

pub struct ProtocolsPerHandle {
    pub handle: Handle,
    pub protocol_buffer: NonNull<Guid>,
    pub buffer_size: usize,
}

impl Drop for ProtocolsPerHandle {
    fn drop(&mut self) {
        free_pool(self.protocol_buffer.cast());
    }
}

pub struct HandleBuffer {
    pub num_handles: usize,
    pub buffer_handlers: NonNull<*mut Handle>,
}

impl Drop for HandleBuffer {
    fn drop(&mut self) {
        free_pool(self.buffer_handlers.cast());
    }
}
