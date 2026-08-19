use core::{ffi::c_void, ptr::null_mut, sync::atomic::{AtomicPtr, Ordering}};
use crate::{table::SystemTable, types::{Status, Handle}}; 
pub struct InitGlobalSystemTable {
    ptr: AtomicPtr<SystemTable>,
}
impl InitGlobalSystemTable {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(null_mut()),
        }
    }
    /* Just a big disclaimer, UEFI is single-threaded. I'm doing this so its safe for rust and
     * lets me use this withouth using unsafe keyword.*/
    #[inline(always)]
    pub fn set(&self, item: *const SystemTable) -> Result<(), Status> {
        let item_mut = item as *mut SystemTable;
        self.ptr
            .compare_exchange(
                core::ptr::null_mut(),
                item_mut,
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
            panic!("Cannot get System table pointer. Pointer is null");
        }
        unsafe { &*p }
    }
}

pub struct InitGlobalHandle {
    ptr: AtomicPtr<c_void>,
}

impl InitGlobalHandle {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(null_mut()),
        }
    }

    #[inline(always)]
    pub fn set(&self, handle: Handle) -> Result<(), Status> {
        self.ptr
            .compare_exchange(
                null_mut(),
                handle,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .map(|_| ())
            .map_err(|_| Status::NOT_FOUND)
    }

    #[inline(always)]
    pub fn get(&self) -> Handle {
        let p = self.ptr.load(Ordering::SeqCst);
        if p.is_null() {
            panic!("Cannot get Handle. Pointer is null");
        }
        p as Handle
    }
}

pub static SYSTEM_TABLE: InitGlobalSystemTable = InitGlobalSystemTable::empty();
pub static HANDLE: InitGlobalHandle = InitGlobalHandle::empty();
