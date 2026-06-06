use crate::uefi::services::boot::{Event, Handler, Image, Memory, Misc};
#[allow(unused)]
#[repr(C)]
// This should be in this order or else the alingment will be trash
pub struct BootServices{
    pub event: Event,
    pub memory: Memory,
    pub handler: Handler,
    pub image: Image,
    pub misc: Misc
}
