use crate::types::Guid;

#[repr(C, packed)]
pub struct DevicePathProtocol {
    pub r#type: u8,
    pub subtype: u8,
    pub length: [u8; 2],
}

#[repr(C, packed)]
pub struct VendorDevicePathNode {
    pub hdr: DevicePathProtocol,
    pub guid: Guid,
}
