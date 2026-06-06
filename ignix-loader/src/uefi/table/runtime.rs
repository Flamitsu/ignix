use crate::uefi::services::runtime::{Misc, Time, Variable, Vmemory};
#[allow(unused)]
#[repr(C)]
// Again, as same as boot.rs this should be in this order
pub struct RuntimeServices{
    pub variable: *mut Variable,
    pub time: *mut Time,
    pub vmemory: *mut Vmemory,
    pub misc: *mut Misc
}
