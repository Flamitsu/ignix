// SPDX-License-Identifier: GPL-3.0-only
#[derive(Clone, Copy)] // Just in case you need to use it inside a loop
#[allow(unused)]
#[repr(C)]
pub struct Tpl(pub usize);
// Those numbers can be found in UEFI spec 2.11 page 150 section "Related definitions"
#[allow(unused)]
impl Tpl{
    pub const TPL_APPLICATION: Self = Self(4);
    pub const TPL_CALLBACK: Self = Self(8);
    pub const TPL_NOTIFY: Self = Self(16);
    pub const TPL_HIGH_LEVEL: Self = Self(31);
}
