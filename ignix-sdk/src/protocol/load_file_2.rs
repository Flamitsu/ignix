use crate::types::{DevicePathProtocol, Guid, IgnixError, Status, Uuid};
use core::ffi::c_void;
#[repr(C)]
pub struct LoadFile2Protocol {
    pub load_file: unsafe extern "efiapi" fn(
        this: *mut LoadFile2Protocol,
        file_path: DevicePathProtocol,
        boot_policy: bool,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status,
}

pub struct LoadFile2Wrapper {
    protocol: *mut LoadFile2Protocol,
}

impl LoadFile2Wrapper {
    pub unsafe fn new(protocol: *mut LoadFile2Protocol) -> Self {
        assert!(!protocol.is_null(), "LoadFile2Protocol is null");
        Self { protocol }
    }
    fn get_protocol(&self) -> &LoadFile2Protocol {
        unsafe { &*self.protocol }
    }
    pub fn load_file(
        &mut self,
        file_path: DevicePathProtocol,
        buffer: &mut [u8],
    ) -> Result<(), IgnixError> {
        let mut len = buffer.len();
        let status = unsafe {
            (self.get_protocol().load_file)(
                self.protocol,
                file_path,
                false,
                &mut len,
                buffer.as_mut_ptr() as *mut c_void,
            )
        };
        Ok(())
    }
}
impl Uuid for LoadFile2Protocol {
    const GUID: Guid = Guid::new(
        0x4006c0c1,
        0xfcb3,
        0x403e,
        [0x99, 0x6d, 0x4a, 0x6c, 0x87, 0x24, 0xe0, 0x6d],
    );
}
/*
 * This GUID is located in the Linux kernel's 'include/linux/efi.h' file on line 420.
 * This GUID is important because with it you can ignore initrds cmdline routes and just load them
 * into memory. After that, whenever it starts the linux image with EFISTUB it will load it
 * withouth problems (needs this specific GUID)
*/
pub const LINUX_EFI_INITRD_MEDIA_GUID: Guid = Guid::new(
    0x5568e427,
    0x68fc,
    0x4f3d,
    [0xac, 0x74, 0xca, 0x55, 0x52, 0x31, 0xcc, 0x68],
);
