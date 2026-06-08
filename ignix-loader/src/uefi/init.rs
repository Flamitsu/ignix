// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::table::SystemTable;

pub struct InitSystemTable {
    ptr: core::cell::UnsafeCell<*const SystemTable>,
}

unsafe impl Sync for InitSystemTable {}

impl InitSystemTable {
    pub const fn empty() -> Self {
        Self {
            ptr: core::cell::UnsafeCell::new(core::ptr::null()),
        }
    }

    pub fn set(&self, table: *const SystemTable) -> Result<(), ()> {
        unsafe {
            let current = *self.ptr.get();
            if !current.is_null() {
                return Err(());
            }
            *self.ptr.get() = table;
            Ok(())
        }
    }

    pub fn get(&self) -> Option<&'static SystemTable> {
        unsafe {
            let p = *self.ptr.get();
            if p.is_null() {
                None
            } else {
                Some(&*p)
            }
        }
    }
}

pub static SYSTEM_TABLE: InitSystemTable = InitSystemTable::empty();
