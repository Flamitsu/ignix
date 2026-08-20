use crate::{
    services::boot::memory::allocate_pool,
    types::{
        AllocateType, DevicePathProtocol, Guid, IgnixError, MemoryType, PoolBuffer, Status, Uuid,
    },
};
use core::{
    ffi::c_void,
    ptr::{NonNull, null_mut},
};
#[repr(C)]
/// This protocol is used to retrieve boot files from arbitrary devices (Like PXE boot or boot from
/// USB to the main disk)
pub struct LoadFileProtocolFFI {
    pub load_file: unsafe extern "efiapi" fn(
        this: *mut Self,
        file_path: *const DevicePathProtocol,
        boot_policy: bool,
        buff_size: *mut usize,
        buff: *mut c_void,
    ) -> Status,
}

pub struct LoadFileProtocol {
    pub protocol: NonNull<LoadFileProtocolFFI>,
}

impl LoadFileProtocol {
    #[inline(always)]
    fn get_protocol(&self) -> &LoadFileProtocolFFI {
        unsafe { self.protocol.as_ref() }
    }

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

impl Uuid for LoadFileProtocol {
    const GUID: Guid = Guid::new(
        0x56EC3091,
        0x954C,
        0x11d2,
        [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

/// Used to obtain files from arbitrary devices but are not used as boot options
pub struct LoadFile2ProtocolFFI {
    pub load_file: unsafe extern "efiapi" fn(
        this: *mut Self,
        file_path: *const DevicePathProtocol,
        boot_policy: bool,
        buff_size: *mut usize,
        buff: *mut c_void,
    ) -> Status,
}

pub struct LoadFile2Protocol {
    pub protocol: NonNull<LoadFile2ProtocolFFI>,
}

impl LoadFile2Protocol {}

impl Uuid for LoadFile2Protocol {
    const GUID: Guid = Guid::new(
        0x4006c0c1,
        0xfcb3,
        0x403e,
        [0x99, 0x6d, 0x4a, 0x6c, 0x87, 0x24, 0xe0, 0x6d],
    );
}
