// SPDX-License-Identifier: GPL-3.0-only
/*
 * Booleans in UEFI ABI are represented like this.

 * Normally, C and Rust manage almost the same way booleans
 * but, if anything we know well is that some UEFI implementations
 * are diabolical, so, sometimes they can return a "0x02" instead
 * of "0x01" as true. That causes UB in Rust. (Yeah my jaw also dropped)
*/
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Boolean(pub u8);

impl Boolean {
    pub const TRUE: Self = Self(1);
    pub const FALSE: Self = Self(0);
    pub fn to_bool(self) -> bool {
        self.0 != 0
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self(value as u8)
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.to_bool()
    }
}
