// SPDX-License-Identifier: GPL-3.0-only
use crate::{table::SystemTable, types::Status};
use core::sync::atomic::{AtomicPtr, Ordering};

pub struct InitSystemTable {
    ptr: AtomicPtr<SystemTable>,
}

impl InitSystemTable {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(core::ptr::null_mut()),
        }
    }
    /* Just a big disclaimer, UEFI is single-threaded. I'm doing this so its safe for rust and
     * lets me use this withouth using unsafe keyword.*/
    #[inline(always)]
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
            .map_err(|_| Status::ST_POINTER_MISSING)
    }
    #[inline(always)]
    pub fn get(&self) -> &'static SystemTable {
        let p = self.ptr.load(Ordering::SeqCst);
        if p.is_null() {
            panic!("System table pointer is null.");
        }
        unsafe { &*p }
    }
}

pub static SYSTEM_TABLE: InitSystemTable = InitSystemTable::empty();
