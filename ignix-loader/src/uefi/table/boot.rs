use crate::uefi::services::boot::{Event, Image, Memory, Misc};
#[allow(unused)]
#[repr(C)]
pub struct BootServices{
    pub event: Event,
    pub image: Image,
    pub memory: Memory,
    pub misc: Misc
}
