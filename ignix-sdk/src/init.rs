// SPDX-License-Identifier: GPL-3.0-only
use crate::{table::SystemTable, types::Status};
use core::sync::atomic::{AtomicPtr, Ordering};

pub struct InitSystemTable {
    ptr: AtomicPtr<SystemTable>,
}

impl InitSystemTable {
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    pub fn set(&self, table: *const SystemTable) -> Result<(), Status> {
        let table_mut = table as *mut SystemTable;
        self.ptr
            .compare_exchange(
                core::ptr::null_mut(),
                table_mut,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .map(|_| ())
            .map_err(|_| Status::NOT_FOUND)
    }

    pub fn get(&self) -> Option<&'static SystemTable> {
        let p = self.ptr.load(Ordering::SeqCst);
        if p.is_null() {
            return None;
        }
        Some(unsafe { &*p })
    }
}

pub static SYSTEM_TABLE: InitSystemTable = InitSystemTable::empty();
