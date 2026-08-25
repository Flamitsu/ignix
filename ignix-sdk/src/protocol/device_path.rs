use crate::types::{Guid, Uuid};

#[repr(C)]
pub struct DevicePathProtocol {
    pub r#type: u8,
    pub subtype: u8,
    pub length: [u8; 2],
}
impl Uuid for DevicePathProtocol {
    const GUID: Guid = Guid::new(
        0x09576e91,
        0x6d3f,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}
#[repr(C)]
pub struct DevicePathNode<T> {
    pub hdr: DevicePathProtocol,
    pub node: T,
    pub end: DevicePathProtocol,
}

impl<T> DevicePathNode<T> {
    pub fn new(r#type: u8, subtype: u8, node: T) -> Self {
        let node_size = (size_of::<DevicePathProtocol>() + size_of::<T>()) as u16;
        Self {
            hdr: DevicePathProtocol {
                r#type,
                subtype,
                length: node_size.to_le_bytes(),
            },
            node,
            end: DevicePathProtocol {
                r#type: 0x7F,
                subtype: 0xFF,
                length: [4, 0],
            },
        }
    }

    pub fn as_device_path(&self) -> &DevicePathProtocol {
        &self.hdr
    }
}
#[repr(C)]
pub struct VendorDevicePathNode {
    pub guid: Guid,
}
